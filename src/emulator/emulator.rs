use raylib::prelude::{Color, RaylibTexture2D, Texture2D};
use crate::consts::TARGET_RESOLUTION;

pub struct Emulator {
    framebuffer: Vec<u32>
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            framebuffer: vec![0; (TARGET_RESOLUTION.x * TARGET_RESOLUTION.y) as usize]
        }
    }

    pub fn update_frame(&self, texture: &mut Texture2D) {
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                self.framebuffer.as_ptr() as *const u8,
                self.framebuffer.len() * 4,
            )
        };

        texture.update_texture(bytes).expect("Failed to update texture");
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.framebuffer[(y * TARGET_RESOLUTION.x + x) as usize] =
            (color.r as u32)
            | ((color.g as u32) << 8)
            | ((color.b as u32) << 16)
            | ((color.a as u32) << 24);
    }
}
