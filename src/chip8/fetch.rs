use crate::{chip8::Chip8, display::Display};

impl<D> Chip8<D>
where
    D: Display,
{
    pub(super) fn fetch(&mut self) -> u16 {
        let i_1 = self.memory[usize::from(self.pc)] as u16;
        self.inc_pc(1);

        let i_2 = self.memory[usize::from(self.pc)] as u16;
        self.inc_pc(1);

        (i_1 << 8) | i_2
    }
}
