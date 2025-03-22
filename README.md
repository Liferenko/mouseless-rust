# Mouseless

A simple Rust application to control your mouse cursor using vim-like HJKL keys.

## Prerequisites

- Rust and Cargo
- libxdo-dev (on Ubuntu/Debian: `sudo apt-get install libxdo-dev`)

## Installation

```bash
git clone https://github.com/yourusername/mouseless.git
cd mouseless
cargo build --release
```

## Usage

Run the program:
```bash
cargo run
```

Controls (hold `q` to activate):
- `u` - move cursor left
- `i` - move cursor down
- `o` - move cursor up
- `p` - move cursor right
- `c` - left click
- `v` - right click
- `e` + movement keys - precise 1px movement
- `Esc` - exit program

Hold the `q` key while pressing any of the above keys to control the mouse. Release `q` to type normally.
Hold `e` while moving (with q still held) for precise 2-pixel movements instead of the default 10-pixel movements.
