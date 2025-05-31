use std::fmt::Display;

use twelve_bit::u12::U12;

#[derive(Debug, PartialEq, Eq)]
pub enum Chip8Instruction {
    ClearScreen(),
    Jump(U12),
    SetVRegister(u8, u8),
    AddVRegister(u8, u8),
    SetIRegister(u16),
    Draw(u8, u8, u8),
}

impl Display for Chip8Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chip8Instruction::ClearScreen() => write!(f, "Clear Screen"),
            Chip8Instruction::Jump(addr) => write!(f, "jmp {}", u16::from(*addr)),
            Chip8Instruction::SetVRegister(v, val) => {
                write!(f, "set v{} to {:02X}", v, val)
            }
            Chip8Instruction::AddVRegister(v, val) => {
                write!(f, "add {:02X} to v{}", val, v)
            }
            Chip8Instruction::SetIRegister(addr) => write!(f, "set i to {:03X}", addr),
            Chip8Instruction::Draw(v, x, y) => write!(f, "draw v{} at ({}, {})", v, x, y),
        }
    }
}
