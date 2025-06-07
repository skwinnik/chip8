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
    SkipIfEqualXY(u8, u8),
    /// 0x9XY0
    SkipIfNotEqualXY(u8, u8),
    /// 0x6XNN
    SetVX(u8, u8),
    /// 0x7XNN
    AddVX(u8, u8),
    /// 0x8XY0
    SetVXToVY(u8, u8),
    /// 0x8XY1
    OrVXVY(u8, u8),
    /// 0x8XY2
    AndVXVY(u8, u8),
    /// 0x8XY3
    XorVXVY(u8, u8),
    /// 0x8XY4
    AddVYRegisterToVX(u8, u8),
    /// 0x8XY5
    /// Set VX to VX - VY
    /// If the minuend (the first operand) is larger than the subtrahend (second operand), VF will be set to 1. If the subtrahend is larger, and we “underflow” the result, VF is set to 0
    SubVYFromVX(u8, u8),
    /// 0x8XY6
    /// COSMAC: Set VX to VY & 0x0F
    /// Chip48: Set VX to VX & 0x0F
    ShiftVXRight(u8, u8),
    /// 0x8XY7
    /// Set VX to VY - VX
    /// If the minuend (the first operand) is larger than the subtrahend (second operand), VF will be set to 1. If the subtrahend is larger, and we “underflow” the result, VF is set to 0
    SubVXFromVY(u8, u8),
    /// 0x8XYE
    /// COSMAC: Set VX to VY << 1
    /// Chip48: Set VX to VX >> 1
    ShiftVXLeft(u8, u8),
    /// 0xANNN
    SetIRegister(u16),
    /// 0xDXYN
    Draw(u8, u8, u8),
}

impl Display for Chip8Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chip8Instruction::ClearScreen() => write!(f, "0x00E0 - Clear Screen"),
            Chip8Instruction::Jump(addr) => {
                write!(f, "0x1NNN - Jump to address {}", u16::from(*addr))
            }
            Chip8Instruction::Return() => write!(f, "0x00EE - Return from subroutine"),
            Chip8Instruction::Call(addr) => write!(
                f,
                "0x2NNN - Call subroutine at address {}",
                u16::from(*addr)
            ),
            Chip8Instruction::SkipIfEqual(v, val) => {
                write!(f, "0x3XNN - Skip if v{} == {:02X}", v, val)
            }
            Chip8Instruction::SkipIfNotEqual(v, val) => {
                write!(f, "0x4XNN - Skip if v{} != {:02X}", v, val)
            }
            Chip8Instruction::SkipIfEqualXY(vx, vy) => {
                write!(f, "0x5XY0 - Skip if v{} == v{}", vx, vy)
            }
            Chip8Instruction::SkipIfNotEqualXY(vx, vy) => {
                write!(f, "0x9XY0 - Skip if v{} != v{}", vy, vx)
            }
            Chip8Instruction::SetVX(v, val) => {
                write!(f, "0x6XNN - Set v{} to {:02X}", v, val)
            }
            Chip8Instruction::AddVX(v, val) => {
                write!(f, "0x7XNN - Add {:02X} to v{}", val, v)
            }
            Chip8Instruction::SetVXToVY(vx, vy) => {
                write!(f, "0x8XY0 - Set v{} to v{}", vx, vy)
            }
            Chip8Instruction::OrVXVY(vx, vy) => {
                write!(f, "0x8XY1 - Or v{} with v{}", vx, vy)
            }
            Chip8Instruction::AndVXVY(vx, vy) => {
                write!(f, "0x8XY2 - And v{} with v{}", vx, vy)
            }
            Chip8Instruction::XorVXVY(vx, vy) => {
                write!(f, "0x8XY3 - Xor v{} with v{}", vx, vy)
            }
            Chip8Instruction::AddVYRegisterToVX(vx, vy) => {
                write!(f, "0x8XY4 - Add v{} to v{}", vy, vx)
            }
            Chip8Instruction::SubVYFromVX(vx, vy) => {
                write!(f, "0x8XY5 - Sub v{} from v{}", vy, vx)
            }
            Chip8Instruction::SubVXFromVY(vx, vy) => {
                write!(f, "0x8XY7 - Sub v{} from v{}", vx, vy)
            }
            Chip8Instruction::ShiftVXRight(x, y) => {
                write!(f, "0x8XY6 - Shift v{}, v{} right", x, y)
            }
            Chip8Instruction::ShiftVXLeft(x, y) => write!(f, "0x8XYE - Shift v{}, v{} left", x, y),
            Chip8Instruction::SetIRegister(addr) => write!(f, "0xANNN - Set i to {:03X}", addr),
            Chip8Instruction::Draw(v, x, y) => write!(f, "0xDXYN - Draw v{} at ({}, {})", v, x, y),
        }
    }
}
