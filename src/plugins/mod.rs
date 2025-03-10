use crate::ffi;

use std::path::Path;
use std::process::Command;

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
    let mut lazy_config = nvim_oxi::Dictionary::new();

    lazy_config.insert("defaults", {
        let mut defaults = nvim_oxi::Dictionary::new();
        defaults.insert("lazy", true);
        defaults
    });

    lazy_config.insert("lockfile", nvim_config.join("lazy-lock.json").as_os_str().to_str().unwrap());

    //ffi::lazy_setup(&lazy_config);
}
