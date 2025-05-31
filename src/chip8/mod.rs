use std::{thread::sleep, time::Duration};

use crate::{
    display::{minifb::MinifbDisplay, Display},
    memory::Memory,
    u12::U12,
};

const PROGRAM_START_ADDR: usize = 0x200;

pub struct Chip8<D>
where
    D: Display,
{
    display: D,
    memory: Memory,

    pc: U12,

    display_buffer: Vec<u32>,
}

impl<D> Chip8<D>
where
    D: Display,
{
    pub fn new(display: D) -> Self {
        let display_size = display.get_size();

        Chip8 {
            display: display,
            memory: Memory::new(),

            pc: U12::from_usize(PROGRAM_START_ADDR),

            display_buffer: vec![0; display_size.0 * display_size.1],
        }
    }

    pub fn run(&mut self) {
        while self.display.is_open() {
            let instruction = self.fetch();
            println!("{:#x}", instruction);

            self.decode();
            self.execute();
            self.sleep();
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        for (i, byte) in program.iter().enumerate() {
            self.memory.write(
                U12::from_usize(PROGRAM_START_ADDR) + U12::from_usize(i),
                *byte,
            );
        }

        self.pc = U12::from_usize(PROGRAM_START_ADDR);
    }

    fn fetch(&mut self) -> u16 {
        let i_1 = self.memory.read(self.pc) as u16;
        self.inc_pc(1);

        let i_2 = self.memory.read(self.pc) as u16;
        self.inc_pc(1);

        (i_1 << 8) | i_2
    }

    fn decode(&self) {}

    fn execute(&mut self) {
        self.render_buffer();
    }

    fn render_buffer(&mut self) {
        self.display.update(&mut self.display_buffer);
    }

    fn inc_pc(&mut self, x: usize) {
        self.pc = self.pc.overflowing_add(U12::from_usize(x));

        if self.pc < U12::from_usize(PROGRAM_START_ADDR) {
            self.pc = U12::from_usize(PROGRAM_START_ADDR);
        }
    }

    fn sleep(&self) {
        sleep(Duration::from_millis(100));
    }
}

type Chip8Macos = Chip8<MinifbDisplay>;
pub fn get_chip8_macos() -> Chip8Macos {
    Chip8::new(MinifbDisplay::new())
}
