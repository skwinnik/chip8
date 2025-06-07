use crate::{
    chip8::{instruction::Chip8Instruction, Chip8},
    display::Display,
};
use twelve_bit::u12::*;

impl<D> Chip8<D>
where
    D: Display,
{
    pub(super) fn decode(&mut self, code: u16) -> Result<Chip8Instruction, String> {
        if let Ok(inst) = Chip8::<D>::_decode(code) {
            Ok(inst)
        } else {
            Err(self.bad_instruction(code))
        }
    }

    fn _decode(code: u16) -> Result<Chip8Instruction, ()> {
        let x = ((code & 0x0f00) >> 8) as u8;
        let y = ((code & 0x00f0) >> 4) as u8;
        let n = (code & 0x000f) as u8;
        let nn = (code & 0x00ff) as u8;
        let nnn = (code & 0x0fff) as u16;

        match code & 0xf000 {
            0x0000 => match nnn {
                0x0e0 => Ok(Chip8Instruction::ClearScreen()),
                0x0ee => Ok(Chip8Instruction::Return()),
                _ => Err(()),
            },
            0x1000 => Ok(Chip8Instruction::Jump(u12![nnn])),
            0x2000 => Ok(Chip8Instruction::Call(u12![nnn])),
            0x3000 => Ok(Chip8Instruction::SkipIfEqual(x, nn)),
            0x4000 => Ok(Chip8Instruction::SkipIfNotEqual(x, nn)),
            0x5000 => Ok(Chip8Instruction::SkipIfEqualXY(x, y)),
            0x6000 => Ok(Chip8Instruction::SetVX(x, nn)),
            0x7000 => Ok(Chip8Instruction::AddVX(x, nn)),
            0x8000 => match n {
                0x0 => Ok(Chip8Instruction::SetVXToVY(x, y)),
                0x1 => Ok(Chip8Instruction::OrVXVY(x, y)),
                0x2 => Ok(Chip8Instruction::AndVXVY(x, y)),
                0x3 => Ok(Chip8Instruction::XorVXVY(x, y)),
                0x4 => Ok(Chip8Instruction::AddVYRegisterToVX(x, y)),
                _ => Err(()),
            },
            0x9000 => Ok(Chip8Instruction::SkipIfNotEqualXY(x, y)),
            0xA000 => Ok(Chip8Instruction::SetIRegister(nnn)),
            0xD000 => Ok(Chip8Instruction::Draw(x, y, n)),
            _ => Err(()),
        }
    }

    fn bad_instruction(&self, code: u16) -> String {
        format!(
            "Invalid instruction at {:#06x}: {:#06x}",
            u16::from(self.pc - u12![2]),
            code
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::display::test_display::TestDisplay;

    use super::*;
    use rstest::*;

    #[rstest]
    #[case::clear_screen(0x00e0, Chip8Instruction::ClearScreen())]
    #[case::clear_screen(0x1123, Chip8Instruction::Jump(u12![0x123]))]
    #[case::call_return(0x00ee, Chip8Instruction::Return())]
    #[case::call(0x2123, Chip8Instruction::Call(u12![0x123]))]
    #[case::skip_if_equal(0x3123, Chip8Instruction::SkipIfEqual(1, 0x23))]
    #[case::skip_if_not_equal(0x4123, Chip8Instruction::SkipIfNotEqual(1, 0x23))]
    #[case::skip_if_equal_xy(0x5123, Chip8Instruction::SkipIfEqualXY(1, 2))]
    #[case::skip_if_not_equal_xy(0x9123, Chip8Instruction::SkipIfNotEqualXY(1, 2))]
    #[case::set_vx(0x6123, Chip8Instruction::SetVX(1, 0x23))]
    #[case::add_vx(0x7123, Chip8Instruction::AddVX(1, 0x23))]
    #[case::set_vx_to_vy(0x8120, Chip8Instruction::SetVXToVY(1, 2))]
    #[case::or_vx_vy(0x8121, Chip8Instruction::OrVXVY(1, 2))]
    #[case::and_vx_vy(0x8122, Chip8Instruction::AndVXVY(1, 2))]
    #[case::xor_vx_vy(0x8123, Chip8Instruction::XorVXVY(1, 2))]
    #[case::add_vy_to_vx(0x8124, Chip8Instruction::AddVYRegisterToVX(1, 2))]
    #[case::set_i_register(0xa123, Chip8Instruction::SetIRegister(0x123))]
    #[case::draw(0xd123, Chip8Instruction::Draw(1, 2, 3))]
    fn test_decode_success(#[case] input: u16, #[case] expected: Chip8Instruction) {
        let mut chip8 = get_test_chip8();
        assert_eq!(expected, chip8.decode(input).unwrap());
    }

    fn get_test_chip8() -> Chip8<TestDisplay> {
        Chip8::new(TestDisplay::new())
    }
}
