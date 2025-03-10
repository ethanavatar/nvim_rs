mod ffi;
mod api;
mod types;

mod options;
mod commands;
mod plugins;

use mlua::prelude::*;
use nvim_oxi::mlua;

fn hello(_: &Lua, name: String) -> LuaResult<()> {
    println!("hello, {}!", name);
    Ok(())
}

#[nvim_oxi::plugin]
fn nvim_rs() -> nvim_oxi::Result<()> {
    options::globals();
    options::options();
    commands::commands();
    plugins::plugins();

    Ok(())
}

