use std::env;
use raylib::init;
use raylib::prelude::{Color, Image, RaylibDraw, RaylibTexture2D, Rectangle, TextureFilter, Vector2};
use crate::compiler::entry::entry;
use crate::consts::{SCREEN_SIZE, TARGET_RESOLUTION};
use crate::emulator::emulator::Emulator;

pub mod consts;
pub mod emulator;
pub mod helper;
pub mod compiler;
pub mod shared;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0);
    println!("{:?}", args);
    if args.len() == 2 && args[0] == "compile" {
        entry(args[1].as_str());
        return;
    }
    #[allow(unused_mut)]
    let (mut rl, mut thread) = init()
        .size(SCREEN_SIZE.x, SCREEN_SIZE.y)
        .title("Rust Raylib")
        .build();
    let mut emulator = Emulator::new(
        if args.len() == 0 {
            None
        } else {
            Some(args[0].clone())
        }
    );
    emulator.load_program_to_rom();
    let mut texture = rl.load_texture_from_image(&thread,
         &Image::gen_image_color(TARGET_RESOLUTION.x, TARGET_RESOLUTION.y, Color::BLACK)).unwrap();
    texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);

    while !rl.window_should_close() {
        emulator.step();
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
