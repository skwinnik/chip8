#[macro_use]
extern crate twelve_bit;

use crate::chip8::{compat::Compatibility, get_chip8_macos};

mod chip8;
mod display;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    validate_args(&args);

    let compatibility = get_compatibility(&args);
    let rom_path = get_rom_path(&args);

    println!("compatibility: {}", compatibility);
    println!("rom_path: {}", rom_path);

    let mut chip8 = get_chip8_macos(compatibility);
    chip8.load_rom(rom_path);
    chip8.run();
}

fn validate_args(args: &[String]) {
    if args.len() < 2 {
        eprintln!(
            "Usage: {} --compatibility, -c <cosmac|chip48> <rom_path>",
            args[0]
        );
        std::process::exit(1);
    }
}

fn get_compatibility(args: &[String]) -> Compatibility {
    for (i, arg) in args.iter().enumerate() {
        if arg == "--compatibility" || arg == "-c" {
            return match args[i + 1].as_str() {
                "cosmac" => Compatibility::Cosmac,
                "chip48" => Compatibility::Chip48,
                _ => {
                    eprintln!(
                        "Invalid compatibility mode: {}. Available options: cosmac, chip48",
                        args[i + 1]
                    );
                    std::process::exit(1);
                }
            };
        }
    }

    Compatibility::Cosmac
}

fn get_rom_path(args: &[String]) -> &str {
    &args[1]
}
