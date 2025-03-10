use nvim_oxi::api as nvim;

pub fn set_option<TOpt>(
    opts: &nvim::opts::OptionOpts,
    name: &str, value: TOpt
) -> ()
where
    TOpt: nvim_oxi::conversion::ToObject
{
    let r = nvim::set_option_value(name, value, opts);
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
    let r = nvim::set_var(name, value);
    if let Err(e) = &r {
        nvim_oxi::print!("{:?}", e);
    }
}
