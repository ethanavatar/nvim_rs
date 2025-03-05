use nvim_oxi::lua;
use nvim_oxi::lua::{ffi::*, macros::cstr};

use std::ffi::CString;
use std::ffi::CStr;

pub fn stdpath(path: &str) -> Option<String> {
    let path_cstr = CString::new(path)
        .expect("CString::new failed");

    let mut result_path: Option<String> = None;
    unsafe {
        lua::with_state(|lstate| {
            // Put `vim.highlight.on_yank` on the stack.
            lua_getglobal(lstate, cstr!("vim"));
            lua_getfield(lstate, -1, cstr!("fn"));
            lua_getfield(lstate, -1, cstr!("stdpath"));

            lua_pushstring(lstate, path_cstr.as_ptr());

            lua_call(lstate, 1, 1);
            let returned = lua_tostring(lstate, -1);
            result_path = CStr::from_ptr(returned)
                .to_str().ok()
                .map(|s| s.to_string());

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

