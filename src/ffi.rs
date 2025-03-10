use nvim_oxi::mlua;
use nvim_oxi::mlua::Table;
use nvim_oxi::mlua::Function;

use mlua::prelude::*;

pub fn lazy_setup(path: &nvim_oxi::Dictionary) {
    todo!()
}

pub fn rtp_prepend(path: &std::path::Path) {
    fn f(path: &std::path::Path) -> mlua::Result<()> {
        let path_str = path.as_os_str().to_str().unwrap();
        let rtp = mlua::lua().globals()
            .get::<_, Table>("vim")?
            .get::<_, Table>("opt")?
            .get::<_, Table>("rtp")?;

        rtp.get_metatable().unwrap()
            .get::<_, Function>("prepend")?
            .call::<_, ()>((rtp, path_str))?;

        Ok(())
    }

    match f(path) {
        Ok(v) => v,
        Err(e) => {
            nvim_oxi::print!("{:?}", e);
        },
    }
}

pub fn stdpath(path_name: &str) -> Option<std::path::PathBuf> {
    fn f(path_name: &str) -> mlua::Result<Option<std::path::PathBuf>> {
        let path = mlua::lua().globals()
            .get::<_, Table>("vim")?
            .get::<_, Table>("fn")?
            .get::<_, Function>("stdpath")?
            .call::<_, Option<String>>(path_name)?;

        let path = path.map(|s| std::path::PathBuf::from(s));
        Ok(path)
    }

    match f(path_name) {
        Ok(v) => v,
        Err(e) => {
            nvim_oxi::print!("{:?}", e);
            None
        },
    }
}


pub fn highlight_on_yank() {
    fn f() -> mlua::Result<()> {
        mlua::lua().globals()
            .get::<_, Table>("vim")?
            .get::<_, Table>("highlight")?
            .get::<_, Function>("on_yank")?
            .call::<(), ()>(())?;

        Ok(())
    }

    match f() {
        Ok(v) => v,
        Err(e) => {
            nvim_oxi::print!("{:?}", e);
        },
    }
}
