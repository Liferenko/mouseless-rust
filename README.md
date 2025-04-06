# Mouseless

A simple Rust application to control your mouse cursor using upper-HJKL keys.

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
- `y` - move cursor left
- `u` - move cursor down
- `i` - move cursor up
- `o` - move cursor right
- `c` - left click
- `v` - right click
- `e` + movement keys - precise 2px movement
- `Esc` - exit program

Hold the `q` key while pressing any of the above keys to control the mouse. Release `q` to type normally.
Hold `e` while moving (with q still held) for precise 2-pixel movements instead of the default 15-pixel movements.

## TODO:
- [x] remap Esc to better key
- [ ] lock keys to prevent unneccesary actions in the applications
- [x] remap left click and right click
- [ ] add vertical scroll
- [ ] check how dragging works
- [ ] add smooth movement (longer key press = faster movement)
