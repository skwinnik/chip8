mod decode;
mod execute;
mod fetch;
mod instruction;
mod load;
use std::{thread::sleep, time::Duration};

use crate::display::{minifb::MinifbDisplay, Display};
use twelve_bit::u12::*;

pub struct Chip8<D>
where
    D: Display,
{
    /// Display adapter specific to current environment
    display: D,

    /// Main memory vector
    memory: Vec<u8>,

    // Program counter
    pc: U12,

    v_reg: Vec<u8>,
    i_reg: u16,

    /// Display buffer, draws every cycle
    display_buffer: Vec<bool>,
}

impl<D> Chip8<D>
where
    D: Display,
{
    pub fn new(display: D) -> Self {
        let display_size = display.get_size();

        Chip8 {
            display: display,
            memory: vec![0; Into::<usize>::into(U12::max_value()) + 1],

            pc: u12![0],
            v_reg: vec![0; 16],
            i_reg: 0,

            display_buffer: vec![false; display_size.0 * display_size.1],
        }
    }

    pub fn run(&mut self) {
        while self.display.is_open() {
            let code = self.fetch();
            let instruction = self.decode(code);

            match instruction {
                Ok(instruction) => self.execute(instruction),
                Err(e) => panic!("Error: {}", e),
            }

            self.render_buffer();
            self.sleep();
        }
    }

    fn render_buffer(&mut self) {
        self.display.update(&mut self.display_buffer);
    }

    fn inc_pc(&mut self, x: u16) {
        self.pc = self.pc + u12![x];
    }

    fn sleep(&self) {
        sleep(Duration::from_millis(10));
    }
}

type Chip8Macos = Chip8<MinifbDisplay>;
pub fn get_chip8_macos() -> Chip8Macos {
    Chip8::new(MinifbDisplay::new())
}
