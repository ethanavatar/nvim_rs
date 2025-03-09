use nvim_oxi::lua;
use nvim_oxi::lua::ffi::*;
use nvim_oxi::lua::macros::cstr;

use std::ffi::CString;
use std::ffi::CStr;

use std::ffi::{c_char, c_int};

#[cfg_attr(
    all(target_os = "windows", target_env = "msvc"),
    link(name = "lua51", kind = "raw-dylib")
)]
unsafe extern "C" {
    pub fn luaL_getmetafield (L: *mut nvim_oxi::lua::ffi::lua_State, obj: c_int, e: *const c_char) -> c_int;
}

pub fn lazy_setup(path: &nvim_oxi::Dictionary) {
    todo!()
}

pub fn rtp_prepend(path: &std::path::Path) {
    let path_str = path.as_os_str()
        .to_str()
        .unwrap();

    let path_cstr = CString::new(path_str)
        .expect("CString::new failed");

    unsafe {
        lua::with_state(move |lstate| {
            lua_getglobal(lstate, cstr!("vim"));
            lua_getfield(lstate, -1, cstr!("opt"));
            lua_getfield(lstate, -1, cstr!("rtp"));

            luaL_getmetafield(lstate, -1, cstr!("prepend"));
            lua_getfield(lstate, -3, cstr!("rtp"));
            lua_pushstring(lstate, path_cstr.as_ptr());

            lua_call(lstate, 2, 0);

            lua_pop(lstate, 1); // rtp
            lua_pop(lstate, 1); // opt
            lua_pop(lstate, 1); // vim
        });
    };
}

pub fn stdpath(path_name: &str) -> Option<std::path::PathBuf> {
    let path_cstr = CString::new(path_name)
        .expect("CString::new failed");

    let mut result_path: Option<std::path::PathBuf> = None;
    unsafe {
        lua::with_state(|lstate| {
            lua_getglobal(lstate,    cstr!("vim"));
            lua_getfield(lstate, -1, cstr!("fn"));
            lua_getfield(lstate, -1, cstr!("stdpath"));

            lua_pushstring(lstate, path_cstr.as_ptr());

            lua_call(lstate, 1, 1);
            let returned = lua_tostring(lstate, -1);
            result_path = CStr::from_ptr(returned)
                .to_str().ok()
                .map(|s| std::path::PathBuf::from(s));

            lua_pop(lstate, 1);
            lua_pop(lstate, 1);
        });
    };

    result_path
}


pub fn highlight_on_yank() {
    unsafe {
        lua::with_state(|lstate| {
            // Put `vim.highlight.on_yank` on the stack.
            lua_getglobal(lstate, cstr!("vim"));
            lua_getfield(lstate, -1, cstr!("highlight"));
            lua_getfield(lstate, -1, cstr!("on_yank"));

            // Call `on_yank` and pop it off the stack.
            lua_call(lstate, 0, 0);

            // Pop `vim` off the stack.
            lua_pop(lstate, 1);

            // Pop `highlight` off the stack.
            lua_pop(lstate, 1);
        })
    };
}

