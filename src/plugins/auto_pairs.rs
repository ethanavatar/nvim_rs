use nvim_oxi::mlua;
use nvim_oxi::mlua::Table;

use crate::types::TableBuilder;

pub fn spec<'a>() -> Table<'a> {
    TableBuilder::new()
        .set_value("echasnovski/mini.pairs")
        .set("version", "*")
        .set("event", "InsertEnter")
        .set("config", true)
        .build()
}
