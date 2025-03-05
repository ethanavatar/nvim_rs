
pub fn set_option<TOpt>(
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

pub fn set_var<TVar>(
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
