use crate::ffi;

pub fn commands() {
    highlight_ok_yank();
    transparent_background();
}

fn highlight_ok_yank() {
    let opts = nvim_oxi::api::opts::CreateAugroupOpts::builder()
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

fn transparent_background() {

    fn callback(
        _: nvim_oxi::api::types::AutocmdCallbackArgs
    ) -> nvim_oxi::Result<bool> {
        nvim_oxi::api::command("highlight Normal ctermbg=NONE guibg=NONE")?;
        nvim_oxi::api::command("highlight Pmenu ctermbg=NONE guibg=NONE")?;
        nvim_oxi::api::command("highlight SignColumn ctermbg=NONE guibg=NONE")?;
        Ok(false)
    }

    let opts  = nvim_oxi::api::opts::CreateAugroupOpts::builder()
        .build();

    let group = nvim_oxi::api::create_augroup("TransparentBackground", &opts)
        .unwrap();

    let opts = nvim_oxi::api::opts::CreateAutocmdOpts::builder()
        .group(group)
        .callback(callback)
        .build();

    nvim_oxi::api::create_autocmd(["UIEnter"], &opts)
        .unwrap();    
}
