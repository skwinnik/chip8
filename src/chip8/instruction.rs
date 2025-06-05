use std::fmt::Display;

use twelve_bit::u12::U12;

#[derive(Debug, PartialEq, Eq)]
pub enum Chip8Instruction {
    /// 0x00E0
    ClearScreen(),
    /// 0x1NNN
    Jump(U12),
    /// 0x00EE
    Return(),
    /// 0x2NNN
    Call(U12),
    /// 0x3XNN
    SkipIfEqual(u8, u8),
    /// 0x4XNN
    SkipIfNotEqual(u8, u8),
    /// 0x5XY0
    SkipIfEqualVRegister(u8, u8),
    /// 0x9XY0
    SkipIfNotEqualVRegister(u8, u8),
    /// 0x6XNN
    SetVRegister(u8, u8),
    /// 0x7XNN
    AddVRegister(u8, u8),
    /// 0x8XY0
    SetVRegisterFromVRegister(u8, u8),
    /// 0xANNN
    SetIRegister(u16),
    /// 0xDXYN
    Draw(u8, u8, u8),
}

impl Display for Chip8Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chip8Instruction::ClearScreen() => write!(f, "Clear Screen"),
            Chip8Instruction::Jump(addr) => write!(f, "jmp {}", u16::from(*addr)),
            Chip8Instruction::Return() => write!(f, "ret"),
            Chip8Instruction::Call(addr) => write!(f, "call {}", u16::from(*addr)),
            Chip8Instruction::SkipIfEqual(v, val) => {
                write!(f, "skip if v{} == {:02X}", v, val)
            }
            Chip8Instruction::SkipIfNotEqual(v, val) => {
                write!(f, "skip if v{} != {:02X}", v, val)
            }
            Chip8Instruction::SkipIfEqualVRegister(v, val) => {
                write!(f, "skip if v{} == v{}", v, val)
            }
            Chip8Instruction::SkipIfNotEqualVRegister(v, val) => {
                write!(f, "skip if v{} != v{}", v, val)
            }
            Chip8Instruction::SetVRegister(v, val) => {
                write!(f, "set v{} to {:02X}", v, val)
            }
            Chip8Instruction::AddVRegister(v, val) => {
                write!(f, "add {:02X} to v{}", val, v)
            }
            Chip8Instruction::SetVRegisterFromVRegister(v, val) => {
                write!(f, "set v{} to v{}", v, val)
            }
            Chip8Instruction::SetIRegister(addr) => write!(f, "set i to {:03X}", addr),
            Chip8Instruction::Draw(v, x, y) => write!(f, "draw v{} at ({}, {})", v, x, y),
        }
    }
}
