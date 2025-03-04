mod ffi;

use nvim_oxi::api as api;

fn set_option<TOpt>(
    opts: &api::opts::OptionOpts,
    name: &str, value: TOpt
) -> Result<(), api::Error>
where
    TOpt: nvim_oxi::conversion::ToObject
{
    api::set_option_value(name, value, opts)
}

#[nvim_oxi::plugin]
fn nvim_rs() -> nvim_oxi::Result<nvim_oxi::Dictionary> {

    let default_scope = api::opts::OptionOpts::builder().build();
    let global_scope = api::opts::OptionOpts::builder().scope(api::opts::OptionScope::Global).build();

    {
        set_option(&default_scope, "number", true);
        set_option(&default_scope, "relativenumber", true);

        set_option(&default_scope, "expandtab", true);
        set_option(&default_scope, "tabstop", 4);
        set_option(&default_scope, "shiftwidth", 4);

        set_option(&default_scope, "colorcolumn", 80);
        set_option(&default_scope, "signcolumn", "yes");

        set_option(&default_scope, "ignorecase", true);
    }

    {
        set_option(&global_scope, "mapleader", " ");
        set_option(&global_scope, "maplocalleader", " ");
    }

    {
        let opts  = api::opts::CreateAugroupOpts::builder().clear(true).build();
        let group = api::create_augroup("YankHighlight", &opts).unwrap();

        let opts = api::opts::CreateAutocmdOpts::builder()
            .group(group)
            .patterns(["*"])
            .callback(|_| {
                ffi::highlight_on_yank();
                false
            })
            .build();

        api::create_autocmd(["TextYankPost"], &opts);
    }

    {
        let opts  = api::opts::CreateAugroupOpts::builder().build();
        let group = api::create_augroup("TransparentBackground", &opts).unwrap();

        let opts = api::opts::CreateAutocmdOpts::builder()
            .group(group)
            .callback(|_| {
                api::command("highlight Normal ctermbg=NONE guibg=NONE");
                api::command("highlight Pmenu ctermbg=NONE guibg=NONE");
                api::command("highlight SignColumn ctermbg=NONE guibg=NONE");
                false
            })
            .build();

        api::create_autocmd(["UIEnter"], &opts);
    }

    Ok(nvim_oxi::Dictionary::new())
}

