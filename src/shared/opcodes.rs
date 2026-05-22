use crate::compiler::ParseError;

#[derive(Copy, Clone)]
pub enum Opcode {
    Noop, Hlt, 
    Mov, Trunc, Ext, Copy
}

impl Opcode {
    pub fn to_bytecode(&self) -> u8 {
        match self {
            Opcode::Noop => 0x00,
            Opcode::Hlt => 0x01,
            Opcode::Mov => 0x10,
            Opcode::Trunc => 0x11,
            Opcode::Ext => 0x12,
            Opcode::Copy => 0x13
        }
    }
    
    pub fn from_bytecode(code: u8) -> Result<Self, ParseError> {
        match code {
            0x00 => Ok(Opcode::Noop),
            0x01 => Ok(Opcode::Hlt),
            0x10 => Ok(Opcode::Mov),
            0x11 => Ok(Opcode::Trunc),
            0x12 => Ok(Opcode::Ext),
            0x13 => Ok(Opcode::Copy),
            _ => Err(ParseError::UnknownOpcode(code.to_string()))
        }
    }
}