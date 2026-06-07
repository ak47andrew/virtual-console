use vea_shared::opcodes::Opcode;
use vea_shared::operand_types::OperandKind;
use crate::vea::operand_checking::get_signature;

pub enum CompilationError {
    UnknownRegister(String),
    IncorrectIndirectAddressBody(String),
    UnknownOpcode(String),
    InvalidOperand(String),
    ImmediateOverflow(String),
    LongImmediateOverflow(String),
    AddressOverflow(String),
    InvalidOpcodeSignature((Opcode, Vec<OperandKind>)),
}

impl CompilationError {
    pub fn to_string_error(&self) -> String {
        match self {
            CompilationError::UnknownRegister(reg) => {
                format!("unknown register '{}'", reg)
            }
            CompilationError::IncorrectIndirectAddressBody(d) => {
                format!("operand inside indirect address should be Long Register. '{}' isn't one", d)
            }
            CompilationError::UnknownOpcode(opcode) => {
                format!("unknown opcode '{}'", opcode)
            }
            CompilationError::InvalidOperand(operand) => {
                format!("invalid/unknown operand '{}'", operand)
            }
            CompilationError::ImmediateOverflow(d) => {
                format!("{} >= 256. Use prefix '&' to turn it to LongImmediate, if applicable by instruction", d)
            }
            CompilationError::LongImmediateOverflow(d) => {
                format!("{} >= 2 ^ 64. Use prefix '^' to turn it to LongerImmediate, if applicable by instruction", d)
            }
            CompilationError::AddressOverflow(d) => {
                format!("{} >= 2 ^ 64. It's impossible to access memory over this limit", d)
            }
            CompilationError::InvalidOpcodeSignature((op, sig)) => {
                format!("You can't use signature {} for instruction *{:?}*. Available signatures are:\n{}",
                signature_to_string(sig), op, available_signatures_string(op))
            }
        }
    }
}

fn signature_to_string(sig: &Vec<OperandKind>) -> String {
    let s = sig.iter().map(|kind| format!("{:?}", kind)).collect::<Vec<_>>().join(", ");
    format!("[{}]", s)
}

fn available_signatures_string(op: &Opcode) -> String {
    get_signature(*op).operands_variations
        .iter()
        .map(signature_to_string)
        .map(|x| format!("    {}", x))
        .collect::<Vec<_>>()
        .join("\n")
}