use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use log::error;
use png::{ColorType, Decoder};
use vea_shared::consts::TARGET_RESOLUTION;
use vea_shared::helper::Vec2;

pub fn encode_bg(filename: PathBuf) -> Option<Vec<u8>> {
    encode_img(filename, TARGET_RESOLUTION.cast(), "bg")
}

pub fn encode_chr(filename: PathBuf) -> Option<Vec<u8>> {
    encode_img(filename, Vec2 { x: 8, y: 8 }, "sprite")
}

fn encode_img(filename: PathBuf, dim: Vec2<u32>, filetype: &str) -> Option<Vec<u8>> {
    let file_str = filename.to_string_lossy().to_string();
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            error!("Error opening file: {:?}", file_str);
            return None;
        }
    };
    let reader = BufReader::new(file);
    let decoder = Decoder::new(reader);
    let mut png_reader = match decoder.read_info() {
        Ok(r) => r,
        Err(_) => {
            error!("Error decoding file {} as png", file_str);
            return None;
        }
    };
    let info = png_reader.info();

    if info.color_type != ColorType::Indexed {
        error!("File {} isn't a palette-indexed png. Recheck your color export settings", file_str);
        return None;
    }

    if info.width != dim.x || info.height != dim.y {
        error!("File {} has wrong image dimensions. It should be {}x{} for {}", file_str, dim.x, dim.y, filetype);
        return None;
    }

    // TODO: maybe check palette and throw a warning if it's not matching
    // Here's the code snippet for that:
    // let palette = info.palette.clone().unwrap(); // Vec<u8>, groups of 3

    let mut buf = vec![0; png_reader.output_buffer_size()?];
    let frame = png_reader.next_frame(&mut buf).unwrap();
    let pixels = buf[..frame.buffer_size()].to_vec();

    Some(pixels)
}