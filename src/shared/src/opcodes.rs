#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    Noop, Hlt, Vsync,
    Mov, Trunc, Ext, Copy,
    Add, Sub, Mul, Div,
    And, Or, Xor, Not, Shr, Shl,
    Jmp, Jnz, Jz,
    PUSH, POP, RET, CALL,
    IMG, BG,
    DBG, DBGSEC
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

            Opcode::Jmp => 0x40,
            Opcode::Jnz => 0x41,
            Opcode::Jz => 0x42,

            Opcode::PUSH => 0x50,
            Opcode::POP => 0x51,
            Opcode::RET => 0x52,
            Opcode::CALL => 0x53,
            
            Opcode::IMG => 0x60,
            Opcode::BG => 0x61,

            Opcode::DBG => 0x70,
            Opcode::DBGSEC => 0x71,
        }
    }
    
    pub fn from_bytecode(code: u8) -> Option<Self> {
        match code {
            0x00 => Some(Opcode::Noop),
            0x01 => Some(Opcode::Hlt),
            0x02 => Some(Opcode::Vsync),
            0x10 => Some(Opcode::Mov),
            0x11 => Some(Opcode::Trunc),
            0x12 => Some(Opcode::Ext),
            0x13 => Some(Opcode::Copy),
            0x20 => Some(Opcode::Add),
            0x21 => Some(Opcode::Sub),
            0x22 => Some(Opcode::Mul),
            0x23 => Some(Opcode::Div),
            0x30 => Some(Opcode::And),
            0x31 => Some(Opcode::Or),
            0x32 => Some(Opcode::Xor),
            0x33 => Some(Opcode::Not),
            0x34 => Some(Opcode::Shl),
            0x35 => Some(Opcode::Shr),
            0x40 => Some(Opcode::Jmp),
            0x41 => Some(Opcode::Jnz),
            0x42 => Some(Opcode::Jz),
            0x50 => Some(Opcode::PUSH),
            0x51 => Some(Opcode::POP),
            0x52 => Some(Opcode::RET),
            0x53 => Some(Opcode::CALL),
            0x60 => Some(Opcode::IMG),
            0x61 => Some(Opcode::BG),
            0x70 => Some(Opcode::DBG),
            0x71 => Some(Opcode::DBGSEC),
            _ => None
        }
    }
}