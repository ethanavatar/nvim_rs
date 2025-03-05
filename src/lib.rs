mod ffi;

fn set_option<TOpt>(
    opts: &nvim_oxi::api::opts::OptionOpts,
    name: &str, value: TOpt
) -> ()
where
    TOpt: nvim_oxi::conversion::ToObject
{
    let r = nvim_oxi::api::set_option_value(name, value, opts);
    if let Err(e) = &r {
        nvim_oxi::print!("{:?}", e);
    }
}

fn set_var<TVar>(
    name: &str, value: TVar
) -> ()
where
    TVar: nvim_oxi::conversion::ToObject
{
    let r = nvim_oxi::api::set_var(name, value);
    if let Err(e) = &r {
        nvim_oxi::print!("{:?}", e);
    }
}

fn transparent_background(
    _: nvim_oxi::api::types::AutocmdCallbackArgs
) -> nvim_oxi::Result<bool> {
    nvim_oxi::api::command("highlight Normal ctermbg=NONE guibg=NONE")?;
    nvim_oxi::api::command("highlight Pmenu ctermbg=NONE guibg=NONE")?;
    nvim_oxi::api::command("highlight SignColumn ctermbg=NONE guibg=NONE")?;
    Ok(false)
}

#[nvim_oxi::plugin]
fn nvim_rs() -> nvim_oxi::Result<nvim_oxi::Dictionary> {

    let default_scope = nvim_oxi::api::opts::OptionOpts::builder().build();

    {
        set_var("mapleader",      " ");
        set_var("maplocalleader", " ");
    }

    {
        set_option(&default_scope, "number",         true);
        set_option(&default_scope, "relativenumber", true);

        set_option(&default_scope, "expandtab",  true);
        set_option(&default_scope, "tabstop",    4);
        set_option(&default_scope, "shiftwidth", 4);

        set_option(&default_scope, "colorcolumn", "80");
        set_option(&default_scope, "signcolumn",  "yes");

        set_option(&default_scope, "ignorecase", true);
    }

    {
        let opts  = nvim_oxi::api::opts::CreateAugroupOpts::builder()
            .clear(true)
            .build();

        let group = nvim_oxi::api::create_augroup("YankHighlight", &opts)
            .unwrap();

        let opts = nvim_oxi::api::opts::CreateAutocmdOpts::builder()
            .group(group)
            .patterns(["*"])
            .callback(|_| {
                ffi::highlight_on_yank();
                false
            })
            .build();

        nvim_oxi::api::create_autocmd(["TextYankPost"], &opts)
            .unwrap();
    }

    {
        let opts  = nvim_oxi::api::opts::CreateAugroupOpts::builder()
            .build();

        let group = nvim_oxi::api::create_augroup("TransparentBackground", &opts)
            .unwrap();

        let opts = nvim_oxi::api::opts::CreateAutocmdOpts::builder()
            .group(group)
            .callback(transparent_background)
            .build();

        nvim_oxi::api::create_autocmd(["UIEnter"], &opts)
            .unwrap();
    }

    Ok(nvim_oxi::Dictionary::new())
}

