#[derive(Eq, Hash, PartialEq)]
pub enum Registers {
    A, X, Y, Z,
    G1, G2, G3, G4, G5,
}

impl Registers {
    pub fn to_bytecode(&self) -> u8 {
        match self {
            Registers::A => { 0x1 }
            Registers::X => { 0x2 }
            Registers::Y => { 0x3 }
            Registers::Z => { 0x4 }

            Registers::G1 => { 0xA1 }
            Registers::G2 => { 0xA2 }
            Registers::G3 => { 0xA3 }
            Registers::G4 => { 0xA4 }
            Registers::G5 => { 0xA5 }
        }
    }
    
    pub fn from_bytecode(code: u8) -> Registers {
        match code { 
            0x1 => Registers::A,
            0x2 => Registers::X,
            0x3 => Registers::Y,
            0x4 => Registers::Z,
            0xA1 => Registers::G1,
            0xA2 => Registers::G2,
            0xA3 => Registers::G3,
            0xA4 => Registers::G4,
            0xA5 => Registers::G5,
            _ => panic!("Uh... Corrupted program ig"),
        }
    }

    pub fn all() -> Vec<Registers> {
        vec![Registers::A, Registers::X, Registers::Y, Registers::Z,
             Registers::G1, Registers::G2, Registers::G3, Registers::G4, Registers::G5]
    }
}

#[derive(Eq, Hash, PartialEq)]
pub enum LongRegisters {
    PC, ADDR,
    LL1, LL2,
    GP1, GP2, GP3,
}

impl LongRegisters {
    pub fn to_bytecode(&self) -> u8 {
        match self {
            LongRegisters::PC => {0xB1}
            LongRegisters::ADDR => {0xB2}

            LongRegisters::LL1 => {0xC1}
            LongRegisters::LL2 => {0xC2}

            LongRegisters::GP1 => {0xD1}
            LongRegisters::GP2 => {0xD2}
            LongRegisters::GP3 => {0xD3}
        }
    }
    
    pub fn from_bytecode(code: u8) -> LongRegisters {
        match code {
            0xB1 => LongRegisters::PC,
            0xB2 => LongRegisters::ADDR,
            0xC1 => LongRegisters::LL1,
            0xC2 => LongRegisters::LL2,
            0xD1 => LongRegisters::GP1,
            0xD2 => LongRegisters::GP2,
            0xD3 => LongRegisters::GP3,
            _ => panic!("Uh... Corrupted program ig"),
        }
    }

    pub fn all() -> Vec<LongRegisters> {
        vec![LongRegisters::PC, LongRegisters::ADDR, LongRegisters::LL1, LongRegisters::LL2,
            LongRegisters::GP1, LongRegisters::GP2, LongRegisters::GP3]
    }
}