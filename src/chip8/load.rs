use crate::{chip8::Chip8, display::Display};
use twelve_bit::u12::*;

const PROGRAM_START_ADDR: u16 = 0x200;

impl<D> Chip8<D>
where
    D: Display,
{
    pub fn load_rom(&mut self, rom_path: &str) {
        let program = std::fs::read(rom_path);
        match program {
            Ok(program) => {
                for (i, byte) in program.iter().enumerate() {
                    let addr = PROGRAM_START_ADDR as usize + i;
                    self.memory[addr] = *byte;
                }

                self.pc = u12![PROGRAM_START_ADDR];
            }
            Err(e) => {
                eprintln!("Failed to read ROM file: {}", e);
            }
        }
    }
}
