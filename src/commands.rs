use crate::ffi;
use nvim_oxi::api as nvim;

pub fn commands() {
    highlight_ok_yank();
    transparent_background();
}

fn highlight_ok_yank() {
    let opts = nvim::opts::CreateAugroupOpts::builder()
        .clear(true)
        .build();

    let group = nvim::create_augroup("YankHighlight", &opts)
        .expect("failed to create augroup");

    let opts = nvim::opts::CreateAutocmdOpts::builder()
        .group(group)
        .patterns(["*"])
        .callback(|_| {
            ffi::highlight_on_yank();
            false
        })
        .build();

    nvim::create_autocmd(["TextYankPost"], &opts)
        .expect("failed to create autocmd");
}

fn transparent_background() {

    fn callback(
        _: nvim::types::AutocmdCallbackArgs
    ) -> nvim_oxi::Result<bool> {
        nvim::command("highlight Normal ctermbg=NONE guibg=NONE").expect("failed to set NORMAL");
        nvim::command("highlight Pmenu ctermbg=NONE guibg=NONE").expect("failed to set PMENU");
        nvim::command("highlight SignColumn ctermbg=NONE guibg=NONE").expect("failed to set SignColumn");
        Ok(false)
    }

    let opts  = nvim::opts::CreateAugroupOpts::builder()
        .build();

    let group = nvim::create_augroup("TransparentBackground", &opts)
        .expect("failed to create augroup");

    let opts = nvim::opts::CreateAutocmdOpts::builder()
        .group(group)
        .callback(callback)
        .build();

    nvim::create_autocmd(["UIEnter"], &opts)
        .expect("failed to create autocmd");
}
