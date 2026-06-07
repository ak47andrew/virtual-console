use std::fs::read_to_string;
use std::path::PathBuf;
use log::{error, warn};

pub fn parse_palette(path: &PathBuf) -> Option<Vec<u8>> {
    let fuck_rust_borrow_checker = match read_to_string(path) {
        Ok(s) => s,
        Err(_) => {
            error!("File {} can't be read", path.display());
            return None;
        }
    };
    let contents = fuck_rust_borrow_checker.split("\n").collect::<Vec<&str>>();

    if contents.len() < 3 {
        error!("Invalid palette file. Invalid header (Less then 3 lines)");
        return None;
    }

    if contents[0] != "JASC-PAL" {
        error!("Invalid palette file. Invalid header (Invalid line-0 type)");
        return None;
    }

    let num = contents[2].parse::<u8>();
    if num.is_err() {
        error!("You can't have more then 255 colors in a palette");
        return None;
    }
    let num = num.unwrap();
    if num == 0 {
        error!("You should have at least one color in a palette");
        return None;
    }

    let mut out = vec![];

    if contents[3] != "0 0 0 0" {
        warn!("Color index 0 (first one) will be set to transparent. Please set it as those in the palette or you'll be confused by colors in-game")
    }
    out.extend(vec![0, 0, 0, 0]);
    for line in contents[4..].iter() {
        if line == &"" {continue;}
        let nums = line.split(" ").collect::<Vec<&str>>();
        if nums.len() != 4 {
            error!("Invalid palette file. Invalid line type (non-4 numbers. Try to export it from RGBA preset)");
            return None;
        }
        for num in nums {
            match num.parse::<u8>() {
                Ok(num) => out.push(num),
                Err(_) => {
                    error!("Invalid palette file. Invalid color entry (non-number in one channel). Line: {}", line);
                    return None;
                }
            }
        }
    }

    Some(out)
}