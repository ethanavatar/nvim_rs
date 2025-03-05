use crate::ffi;

use std::path::Path;
use std::process::Command;

pub fn plugins() {
    let lazy_path = Path::new(&ffi::stdpath("data").unwrap())
        .join("lazy")
        .join("lazy.nvim");

    if !lazy_path.exists() {
        Command::new("git")
            .args(["clone", "--filter=blob:none", "https://github.com/folke/lazy.nvim.git", lazy_path.to_str().unwrap()])
            .output().expect("failed to clone lazy");

        Command::new("git")
            .args(["git", "-C", lazy_path.to_str().unwrap(), "checkout", "tags/stable"])
            .output().expect("failed to clone lazy");
    }

    let home_var = if cfg!(windows) { "HOMEPATH" } else { "HOME" };
    let home_str = std::env::var(home_var).unwrap();
    let home_dir = Path::new(&home_str);

    let cache_dir = home_dir.join(".cache").join("nvim");
    if !cache_dir.exists() {
        // Maybe RWX permissions are needed?
        std::fs::create_dir(cache_dir).unwrap();
    }
}
