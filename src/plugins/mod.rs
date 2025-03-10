use crate::ffi;

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
        let lazy_repo = "https://github.com/folke/lazy.nvim.git";
        Command::new("git")
            .args([
                "clone", "--filter=blob:none", lazy_repo,
                lazy_path.to_str().unwrap()
            ])
            .output()
            .expect("failed to clone lazy");

        Command::new("git")
            .args([
                "git", "-C", lazy_path.to_str().unwrap(),
                "checkout", "tags/stable"
            ])
            .output()
            .expect("failed to clone lazy");
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
    let lazy_config = lua.create_table().unwrap();

    lazy_config.set("lockfile", nvim_config.join("lazy-lock.json").as_os_str().to_str().unwrap());

    lazy_config.set("spec", {
        lua.create_sequence_from([
            {
                let oil = lua.create_sequence_from(["stevearc/oil.nvim"]).unwrap();
                oil.set("lazy", false);
                oil.set("opts", {
                    let opts = lua.create_table().unwrap();
                    opts.set("float", {
                        let float = lua.create_table().unwrap();
                        float.set("padding", 6);
                        float
                    });
                    opts
                });

                oil.set("dependencies", 
                    lua.create_sequence_from([
                        {
                            let icons = lua.create_sequence_from(["echasnovski/mini.icons"]).unwrap();
                            icons.set("version", "*");
                            icons.set("opts", lua.create_table().unwrap());
                            icons
                        }
                    ]).unwrap()
                );

                oil.set(
                    "keys", 
                    lua.create_sequence_from([
                    {
                        let keys = lua.create_sequence_from([
                            "<leader>n",
                            "<CMD>Oil --float<CR>"
                        ]).unwrap();
                        keys.set("desc", "Open parent directory (Oil.nvim)");
                        keys.set("mode", "n");

                        keys
                    }
                    ]).unwrap()
                );

                oil
            }
        ]).unwrap()
    });

    lazy_config.set("defaults", {
        let defaults = lua.create_table().unwrap();
        defaults.set("lazy", true);
        defaults
    });

    lazy_config.set("performance", {
        let performance = lua.create_table().unwrap();

        performance.set("cache", {
            let cache = lua.create_table().unwrap();
            cache.set("enabled", true);
            let lazy_cache = cache_dir
                .join("lazy")
                .join("cache");

            let lazy_cache = lazy_cache
                .as_os_str()
                .to_str()
                .unwrap();

            cache.set("path", lazy_cache);
            cache.set(
                "disable_events",
                lua.create_sequence_from([ "VimEnter", "BufReadPre" ]).unwrap()
            );
            cache
        });

        performance.set("rtp", {
            let rtp = lua.create_table().unwrap();
            rtp.set("reset", true);
            rtp.set(
                "disabled_plugins", 
                lua.create_sequence_from([
                    "netrw",
                    "netrwPlugin",
                    "netrwSettings",
                    "netrwFileHandlers",
                    "gzip",
                    "zip",
                    "zipPlugin",
                    "tar",
                    "tarPlugin",
                    "getscript",
                    "getscriptPlugin",
                    "vimball",
                    "vimballPlugin",
                    "2html_plugin",
                    "logipat",
                    "rrhelper",
                    "spellfile_plugin",
                    "matchit",
                ]).unwrap()
            );
            rtp
        });

        performance
    });

    mlua::lua().globals()
        .get::<_, Function>("require").unwrap()
        .call::<_, Table>("lazy").unwrap()
        .get::<_, Function>("setup").unwrap()
        .call::<_, ()>(lazy_config).unwrap();
}
