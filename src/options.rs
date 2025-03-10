use crate::api;

pub fn globals() {
    api::set_var("mapleader",      " ");
    api::set_var("maplocalleader", " ");
}

pub fn options() {
    let default_scope = nvim_oxi::api::opts::OptionOpts::builder().build();

    api::set_option(&default_scope, "shellslash", true);
    api::set_option(&default_scope, "termguicolors", true);

    api::set_option(&default_scope, "number",         true);
    api::set_option(&default_scope, "relativenumber", true);

    api::set_option(&default_scope, "expandtab",  true);
    api::set_option(&default_scope, "tabstop",    4);
    api::set_option(&default_scope, "shiftwidth", 4);

    api::set_option(&default_scope, "colorcolumn", "80");
    api::set_option(&default_scope, "signcolumn",  "yes");

    api::set_option(&default_scope, "ignorecase", true);

    api::set_option(&default_scope, "mouse", "");
}

