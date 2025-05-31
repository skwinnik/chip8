use crate::{
    display::{self, minifb::MinifbDisplay, Display},
    memory::Memory,
};

pub struct Chip8<D>
where
    D: Display,
{
    display: D,
    memory: Memory,

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

            display_buffer: vec![0; display_size.0 * display_size.1],
        }
    }

    pub fn run(&mut self) {
        while self.display.is_open() {
            self.fetch();
            self.decode();
            self.execute();
        }
    }

    fn fetch(&self) {}

    fn decode(&self) {}

    fn execute(&mut self) {
        self.render_buffer();
    }

    fn render_buffer(&mut self) {
        self.display.update(&mut self.display_buffer);
    }
}

type Chip8Macos = Chip8<MinifbDisplay>;
pub fn get_chip8_macos() -> Chip8Macos {
    Chip8::new(MinifbDisplay::new())
}
