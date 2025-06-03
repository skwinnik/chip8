#[macro_use]
extern crate twelve_bit;

use crate::chip8::get_chip8_macos;

mod chip8;
mod display;

fn main() {
    let mut chip8 = get_chip8_macos();
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <rom_path>", args[0]);
        std::process::exit(1);
    }

    chip8.load_rom(&args[1]);
    chip8.run();
}
