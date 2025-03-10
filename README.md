# nvim_rs

A neovim configuration written in Rust using [noib3/nvim-oxi](https://github.com/noib3/nvim-oxi).

Created mostly for fun/as a joke, but maybe I'll actually switch to it at some point lol.

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

All of the plugins are still written in Lua anyways.

You know what that means...
