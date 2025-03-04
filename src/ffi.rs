use std::error::Error as StdError;

use nvim_oxi::lua;
use nvim_oxi::lua::{ffi::*, macros::cstr};

pub fn highlight_on_yank() {
    unsafe {
        lua::with_state(move |lstate| {
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

