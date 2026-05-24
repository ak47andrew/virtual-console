use crate::shared::operand_types::OperandKind;
use crate::shared::operand_types::OperandKind::{Address, Immediate, LongImmediate, LongRegister, LongerImmediate, Register};
use once_cell::sync::Lazy;
use crate::shared::opcodes::Opcode;

pub struct InstructionSignature {
    pub operands_variations: Vec<Vec<OperandKind>>,
}

impl InstructionSignature {
    pub fn check(&self, operands: &Vec<OperandKind>) -> bool {
        self.operands_variations.contains(operands)
    }
}

pub const MOV_SIGNATURE: Lazy<InstructionSignature> = Lazy::new(|| {
    InstructionSignature {
        operands_variations: vec![
            vec![Immediate, Address],
            vec![LongImmediate, Address],
            vec![LongerImmediate, Address],
            vec![Register, Address],
            vec![LongRegister, Address],
            vec![Address, Register],
            vec![Register, Register],
            vec![LongImmediate, LongRegister],
            vec![LongRegister, LongRegister]
        ]
    }
});

pub const TRUNC_SIGNATURE: Lazy<InstructionSignature> = Lazy::new(|| {
    InstructionSignature {
        operands_variations: vec![
            vec![LongImmediate, Register],
            vec![LongerImmediate, Register],
            vec![LongRegister, Register],
            vec![LongerImmediate, LongRegister],
        ]
    }
});

pub const EXT_SIGNATURE: Lazy<InstructionSignature> = Lazy::new(|| {
    InstructionSignature {
        operands_variations: vec![
            vec![Address, LongRegister],
            vec![Immediate, LongRegister],
            vec![Register, LongRegister]
        ]
    }
});

pub const COPY_SIGNATURE: Lazy<InstructionSignature> = Lazy::new(|| {
    InstructionSignature {
        operands_variations: vec![
            vec![Immediate, Address, Address],
            vec![LongImmediate, Address, Address],
            vec![LongerImmediate, Address, Address],
            vec![Register, Address, Address],
            vec![LongRegister, Address, Address],
        ]
    }
});

pub const EMPTY_SIGNATURE: Lazy<InstructionSignature> = Lazy::new(|| {
    InstructionSignature {
        operands_variations: vec![vec![]]
    }
});

pub fn get_signature(opcode: Opcode) -> Lazy<InstructionSignature> {
    match opcode {
        Opcode::Noop => EMPTY_SIGNATURE,
        Opcode::Hlt => EMPTY_SIGNATURE,
        Opcode::Vsync => EMPTY_SIGNATURE,
        Opcode::Mov => MOV_SIGNATURE,
        Opcode::Trunc => TRUNC_SIGNATURE,
        Opcode::Ext => EXT_SIGNATURE,
        Opcode::Copy => COPY_SIGNATURE
    }
}