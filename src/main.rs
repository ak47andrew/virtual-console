use std::cmp::min;
use num_traits::Signed;
use raylib::init;
use raylib::prelude::{Color, Image, RaylibDraw, RaylibTexture2D, Rectangle, TextureFilter, Vector2};
use crate::consts::{SCREEN_SIZE, TARGET_RESOLUTION};
use crate::emulator::emulator::Emulator;

mod consts;
pub mod emulator;
mod helper;

fn main() {
    #[allow(unused_mut)]
    let (mut rl, mut thread) = init()
        .size(SCREEN_SIZE.x, SCREEN_SIZE.y)
        .title("Rust Raylib")
        .build();
    // rl.set_target_fps(60);
    let mut emulator = Emulator::new();
    let mut texture = rl.load_texture_from_image(&thread,
         &Image::gen_image_color(TARGET_RESOLUTION.x, TARGET_RESOLUTION.y, Color::BLACK)).unwrap();
    texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);
    let mut i = 0;
    let mut counter = 0;
    let limit = min(TARGET_RESOLUTION.x, TARGET_RESOLUTION.y) - 1;

    while !rl.window_should_close() {
        counter += 1;
        if counter == 5 {
            let t = rl.get_time() as f32;

            let x = ((t * 123.45).sin() * 1000.0) as i32 % (TARGET_RESOLUTION.x - 1);
            let y = ((t * 678.90).cos() * 1000.0) as i32 % (TARGET_RESOLUTION.y - 1);
            counter = 0;
            let time = (rl.get_time() * 123.4 % 255.0) as u8;
            let color = Color::new(
                time << 5 ^ time,
                time >> 2 ^ time,
                (time % 255 / 2) * 2 << 5 ^ time,
                255,
            );
            println!("Color: {:?}", color);
            emulator.put_pixel(x.abs(), y.abs(), color);
        }

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
