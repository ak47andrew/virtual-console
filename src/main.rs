use std::env;
use raylib::init;
use raylib::prelude::{Color, Image, RaylibDraw, RaylibTexture2D, Rectangle, TextureFilter, Vector2};
use raylib::prelude::KeyboardKey::KEY_SPACE;
use crate::compiler::entry::entry;
use crate::consts::{SCREEN_SIZE, TARGET_RESOLUTION};
use crate::emulator::emulator::Emulator;

mod consts;
pub mod emulator;
mod helper;
pub mod compiler;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    println!("{:?}", args);
    if args.len() == 3 && args[1] == "compile" {
        entry(args[2].as_str());
        return;
    }
    #[allow(unused_mut)]
    let (mut rl, mut thread) = init()
        .size(SCREEN_SIZE.x, SCREEN_SIZE.y)
        .title("Rust Raylib")
        .build();
    let mut emulator = Emulator::new();
    let mut texture = rl.load_texture_from_image(&thread,
         &Image::gen_image_color(TARGET_RESOLUTION.x, TARGET_RESOLUTION.y, Color::BLACK)).unwrap();
    texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);

    while !rl.window_should_close() {
        emulator.update_frame(&mut texture);

        let mut d = rl.begin_drawing(&thread);

        d.draw_texture_pro(
            &texture,
            Rectangle::new(
                0.0, 0.0,
                texture.width() as f32, texture.height() as f32
            ),
            Rectangle::new(0.0, 0.0, SCREEN_SIZE.x as f32, SCREEN_SIZE.y as f32),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE
        )
    }
}
