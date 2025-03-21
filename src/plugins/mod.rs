mod file_navigation;
mod auto_pairs;
mod treesitter;

use crate::ffi;
use crate::types::TableBuilder;

use std::path::Path;
use std::process::Command;

use nvim_oxi::mlua;
use nvim_oxi::mlua::Table;
use nvim_oxi::mlua::Function;

use mlua::prelude::*;

pub fn plugins() {
    let nvim_data = ffi::stdpath("data").unwrap();
    let lazy_path = nvim_data
        .join("lazy")
        .join("lazy.nvim");

    if !lazy_path.exists() {
        install_lazy(&lazy_path);
    }

    let home_var = if cfg!(windows) { "HOMEPATH" } else { "HOME" };
    let home_str = std::env::var(home_var).unwrap();
    let home_dir = Path::new(&home_str);

    let cache_dir = home_dir
        .join(".cache")
        .join("nvim");

    if !cache_dir.exists() {
        // Maybe RWX (o755) permissions are needed?
        std::fs::create_dir(&cache_dir).unwrap();
    }

    ffi::rtp_prepend(&cache_dir);
    ffi::rtp_prepend(&lazy_path);

    let nvim_config = ffi::stdpath("config").unwrap();

    let lua = nvim_oxi::mlua::lua();

    let plugins = [
        file_navigation::spec(),
        auto_pairs::spec(),
        treesitter::spec(),
    ];

    let lazy_cache = cache_dir
        .join("lazy")
        .join("cache");

    let lazy_cache = lazy_cache
        .as_os_str()
        .to_str()
        .unwrap();

    let disabled_plugins = [
        "netrw",         "netrwPlugin",
        "netrwSettings", "netrwFileHandlers",
        "gzip",
        "zip",       "zipPlugin",
        "tar",       "tarPlugin",
        "getscript", "getscriptPlugin",
        "vimball",   "vimballPlugin",
        "2html_plugin",
        "logipat",
        "rrhelper",
        "spellfile_plugin",
        "matchit",
    ];

    let lazy_config = TableBuilder::new()
        .set("lockfile", nvim_config.join("lazy-lock.json").as_os_str().to_str().unwrap())
        .set("spec", lua.create_sequence_from(plugins).unwrap())
        .set("defaults", TableBuilder::new()
            .set("lazy", true)
            .build()
        )
        .set("performance", TableBuilder::new()
            .set("cache", TableBuilder::new()
                .set("enabled", true)
                .set("path", lazy_cache)
                .set(
                    "disable_events",
                    lua.create_sequence_from([ "VimEnter", "BufReadPre" ]).unwrap()
                )
                .build()
            )
            .set("rtp", TableBuilder::new()
                .set("reset", true)
                .set("disabled", lua.create_sequence_from(disabled_plugins).unwrap())
                .build()
            )
            .build()
        )
        .build();

    mlua::lua().globals()
        .get::<_, Function>("require").unwrap()
        .call::<_, Table>("lazy").unwrap()
        .get::<_, Function>("setup").unwrap()
        .call::<_, ()>(lazy_config).unwrap();
}

fn install_lazy(lazy_path: &std::path::PathBuf) {
    Command::new("git")
        .arg("clone")
        .arg("--filter=blob:none")
        .arg("https://github.com/folke/lazy.nvim.git")
        .arg(lazy_path)
        .output()
        .expect("failed to clone lazy");

    Command::new("git")
        .arg("-C")
        .arg(lazy_path)
        .arg("checkout")
        .arg("tags/stable")
        .output()
        .expect("failed to clone lazy");
}
