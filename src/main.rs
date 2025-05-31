use crate::chip8::get_chip8_macos;

mod chip8;
mod display;
mod memory;
mod u12;

fn main() {
    let mut chip8 = get_chip8_macos();
    chip8.run();
}
