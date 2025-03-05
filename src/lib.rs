mod ffi;
mod api;

mod options;
mod commands;
mod plugins;

#[nvim_oxi::plugin]
fn nvim_rs() -> nvim_oxi::Result<nvim_oxi::Dictionary> {
    options::globals();
    options::options();
    commands::commands();
    plugins::plugins();

    Ok(nvim_oxi::Dictionary::new())
}

