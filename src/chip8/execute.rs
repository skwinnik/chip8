use crate::{
    chip8::{
        instruction::{self, Chip8Instruction},
        Chip8,
    },
    display::Display,
};

impl<D> Chip8<D>
where
    D: Display,
{
    pub(super) fn execute(&mut self, instruction: Chip8Instruction) {
        println!("{}", instruction);
        match instruction {
            Chip8Instruction::ClearScreen() => {
                let display_size = self.display.get_size();
                self.display_buffer = vec![false; display_size.0 * display_size.1];
            }
            Chip8Instruction::Jump(nnn) => self.pc = nnn,
            Chip8Instruction::SetVRegister(x, nn) => self.v_reg[x as usize] = nn,
            Chip8Instruction::AddVRegister(x, nn) => {
                self.v_reg[x as usize] = self.v_reg[x as usize].saturating_add(nn)
            }
            Chip8Instruction::SetIRegister(nnn) => self.i_reg = nnn,
            Chip8Instruction::Draw(x_reg, y_reg, n) => {
                let display_size = self.display.get_size();
                let mut x = (self.v_reg[x_reg as usize] as usize) & (display_size.0 - 1);
                let mut y = (self.v_reg[y_reg as usize] as usize) & (display_size.1 - 1);
                self.v_reg[0xF] = 0;

                let display_buffer = &mut self.display_buffer;
                'rows: for i in 0..n {
                    let b = self.memory[self.i_reg as usize + i as usize];
                    'cols: for j in 0..8 {
                        let px = b & (0x1 << (7 - j));
                        let idx = y * display_size.0 + x;
                        if Self::set_display_pixel(display_buffer, idx, px != 0) {
                            self.v_reg[0xf] = 1;
                        }
                        // self.redraw[idx].store(true, Ordering::Release);

                        x += 1;
                        if x >= display_size.0 {
                            break 'cols;
                        }
                    }

                    y += 1;
                    if y >= display_size.1 {
                        break 'rows;
                    }
                    x = (self.v_reg[x_reg as usize] as usize) & (display_size.0 - 1);
                }
            }
        }
    }

    #[inline]
    fn set_display_pixel(display_buffer: &mut Vec<bool>, idx: usize, px: bool) -> bool {
        let px0 = display_buffer[idx];
        display_buffer[idx] = px0 ^ px;
        px0 && (px ^ px0)
    }
}
