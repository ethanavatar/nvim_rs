use nvim_oxi::mlua;
use nvim_oxi::mlua::Table;
use nvim_oxi::mlua::Function;

use crate::types::TableBuilder;

pub fn spec<'a>() -> Table<'a> {
    let lua = mlua::lua();
    TableBuilder::new()
        .set_value("nvim-treesitter/nvim-treesitter")
        .set("build", ":TSUpdate")
        .set("event", "BufReadPre")
        .set("config", lua.create_function::<(), (), _>(|L, _| {
            let install = lua.globals()
                .get::<_, Function>("require").unwrap()
                .call::<_, Table>("nvim-treesitter.install").unwrap();

            install.set("prefer_git", true);
            install.set("compilers", [ "zig" ]);

            let configs = lua.globals()
                .get::<_, Function>("require").unwrap()
                .call::<_, Table>("nvim-treesitter.configs").unwrap();

            configs.get::<_, Function>("setup").unwrap()
                .call::<_, ()>(TableBuilder::new()
                    .set("sync_install", false)
                    .set("auto_install", false)
                    .set("highlight", TableBuilder::new()
                        .set("enable", true)
                        .build()
                    )
                    .build()
                ).unwrap();

            Ok(())
        }).unwrap())
        .build()
}

