use crate::{
    chip8::{instruction::Chip8Instruction, Chip8},
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
            Chip8Instruction::Draw(vx, vy, n) => {
                let display_size = self.display.get_size();
                let mut y = (self.v_reg[vy as usize] as usize) & (display_size.1 - 1);
                self.v_reg[0xf] = 0;

                'rows: for i in 0..n {
                    let mut x = (self.v_reg[vx as usize] as usize) & (display_size.0 - 1);

                    let sprite_byte = self.memory[self.i_reg as usize + i as usize];
                    'cols: for j in 0..8 {
                        let sprite_bit = sprite_byte & (0x1 << (7 - j));
                        let display_px_inx = y * display_size.0 + x;
                        if Self::set_display_pixel(
                            &mut self.display_buffer,
                            display_px_inx,
                            sprite_bit != 0,
                        ) {
                            self.v_reg[0xf] = 1;
                        }

                        x += 1;
                        if x >= display_size.0 {
                            break 'cols;
                        }
                    }

                    y += 1;
                    if y >= display_size.1 {
                        break 'rows;
                    }
                }
            }
        }
    }

    #[inline]
    fn set_display_pixel(display_buffer: &mut Vec<bool>, px_idx: usize, px_val: bool) -> bool {
        let px_old = display_buffer[px_idx];
        display_buffer[px_idx] = px_old ^ px_val;
        px_old && !display_buffer[px_idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display::test_display::TestDisplay;
    use rstest::*;
    use twelve_bit::u12::*;

    #[rstest]
    fn test_clear_screen() {
        let mut chip8 = get_test_chip8();
        chip8.display_buffer = vec![true; 64 * 32];

        chip8.execute(Chip8Instruction::ClearScreen());

        assert_eq!(chip8.display_buffer, vec![false; 64 * 32]);
    }

    #[rstest]
    #[case::jump(0x123, 0x123)]
    fn test_jump(#[case] nnn: u16, #[case] expected: u16) {
        let mut chip8 = get_test_chip8();
        chip8.execute(Chip8Instruction::Jump(u12![nnn]));
        assert_eq!(chip8.pc, u12![expected]);
    }

    #[rstest]
    #[case::set_v_register(0x1, 0x12, 0x12)]
    fn test_set_v_register(#[case] x: u8, #[case] nn: u8, #[case] expected: u8) {
        let mut chip8 = get_test_chip8();
        chip8.execute(Chip8Instruction::SetVRegister(x, nn));
        assert_eq!(chip8.v_reg[x as usize], expected);
    }

    #[rstest]
    /// Simple addition
    #[case::add_v_register(0x0, 0x12, 0x12, 0x24)]
    /// Addition with saturation
    #[case::add_v_register(0x1, 0xF0, 0x20, 0xFF)]
    fn test_add_v_register(
        #[case] x: u8,
        #[case] initial_value: u8,
        #[case] nn: u8,
        #[case] expected: u8,
    ) {
        let mut chip8 = get_test_chip8();
        chip8.execute(Chip8Instruction::SetVRegister(x, initial_value));
        chip8.execute(Chip8Instruction::AddVRegister(x, nn));
        assert_eq!(chip8.v_reg[x as usize], expected);
    }

    #[rstest]
    #[case::set_i_register(0x123, 0x123)]
    fn test_set_i_register(#[case] nnn: u16, #[case] expected: u16) {
        let mut chip8 = get_test_chip8();
        chip8.execute(Chip8Instruction::SetIRegister(nnn));
        assert_eq!(expected, chip8.i_reg);
    }

    #[rstest]
    // true should be returned if the pixel is being turned off
    #[case::set_display_pixel(&mut vec![true], 0, true, false, true)]
    #[case::set_display_pixel(&mut vec![false], 0, true, true, false)]
    #[case::set_display_pixel(&mut vec![false], 0, false, false, false)]
    #[case::set_display_pixel(&mut vec![true], 0, false, true, false)]
    fn test_set_display_pixel(
        #[case] display_buffer: &mut Vec<bool>,
        #[case] px_idx: usize,
        #[case] px_val: bool,
        #[case] expected_px_value: bool,
        #[case] expected_return_value: bool,
    ) {
        assert_eq!(
            expected_return_value,
            Chip8::<TestDisplay>::set_display_pixel(display_buffer, px_idx, px_val)
        );
        assert_eq!(expected_px_value, display_buffer[px_idx]);
    }

    fn get_test_chip8() -> Chip8<TestDisplay> {
        Chip8::new(TestDisplay::new())
    }
}
