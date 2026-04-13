use std::io::Write;

pub fn entry(file: &str) {
    // file ends in .vea (virtual emulator assembly)
    let file_content = std::fs::read_to_string(file).unwrap();
    let lines = file_content.lines().collect::<Vec<&str>>();

    let mut output = vec![];
    for line in lines {
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

            let value = args[0].parse::<u8>().unwrap();
            let addr = u64::from_str_radix(args[1].strip_prefix("0x").unwrap(), 16).unwrap();

            let mut output = vec![];
            output.push(1); // op code
            output.extend(addr.to_be_bytes());
            output.push(value);

            output
        }
        "hlt" => {  // Stop execution and enter endless loop
            vec![2]
        }
        _ => {
            println!("WARNING: Unknown op {}. It's gonna be treated as noop", op);
            vec![0]
        }
    }
}