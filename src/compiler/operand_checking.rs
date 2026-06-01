use crate::shared::operand_types::OperandKind;
use crate::shared::operand_types::OperandKind::{Address, Immediate, IndirectAddress, LongImmediate, LongRegister, LongerImmediate, Register};
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

            vec![Immediate, IndirectAddress],
            vec![LongImmediate, IndirectAddress],
            vec![LongerImmediate, IndirectAddress],
            vec![Register, IndirectAddress],
            vec![LongRegister, IndirectAddress],

            vec![Address, Register],
            vec![Address, LongRegister],
            vec![Register, Register],
            vec![Immediate, Register],
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

            vec![Immediate, IndirectAddress, Address],
            vec![LongImmediate, IndirectAddress, Address],
            vec![LongerImmediate, IndirectAddress, Address],
            vec![Register, IndirectAddress, Address],
            vec![LongRegister, IndirectAddress, Address],

            vec![Immediate, Address, IndirectAddress],
            vec![LongImmediate, Address, IndirectAddress],
            vec![LongerImmediate, Address, IndirectAddress],
            vec![Register, Address, IndirectAddress],
            vec![LongRegister, Address, IndirectAddress],

            vec![Immediate, IndirectAddress, IndirectAddress],
            vec![LongImmediate, IndirectAddress, IndirectAddress],
            vec![LongerImmediate, IndirectAddress, IndirectAddress],
            vec![Register, IndirectAddress, IndirectAddress],
            vec![LongRegister, IndirectAddress, IndirectAddress],
        ]
    }
});

pub const BINARY_MATH_SIGNATURE: Lazy<InstructionSignature> = Lazy::new(|| {
    InstructionSignature {
        operands_variations: vec![
            vec![Immediate, Immediate],  // 1 byte
            vec![LongImmediate, LongImmediate],  // 8 bytes
            vec![Register, Register],  // 1 byte
            vec![LongRegister, LongRegister], // 8 bytes
            vec![Immediate, Register],
            vec![Register, Immediate],
            vec![LongImmediate, LongRegister],
            vec![LongRegister, LongImmediate],
        ]
    }
});

pub const NOT_SIGNATURE: Lazy<InstructionSignature> = Lazy::new(|| {
    InstructionSignature {
        operands_variations: vec![
            vec![Immediate],
            vec![LongImmediate],
            vec![Register],
            vec![LongRegister]
        ]
    }
});

pub const SHIFTS_SIGNATURE: Lazy<InstructionSignature> = Lazy::new(|| {
    InstructionSignature {
        operands_variations: vec![
            // 1 byte by 1 byte
            vec![Immediate, Immediate],
            vec![Register, Immediate],
            vec![Immediate, Register],

            // 8 bytes by 1 byte
            vec![LongImmediate, Immediate],
            vec![LongImmediate, Register],
            vec![LongRegister, Immediate],
            vec![LongRegister, Register],

            // 8 bytes by 8 bytes
            vec![LongRegister, LongRegister],
            vec![LongRegister, LongImmediate],
            vec![LongImmediate, LongRegister],
            vec![LongImmediate, LongImmediate],
        ]
    }
});

pub const JMP_SIGNATURE: Lazy<InstructionSignature> = Lazy::new(|| {
    InstructionSignature {
        operands_variations: vec![
            vec![Address],
            vec![IndirectAddress]
        ]
    }
});

pub const CONDITIONAL_JUMP_SIGNATURE: Lazy<InstructionSignature> = Lazy::new(|| {
    InstructionSignature {
        operands_variations: vec![
            vec![Register, Address],
            vec![Register, IndirectAddress],
            vec![LongRegister, Address],
            vec![LongRegister, IndirectAddress],
        ]
    }
});

pub const EMPTY_SIGNATURE: Lazy<InstructionSignature> = Lazy::new(|| {
    InstructionSignature {
        operands_variations: vec![vec![]]
    }
});

pub const PUSH_SIGNATURE: Lazy<InstructionSignature> = Lazy::new(|| {
    InstructionSignature {
        operands_variations: vec![
            vec![Immediate],
            vec![Register],
        ]
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
        Opcode::Copy => COPY_SIGNATURE,
        Opcode::Add => BINARY_MATH_SIGNATURE,
        Opcode::Sub => BINARY_MATH_SIGNATURE,
        Opcode::Mul => BINARY_MATH_SIGNATURE,
        Opcode::Div => BINARY_MATH_SIGNATURE,
        Opcode::And => BINARY_MATH_SIGNATURE,
        Opcode::Or => BINARY_MATH_SIGNATURE,
        Opcode::Xor => BINARY_MATH_SIGNATURE,
        Opcode::Not => NOT_SIGNATURE,
        Opcode::Shr => SHIFTS_SIGNATURE,
        Opcode::Shl => SHIFTS_SIGNATURE,
        Opcode::Jmp => JMP_SIGNATURE,
        Opcode::Je => CONDITIONAL_JUMP_SIGNATURE,
        Opcode::Jne => CONDITIONAL_JUMP_SIGNATURE,
        Opcode::PUSH => PUSH_SIGNATURE,
        Opcode::POP => EMPTY_SIGNATURE,
        Opcode::RET => EMPTY_SIGNATURE,
        Opcode::CALL => JMP_SIGNATURE,
    }
}