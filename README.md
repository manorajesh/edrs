# edrs
![Build Status](https://github.com/manorajesh/edrs/actions/workflows/MacOS.yml/badge.svg)
![Build Status](https://github.com/manorajesh/edrs/actions/workflows/Linux.yml/badge.svg)
![Build Status](https://github.com/manorajesh/edrs/actions/workflows/Windows.yml/badge.svg)

Fast, Cross-Platform Terminal Text Editor built with Rust

## Installation
```
cargo install edrs
```
or
```
git clone https://github.com/manorajesh/edrs.git
cd edrs
cargo run --release
```

## Usage
```
A simple text editor in Rust!

Usage: edrs.exe [OPTIONS] [FILE]

Arguments:
  [FILE]  Path to file

Options:
  -s, --syntax         Enable syntax highlighting
  -t, --theme <THEME>  Highlighting theme [default: base16-eighties.dark]
  -h, --help           Print help (see more with '--help')
  -V, --version        Print version
```

For the `-t` option, see [these defaults](https://docs.rs/syntect/latest/syntect/highlighting/struct.ThemeSet.html#method.load_defaults) provided by the [syntect](https://github.com/trishume/syntect)