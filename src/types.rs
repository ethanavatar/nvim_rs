use nvim_oxi::mlua;
use nvim_oxi::mlua::Table;

use mlua::prelude::*;

pub struct TableBuilder<'a> {
    table: Table<'a>,
    counter: usize,
}

impl<'a> TableBuilder<'a> {
    pub fn new() -> Self {
        let table = nvim_oxi::mlua::lua()
            .create_table()
            .unwrap();

        TableBuilder {
            table,
            counter: 1
        }
    }

    pub fn set(mut self, k: impl IntoLua<'a>, v: impl IntoLua<'a>) -> TableBuilder<'a> {
        self.table.set(k, v);
        self
    }

    pub fn set_value(mut self, v: impl IntoLua<'a>) -> TableBuilder<'a> {
        self.table.set(
            { let tmp = self.counter; self.counter += 1; tmp },
            v
        );
        self
    }

    pub fn build(self) -> Table<'a> {
        self.table
    }
}
