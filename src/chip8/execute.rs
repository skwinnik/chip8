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
            Chip8Instruction::Return() => {
                let popped = self.stack.pop();
                if let Some(popped) = popped {
                    self.pc = popped;
                } else {
                    panic!("Stack is empty");
                }
            }
            Chip8Instruction::Call(nnn) => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            Chip8Instruction::SkipIfEqual(x, val) => {
                if self.v_reg[x as usize] == val {
                    self.inc_pc(2);
                }
            }
            Chip8Instruction::SkipIfNotEqual(x, val) => {
                if self.v_reg[x as usize] != val {
                    self.inc_pc(2);
                }
            }
            Chip8Instruction::SkipIfEqualXY(x, y) => {
                if self.v_reg[x as usize] == self.v_reg[y as usize] {
                    self.inc_pc(2);
                }
            }
            Chip8Instruction::SkipIfNotEqualXY(x, y) => {
                if self.v_reg[x as usize] != self.v_reg[y as usize] {
                    self.inc_pc(2);
                }
            }
            Chip8Instruction::SetVX(x, nn) => self.v_reg[x as usize] = nn,
            Chip8Instruction::AddVX(x, nn) => {
                let (result, _) = self.v_reg[x as usize].overflowing_add(nn);
                self.v_reg[x as usize] = result;
            }
            Chip8Instruction::SetVXToVY(x, y) => {
                self.v_reg[x as usize] = self.v_reg[y as usize];
            }
            Chip8Instruction::OrVXVY(x, y) => {
                self.v_reg[x as usize] = self.v_reg[x as usize] | self.v_reg[y as usize];
            }
            Chip8Instruction::AndVXVY(x, y) => {
                self.v_reg[x as usize] = self.v_reg[x as usize] & self.v_reg[y as usize];
            }
            Chip8Instruction::XorVXVY(x, y) => {
                self.v_reg[x as usize] = self.v_reg[x as usize] ^ self.v_reg[y as usize];
            }
            Chip8Instruction::AddVYRegisterToVX(x, y) => {
                let (result, overflow) =
                    self.v_reg[x as usize].overflowing_add(self.v_reg[y as usize]);
                self.v_reg[x as usize] = result;
                self.v_reg[0xf] = overflow as u8;
            }
            Chip8Instruction::SubVYFromVX(x, y) => {
                self.v_reg[0xf] = 1;
                let (result, overflow) =
                    self.v_reg[x as usize].overflowing_sub(self.v_reg[y as usize]);
                self.v_reg[x as usize] = result;
                self.v_reg[0xf] = !overflow as u8;
            }
            Chip8Instruction::SubVXFromVY(x, y) => {
                self.v_reg[0xf] = 1;
                let (result, overflow) =
                    self.v_reg[y as usize].overflowing_sub(self.v_reg[x as usize]);
                self.v_reg[x as usize] = result;
                self.v_reg[0xf] = !overflow as u8;
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
    fn test_return() {
        let mut chip8 = get_test_chip8();
        chip8.stack.push(u12![0x123]);
        chip8.execute(Chip8Instruction::Return());
        assert_eq!(chip8.pc, u12![0x123]);
    }

    #[rstest]
    fn test_call() {
        let mut chip8 = get_test_chip8();
        chip8.execute(Chip8Instruction::Call(u12![0x123]));
        assert_eq!(chip8.pc, u12![0x123]);
        assert_eq!(chip8.stack, vec![u12![0x0]]);
    }

    #[rstest]
    #[case::skip_if_equal_true(Chip8Instruction::SkipIfEqual(0, 0x12), 0, 0x12, 0x0, 0x2)]
    #[case::skip_if_equal_false(Chip8Instruction::SkipIfEqual(0, 0x12), 0, 0x13, 0x0, 0x0)]
    fn test_skip_if_equal(
        #[case] instruction: Chip8Instruction,
        #[case] vx: u8,
        #[case] val: u8,
        #[case] start_pc: u16,
        #[case] expected_pc: u16,
    ) {
        let mut chip8 = get_test_chip8();
        chip8.pc = u12![start_pc];
        chip8.v_reg[vx as usize] = val;
        chip8.execute(instruction);
        assert_eq!(u12![expected_pc], chip8.pc);
    }

    #[rstest]
    #[case::skip_if_not_equal_true(Chip8Instruction::SkipIfNotEqual(0, 0x12), 0, 0x13, 0x0, 0x2)]
    #[case::skip_if_not_equal_false(Chip8Instruction::SkipIfNotEqual(0, 0x12), 0, 0x12, 0x0, 0x0)]
    fn test_skip_if_not_equal(
        #[case] instruction: Chip8Instruction,
        #[case] vx: u8,
        #[case] val: u8,
        #[case] start_pc: u16,
        #[case] expected_pc: u16,
    ) {
        let mut chip8 = get_test_chip8();
        chip8.pc = u12![start_pc];
        chip8.v_reg[vx as usize] = val;
        chip8.execute(instruction);
        assert_eq!(u12![expected_pc], chip8.pc);
    }

    #[rstest]
    #[case::skip_if_equal_v_register_true(
        Chip8Instruction::SkipIfEqualXY(0, 1),
        0,
        0x12,
        1,
        0x12,
        0x0,
        0x2
    )]
    #[case::skip_if_equal_v_register_false(
        Chip8Instruction::SkipIfEqualXY(0, 1),
        0,
        0x13,
        1,
        0x12,
        0x0,
        0x0
    )]
    fn test_skip_if_equal_v_register(
        #[case] instruction: Chip8Instruction,
        #[case] vx: u8,
        #[case] vx_val: u8,
        #[case] vy: u8,
        #[case] vy_val: u8,
        #[case] start_pc: u16,
        #[case] expected_pc: u16,
    ) {
        let mut chip8 = get_test_chip8();
        chip8.pc = u12![start_pc];
        chip8.v_reg[vx as usize] = vx_val;
        chip8.v_reg[vy as usize] = vy_val;
        chip8.execute(instruction);
        assert_eq!(u12![expected_pc], chip8.pc);
    }

    #[rstest]
    #[case::skip_if_not_equal_v_register_true(
        Chip8Instruction::SkipIfNotEqualXY(0, 1),
        0,
        0x13,
        1,
        0x12,
        0x0,
        0x2
    )]
    #[case::skip_if_not_equal_v_register_false(
        Chip8Instruction::SkipIfNotEqualXY(0, 1),
        0,
        0x12,
        1,
        0x12,
        0x0,
        0x0
    )]
    fn test_skip_if_not_equal_v_register(
        #[case] instruction: Chip8Instruction,
        #[case] vx: u8,
        #[case] vx_val: u8,
        #[case] vy: u8,
        #[case] vy_val: u8,
        #[case] start_pc: u16,
        #[case] expected_pc: u16,
    ) {
        let mut chip8 = get_test_chip8();
        chip8.pc = u12![start_pc];
        chip8.v_reg[vx as usize] = vx_val;
        chip8.v_reg[vy as usize] = vy_val;
        chip8.execute(instruction);
        assert_eq!(u12![expected_pc], chip8.pc);
    }

    #[rstest]
    #[case::set_v_register(0x1, 0x12, 0x12)]
    fn test_set_v_register(#[case] x: u8, #[case] nn: u8, #[case] expected: u8) {
        let mut chip8 = get_test_chip8();
        chip8.execute(Chip8Instruction::SetVX(x, nn));
        assert_eq!(chip8.v_reg[x as usize], expected);
    }

    #[rstest]
    /// Simple addition
    #[case::add_v_register(0x0, 0x12, 0x12, 0x24)]
    /// Addition with overflow
    #[case::add_v_register_overflow(0x1, 0xF0, 0x20, 0x10)]
    fn test_add_v_register(
        #[case] x: u8,
        #[case] initial_value: u8,
        #[case] nn: u8,
        #[case] expected: u8,
    ) {
        let mut chip8 = get_test_chip8();
        chip8.execute(Chip8Instruction::SetVX(x, initial_value));
        chip8.execute(Chip8Instruction::AddVX(x, nn));
        assert_eq!(chip8.v_reg[x as usize], expected);
    }

    #[rstest]
    #[case::set_v_register_from_v_register(
        0x1,
        0x5,
        0x2,
        0x12,
        Chip8Instruction::SetVXToVY(0x1, 0x2),
        0x12
    )]
    fn test_set_v_register_from_v_register(
        #[case] x: u8,
        #[case] x_val: u8,
        #[case] y: u8,
        #[case] y_val: u8,
        #[case] instruction: Chip8Instruction,
        #[case] expected: u8,
    ) {
        let mut chip8 = get_test_chip8();
        chip8.execute(Chip8Instruction::SetVX(x, x_val));
        chip8.execute(Chip8Instruction::SetVX(y, y_val));
        chip8.execute(instruction);
        assert_eq!(chip8.v_reg[x as usize], expected);
    }

    #[rstest]
    #[case::or_v_register(0x1, 0x5, 0x2, 0x7, 0x7)]
    fn test_or_v_register(
        #[case] x: u8,
        #[case] x_val: u8,
        #[case] y: u8,
        #[case] y_val: u8,
        #[case] expected: u8,
    ) {
        let mut chip8 = get_test_chip8();
        chip8.execute(Chip8Instruction::SetVX(x, x_val));
        chip8.execute(Chip8Instruction::SetVX(y, y_val));
        chip8.execute(Chip8Instruction::OrVXVY(x, y));
        assert_eq!(chip8.v_reg[x as usize], expected);
    }

    #[rstest]
    #[case::and_v_register(0x1, 0x5, 0x2, 0x0, 0x0)]
    fn test_and_v_register(
        #[case] x: u8,
        #[case] x_val: u8,
        #[case] y: u8,
        #[case] y_val: u8,
        #[case] expected: u8,
    ) {
        let mut chip8 = get_test_chip8();
        chip8.execute(Chip8Instruction::SetVX(x, x_val));
        chip8.execute(Chip8Instruction::SetVX(y, y_val));
        chip8.execute(Chip8Instruction::AndVXVY(x, y));
        assert_eq!(chip8.v_reg[x as usize], expected);
    }

    #[rstest]
    #[case::xor_v_register(0x1, 0x5, 0x2, 0x7, 0x2)]
    fn test_xor_v_register(
        #[case] x: u8,
        #[case] x_val: u8,
        #[case] y: u8,
        #[case] y_val: u8,
        #[case] expected: u8,
    ) {
        let mut chip8 = get_test_chip8();
        chip8.execute(Chip8Instruction::SetVX(x, x_val));
        chip8.execute(Chip8Instruction::SetVX(y, y_val));
        chip8.execute(Chip8Instruction::XorVXVY(x, y));
        assert_eq!(chip8.v_reg[x as usize], expected);
    }

    #[rstest]
    #[case::add_v_register_to_v_register(0x1, 0x5, 0x2, 0x7, 0xC, 0x0)]
    #[case::add_v_register_to_v_register_overflow(0x1, 0xFE, 0x2, 0x03, 0x01, 0x1)]
    fn test_add_v_register_to_v_register(
        #[case] x: u8,
        #[case] x_val: u8,
        #[case] y: u8,
        #[case] y_val: u8,
        #[case] expected: u8,
        #[case] vf: u8,
    ) {
        let mut chip8 = get_test_chip8();
        chip8.execute(Chip8Instruction::SetVX(x, x_val));
        chip8.execute(Chip8Instruction::SetVX(y, y_val));
        chip8.execute(Chip8Instruction::AddVYRegisterToVX(x, y));
        assert_eq!(chip8.v_reg[x as usize], expected);
        assert_eq!(chip8.v_reg[0xf], vf);
    }

    #[rstest]
    #[case::sub_y_from_x(0x1, 0x5, 0x2, 0x3, 0x2, 0x1)]
    #[case::sub_y_from_x_underflow(0x1, 0x5, 0x2, 0x8, 0xFD, 0x0)]
    fn test_sub_y_from_x(
        #[case] x: u8,
        #[case] x_val: u8,
        #[case] y: u8,
        #[case] y_val: u8,
        #[case] expected: u8,
        #[case] vf: u8,
    ) {
        let mut chip8 = get_test_chip8();
        chip8.execute(Chip8Instruction::SetVX(x, x_val));
        chip8.execute(Chip8Instruction::SetVX(y, y_val));
        chip8.execute(Chip8Instruction::SubVYFromVX(x, y));
        assert_eq!(chip8.v_reg[x as usize], expected);
        assert_eq!(chip8.v_reg[0xf], vf);
    }

    #[rstest]
    #[case::sub_x_from_y(0x1, 0x3, 0x2, 0x5, 0x2, 0x1)]
    #[case::sub_x_from_y_underflow(0x1, 0x3, 0x2, 0x1, 0xFE, 0x0)]
    fn test_sub_x_from_y(
        #[case] x: u8,
        #[case] x_val: u8,
        #[case] y: u8,
        #[case] y_val: u8,
        #[case] expected: u8,
        #[case] vf: u8,
    ) {
        let mut chip8 = get_test_chip8();
        chip8.execute(Chip8Instruction::SetVX(x, x_val));
        chip8.execute(Chip8Instruction::SetVX(y, y_val));
        chip8.execute(Chip8Instruction::SubVXFromVY(x, y));
        assert_eq!(chip8.v_reg[x as usize], expected);
        assert_eq!(chip8.v_reg[0xf], vf);
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

    #[rstest]
    fn test_draw_instruction() {
        let mut chip8 = get_test_chip8();

        // Set up sprite data in memory at address 0x300
        let sprite_data = [
            0b11110000, // ####....
            0b10010000, // #..#....
            0b10010000, // #..#....
            0b11110000, // ####....
        ];
        let sprite_address = 0x300;

        // Load sprite data into memory
        for (i, &byte) in sprite_data.iter().enumerate() {
            chip8.memory[sprite_address + i] = byte;
        }

        // Set I register to point to sprite data
        chip8.execute(Chip8Instruction::SetIRegister(sprite_address as u16));

        // Set position registers V0=5, V1=10 (x=5, y=10)
        chip8.execute(Chip8Instruction::SetVX(0, 5));
        chip8.execute(Chip8Instruction::SetVX(1, 10));

        // Execute draw instruction: draw 4 bytes from I register at position (V0, V1)
        chip8.execute(Chip8Instruction::Draw(0, 1, 4));

        // Verify sprite was drawn correctly
        let display_size = chip8.display.get_size();

        // Check first row of sprite (####....)
        assert_eq!(chip8.display_buffer[10 * display_size.0 + 5], true); // pixel (5,10)
        assert_eq!(chip8.display_buffer[10 * display_size.0 + 6], true); // pixel (6,10)
        assert_eq!(chip8.display_buffer[10 * display_size.0 + 7], true); // pixel (7,10)
        assert_eq!(chip8.display_buffer[10 * display_size.0 + 8], true); // pixel (8,10)
        assert_eq!(chip8.display_buffer[10 * display_size.0 + 9], false); // pixel (9,10)

        // Check second row of sprite (#..#....)
        assert_eq!(chip8.display_buffer[11 * display_size.0 + 5], true); // pixel (5,11)
        assert_eq!(chip8.display_buffer[11 * display_size.0 + 6], false); // pixel (6,11)
        assert_eq!(chip8.display_buffer[11 * display_size.0 + 7], false); // pixel (7,11)
        assert_eq!(chip8.display_buffer[11 * display_size.0 + 8], true); // pixel (8,11)

        // Check that VF (collision flag) is 0 (no collision on clear screen)
        assert_eq!(chip8.v_reg[0xf], 0);
    }

    #[rstest]
    fn test_draw_instruction_with_collision() {
        let mut chip8 = get_test_chip8();

        // Set up sprite data in memory
        let sprite_data = [0b11110000]; // ####....
        let sprite_address = 0x400;
        chip8.memory[sprite_address] = sprite_data[0];

        // Pre-fill some pixels at the draw location to test collision
        let display_size = chip8.display.get_size();
        chip8.display_buffer[5 * display_size.0 + 3] = true; // pixel (3,5)
        chip8.display_buffer[5 * display_size.0 + 4] = true; // pixel (4,5)

        // Set I register and position
        chip8.execute(Chip8Instruction::SetIRegister(sprite_address as u16));
        chip8.execute(Chip8Instruction::SetVX(0, 3)); // x=3
        chip8.execute(Chip8Instruction::SetVX(1, 5)); // y=5

        // Execute draw instruction
        chip8.execute(Chip8Instruction::Draw(0, 1, 1));

        // Check collision detection - VF should be 1 because some pixels were turned off
        assert_eq!(chip8.v_reg[0xf], 1);

        // Check XOR behavior - overlapping pixels should be turned off
        assert_eq!(chip8.display_buffer[5 * display_size.0 + 3], false); // was on, now off (collision)
        assert_eq!(chip8.display_buffer[5 * display_size.0 + 4], false); // was on, now off (collision)
        assert_eq!(chip8.display_buffer[5 * display_size.0 + 5], true); // was off, now on
        assert_eq!(chip8.display_buffer[5 * display_size.0 + 6], true); // was off, now on
    }

    #[rstest]
    fn test_draw_instruction_different_i_register() {
        let mut chip8 = get_test_chip8();

        // Set up different sprite data at different memory locations
        chip8.memory[0x200] = 0b10101010; // #.#.#.#.
        chip8.memory[0x300] = 0b01010101; // .#.#.#.#

        // Test drawing from first location
        chip8.execute(Chip8Instruction::SetIRegister(0x200));
        chip8.execute(Chip8Instruction::SetVX(0, 0)); // x=0
        chip8.execute(Chip8Instruction::SetVX(1, 0)); // y=0
        chip8.execute(Chip8Instruction::Draw(0, 1, 1));

        let display_size = chip8.display.get_size();

        // Check pattern from 0x200 (10101010)
        assert_eq!(chip8.display_buffer[0 * display_size.0 + 0], true); // bit 7
        assert_eq!(chip8.display_buffer[0 * display_size.0 + 1], false); // bit 6
        assert_eq!(chip8.display_buffer[0 * display_size.0 + 2], true); // bit 5
        assert_eq!(chip8.display_buffer[0 * display_size.0 + 3], false); // bit 4

        // Clear screen and test drawing from second location
        chip8.execute(Chip8Instruction::ClearScreen());
        chip8.execute(Chip8Instruction::SetIRegister(0x300));
        chip8.execute(Chip8Instruction::Draw(0, 1, 1));

        // Check pattern from 0x300 (01010101)
        assert_eq!(chip8.display_buffer[0 * display_size.0 + 0], false); // bit 7
        assert_eq!(chip8.display_buffer[0 * display_size.0 + 1], true); // bit 6
        assert_eq!(chip8.display_buffer[0 * display_size.0 + 2], false); // bit 5
        assert_eq!(chip8.display_buffer[0 * display_size.0 + 3], true); // bit 4
    }
}
