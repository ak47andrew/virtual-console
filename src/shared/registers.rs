#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub enum Registers {
    A, X, Y, Z,
    G1, G2, G3, G4, G5,
}

impl Registers {
    pub fn to_bytecode(&self) -> u8 {
        match self {
            Registers::A => { 0x0 }
            Registers::X => { 0x1 }
            Registers::Y => { 0x2 }
            Registers::Z => { 0x3 }

            Registers::G1 => { 0x4 }
            Registers::G2 => { 0x5 }
            Registers::G3 => { 0x6 }
            Registers::G4 => { 0x7 }
            Registers::G5 => { 0x8 }
        }
    }
    
    pub fn from_bytecode(code: u8) -> Registers {
        match code { 
            0x0 => Registers::A,
            0x1 => Registers::X,
            0x2 => Registers::Y,
            0x3 => Registers::Z,
            0x4 => Registers::G1,
            0x5 => Registers::G2,
            0x6 => Registers::G3,
            0x7 => Registers::G4,
            0x8 => Registers::G5,
            _ => panic!("Uh... Corrupted program ig"),
        }
    }

    pub fn all() -> Vec<Registers> {
        vec![Registers::A, Registers::X, Registers::Y, Registers::Z,
             Registers::G1, Registers::G2, Registers::G3, Registers::G4, Registers::G5]
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub enum LongRegisters {
    PC,
    LL1, LL2,
    GP1, GP2, GP3,
}

impl LongRegisters {
    pub fn to_bytecode(&self) -> u8 {
        match self {
            LongRegisters::PC => {0x0}

            LongRegisters::LL1 => {0x1}
            LongRegisters::LL2 => {0x2}

            LongRegisters::GP1 => {0x3}
            LongRegisters::GP2 => {0x4}
            LongRegisters::GP3 => {0x5}
        }
    }
    
    pub fn from_bytecode(code: u8) -> LongRegisters {
        match code {
            0x0 => LongRegisters::PC,
            0x1 => LongRegisters::LL1,
            0x2 => LongRegisters::LL2,
            0x3 => LongRegisters::GP1,
            0x4 => LongRegisters::GP2,
            0x5 => LongRegisters::GP3,
            _ => panic!("Uh... Corrupted program ig"),
        }
    }

    pub fn all() -> Vec<LongRegisters> {
        vec![LongRegisters::PC, LongRegisters::LL1, LongRegisters::LL2,
            LongRegisters::GP1, LongRegisters::GP2, LongRegisters::GP3]
    }
}