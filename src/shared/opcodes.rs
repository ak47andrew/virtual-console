use crate::compiler::ParseError;

#[derive(Copy, Clone)]
pub enum Opcode {
    Noop, Hlt, Vsync,
    Mov, Trunc, Ext, Copy,
    Add, Sub, Mul, Div,
    And, Or, Xor, Not, Shr, Shl
}

impl Opcode {
    pub fn to_bytecode(&self) -> u8 {
        match self {
            Opcode::Noop => 0x00,
            Opcode::Hlt => 0x01,
            Opcode::Vsync => 0x02,

            Opcode::Mov => 0x10,
            Opcode::Trunc => 0x11,
            Opcode::Ext => 0x12,
            Opcode::Copy => 0x13,

            Opcode::Add => 0x20,
            Opcode::Sub => 0x21,
            Opcode::Mul => 0x22,
            Opcode::Div => 0x23,

            Opcode::And => 0x30,
            Opcode::Or => 0x31,
            Opcode::Xor => 0x32,
            Opcode::Not => 0x33,
            Opcode::Shl => 0x34,
            Opcode::Shr => 0x35,
        }
    }
    
    pub fn from_bytecode(code: u8) -> Result<Self, ParseError> {
        match code {
            0x00 => Ok(Opcode::Noop),
            0x01 => Ok(Opcode::Hlt),
            0x02 => Ok(Opcode::Vsync),
            0x10 => Ok(Opcode::Mov),
            0x11 => Ok(Opcode::Trunc),
            0x12 => Ok(Opcode::Ext),
            0x13 => Ok(Opcode::Copy),
            0x20 => Ok(Opcode::Add),
            0x21 => Ok(Opcode::Sub),
            0x22 => Ok(Opcode::Mul),
            0x23 => Ok(Opcode::Div),
            0x30 => Ok(Opcode::And),
            0x31 => Ok(Opcode::Or),
            0x32 => Ok(Opcode::Xor),
            0x33 => Ok(Opcode::Not),
            0x34 => Ok(Opcode::Shl),
            0x35 => Ok(Opcode::Shr),
            _ => Err(ParseError::UnknownOpcode(code.to_string()))
        }
    }
}