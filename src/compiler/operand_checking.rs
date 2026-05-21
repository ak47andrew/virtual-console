use crate::compiler::operands::OperandKind;
use once_cell::sync::Lazy;
use crate::compiler::operands::OperandKind::{Address, Immediate, LongImmediate, LongRegister, LongerImmediate, Register};

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
            vec![LongerImmediate, LongRegister],
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
            vec![LongRegister, Address, Address],
            vec![LongerImmediate, Address, Address],
        ]
    }
});
