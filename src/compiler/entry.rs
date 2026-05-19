use std::io::Write;
use num_bigint::BigUint;
use num_traits::{ToBytes, ToPrimitive};
use crate::compiler::operands::{parse_operand, parse_u64_num, Operand};

pub fn entry(file: &str) {
    // file ends in .vea (virtual emulator assembly)
    let file_content = std::fs::read_to_string(file).unwrap();
    let lines = file_content.lines().collect::<Vec<&str>>();

    let mut output = vec![];
    for line in lines {
        if line.starts_with(";") {continue;}
        output.extend(compile(line))
    }

    let mut file = std::fs::File::create(
        [file.split(".").collect::<Vec<&str>>()[0], ".veb"].concat(), // .veb = virtual emulator binary
    ).unwrap();
    file.write_all(&output).unwrap();
}

pub fn compile(line: &str) -> Vec<u8> {
    let parts = line.split(" ").collect::<Vec<&str>>();
    if parts.len() == 0 {
        return vec![];
    }
    let op = parts[0];
    let args = &parts[1..];

    match op {
        "mov" => {  // instruction to move value to specified address in memory
            if args.len() != 2 {
                println!("Wrong number of arguments for mov. It's gonna be treated as noop");
                return vec![0]
            };

            let arg1 = match parse_operand(args[0].to_string()) {
                Ok(v) => v,
                Err(e) => {
                    println!("Parse error: {:?}. Treating as noop", e);
                    return vec![0];
                }
            };
            let arg2 = match parse_operand(args[1].to_string()) {
                Ok(v) => v,
                Err(e) => {
                    println!("Parse error: {:?}. Treating as noop", e);
                    return vec![0];
                }
            };

            match (arg1, arg2) {
                (Operand::Address(addr1), Operand::Address(addr2)) => {
                    let mut output = vec![];
                    output.push(1);
                    output.push(0);
                    output.extend(addr1.to_be_bytes().to_vec());
                    output.extend(addr2.to_be_bytes().to_vec());
                    output
                }
                (Operand::Address(addr), Operand::Register(reg)) => {
                    let mut output = vec![];
                    output.push(1);
                    output.push(1);
                    output.extend(addr.to_be_bytes().to_vec());
                    output.push(reg.to_u8());
                    output
                }
                (Operand::Address(addr), Operand::LongRegister(reg)) => {
                    let mut output = vec![];
                    output.push(1);
                    output.push(2);
                    output.extend(addr.to_be_bytes().to_vec());
                    output.push(reg.to_u8());
                    output
                }
                (Operand::Register(reg), Operand::Address(addr)) => {
                    let mut output = vec![];
                    output.push(1);
                    output.push(3);
                    output.push(reg.to_u8());
                    output.extend(addr.to_be_bytes().to_vec());
                    output
                }
                (Operand::Register(reg1), Operand::Register(reg2)) => {
                    let mut output = vec![];
                    output.push(1);
                    output.push(4);
                    output.push(reg1.to_u8());
                    output.push(reg2.to_u8());
                    output
                }
                (Operand::Register(reg1), Operand::LongRegister(reg2)) => {
                    let mut output = vec![];
                    output.push(1);
                    output.push(5);
                    output.push(reg1.to_u8());
                    output.push(reg2.to_u8());
                    output
                }
                (Operand::Immediate(value), Operand::Register(reg)) => {
                    if value > BigUint::from(u8::MAX) {
                        println!("Immediate value should be 1 byte in size. Line's gonna be treated as noop");
                        return vec![0];
                    }
                    let mut output = vec![];
                    output.push(1);
                    output.push(6);
                    output.push(value.to_u8().unwrap());
                    output.push(reg.to_u8());
                    output
                }
                (Operand::Immediate(value), Operand::Address(addr)) => {
                    if value > BigUint::from(u8::MAX) {
                        println!("Immediate value should be 1 byte in size. Line's gonna be treated as noop");
                        return vec![0];
                    }
                    let mut output = vec![];
                    output.push(1);
                    output.push(7);
                    output.push(value.to_u8().unwrap());
                    output.extend(addr.to_be_bytes().to_vec());
                    output
                }
                (_, _) => {
                    println!("Unknown/Illegal combination of operands for mov. Line's gonna be treated as noop");
                    vec![0]
                }
            }
        }
        "movl" => {  // 2
            if args.len() != 2 {
                println!("Wrong number of arguments for mov. It's gonna be treated as noop");
                return vec![0]
            };

            let arg1 = match parse_operand(args[0].to_string()) {
                Ok(v) => v,
                Err(e) => {
                    println!("Parse error: {:?}. Treating as noop", e);
                    return vec![0];
                }
            };
            let arg2 = match parse_operand(args[1].to_string()) {
                Ok(v) => v,
                Err(e) => {
                    println!("Parse error: {:?}. Treating as noop", e);
                    return vec![0];
                }
            };

            match (arg1, arg2) {
                (Operand::Immediate(v), Operand::LongRegister(lr)) => {
                    let mut s = v.to_be_bytes();
                    if s.len() > 8 {
                        println!("Amount of bytes isn't corresponding to immediate value size. Line's gonna be treated as noop");
                        return vec![0]
                    }
                    if s.len() < 8 {
                        let mut padding = vec![0u8; 8 - s.len()];
                        padding.extend_from_slice(s.as_slice());
                        s = padding;
                    }

                    let mut output = vec![];
                    output.push(2);
                    output.push(0);
                    output.extend(s);
                    output.push(lr.to_u8());
                    output
                }
                (Operand::LongRegister(rl), Operand::Address(addr)) => {
                    let mut output = vec![];
                    output.push(2);
                    output.push(1);
                    output.push(rl.to_u8());
                    output.extend(addr.to_be_bytes().to_vec());
                    output
                }
                (Operand::LongRegister(rl1), Operand::LongRegister(rl2)) => {
                    let mut output = vec![];
                    output.push(2);
                    output.push(2);
                    output.push(rl1.to_u8());
                    output.push(rl2.to_u8());
                    output
                }
                (_, _) => {
                    println!("Unknown/Illegal combination of operands for movl. Line's gonna be treated as noop");
                    vec![0]
                }
            }
        }
        "hlt" => {  // Stop execution and enter endless loop
            vec![3]
        }
        "load" => {
            if args.len() != 3 {
                println!("Wrong number of arguments for load. It's gonna be treated as noop");
                return vec![0]
            }

            let n = match args[0].parse::<u8>() {
                Ok(v) => {
                    if v > 8 {
                        println!("Limit for amount of bytes to load is 8. Line's gonna be treated as noop");
                        return vec![0]
                    }

                    v
                },
                Err(e) => {
                    println!("Limit for amount of bytes to load is 8. Line's gonna be treated as noop");
                    return vec![0]
                }
            };

            let value = match parse_operand(args[1].to_string()) {
                Ok(v) => {
                    match v {
                        Operand::Immediate(imm) => {
                            let mut s = imm.to_be_bytes();
                            if s.len() > n as usize {
                                println!("Amount of bytes isn't corresponding to immediate value size. Line's gonna be treated as noop");
                                return vec![0]
                            }
                            if s.len() < n as usize {
                                let mut padding = vec![0u8; n as usize - s.len()];
                                padding.extend_from_slice(s.as_slice());
                                s = padding;
                            }
                            s
                        }
                        _ => {
                            println!("Second argument should be an immediate value. Line's gonna be treated as noop");
                            return vec![0]
                        }
                    }
                }
                Err(e) => {
                    println!("Error parsing immediate value");
                    return vec![0]
                }
            };

            let addr = match parse_u64_num(args[2].to_string()) {
                Ok(v) => v,
                Err(e) => {
                    println!("Third argument should be an address. Line's gonna be treated as noop");
                    return vec![0]
                }
            };

            let mut output = vec![];
            output.push(4);
            output.push(0);
            output.push(n);
            output.extend(value);
            output.extend(addr.to_be_bytes().to_vec());
            output
        }
        _ => {
            println!("WARNING: Unknown op {}. Line's gonna be treated as noop", op);
            vec![0]
        }
    }
}