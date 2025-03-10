# nvim_rs

A neovim configuration written in Rust using [noib3/nvim-oxi](https://github.com/noib3/nvim-oxi).

Created mostly for fun/as a joke, but maybe I'll actually switch to it at some point lol.

Current features:
- 4-space indents
- Relative line numbers
- Highlight-on-Yank
- Transparent background (For terminals that support it)

## TODO-list

- [x] Initializing [folke/lazy](https://github.com/folke/lazy.nvim)
    - [x] Install the only plugin that matters, [stevearc/oil.nvim](https://github.com/stevearc/oil.nvim)
    - [x] Disable default neovim plugins
    - Install a color theme

## Installation

- Clone this repo into `~/.config/nvim_rs`
- Build with `cargo`
- Copy the output `nvim_rs.dll` to `~/.config/nvim_rs/nvim_rs.dll`
- Set the env-var `NVIM_APPNAME=nvim_rs`

## Is it even good?

Of course. It's blazingly fast.

![BLAZINGLY FAST](./images/blazing.webp)
