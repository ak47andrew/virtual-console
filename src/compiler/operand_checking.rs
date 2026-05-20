use crate::compiler::operands::OperandKind;
use once_cell::sync::Lazy;

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
            vec![OperandKind::Address, OperandKind::Address],
            vec![OperandKind::Address, OperandKind::Register],
            vec![OperandKind::Address, OperandKind::LongRegister],
            vec![OperandKind::Register, OperandKind::Address],
            vec![OperandKind::Register, OperandKind::Register],
            vec![OperandKind::Register, OperandKind::LongRegister],
            vec![OperandKind::Immediate, OperandKind::Register],
            vec![OperandKind::Immediate, OperandKind::Address]
        ]
    }
});
