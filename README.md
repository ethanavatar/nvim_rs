# nvim_rs

Contents:

- [TODO-list](#TODO-list)
- [Installation](#Installation)
- [Is it even good?](#Is-it-even-good)
- [Resources](#Resources)

---

A neovim configuration written in Rust using [noib3/nvim-oxi](https://github.com/noib3/nvim-oxi) and [mlua-rs/mlua](https://github.com/mlua-rs/mlua).

Started as a joke, but turns out it's actually pretty nice, and now I'm actually using it as my daily setup.

No screenshots because it's, visually, completely stock right now.

Current features:
- 4-space indents
- Relative line numbers
- Highlight-on-Yank
- Transparent background (For terminals that support it)
- [stevearc/oil.nvim](https://github.com/stevearc/oil.nvim)
    - [echasnovski/mini.icons](https://github.com/echasnovski/mini.nvim/blob/main/readmes/mini-icons.md)
- [echasnovski/mini.pairs](https://github.com/echasnovski/mini.nvim/blob/main/readmes/mini-pairs.md) for auto-pairs

## TODO-list

- [x] Setup [folke/lazy](https://github.com/folke/lazy.nvim) for plugin management
    - [x] Install the only important plugin, [stevearc/oil.nvim](https://github.com/stevearc/oil.nvim)
    - [x] Disable default neovim plugins
    - [X] Disable mouse
    - [X] something for auto-pairs
    - [ ] Install a color theme
    - [ ] [nvim-lualine/lualine.nvim](https://github.com/nvim-lualine/lualine.nvim) or something like it

## Installation

- Clone this repo into `~/.config/nvim_rs`
- Build with `cargo`
- Copy the output `nvim_rs.dll` to `~/.config/nvim_rs/nvim_rs.dll`
- Set the env-var `NVIM_APPNAME=nvim_rs`

## Is it even good?

Of course. It's blazingly fast.

![BLAZINGLY FAST](./images/blazing.webp)

Its actually not even faster than a pure-Lua version would be. It might even be slower because of the interop layer.

All of the plugins are still written in Lua anyways...

You know what that means...

## Resources

This project was inspired by:

- [rewhile/CatNvim](https://github.com/rewhile/CatNvim) - A neovim config written in C
- [turboladen/init.rs](https://github.com/turboladen/init.rs) - A neovim config written in Rust

If you want to try it out yourself, also check out the [examples directory in nvim-oxi](https://github.com/noib3/nvim-oxi/tree/main/examples)
