use crate::helper::Vec2;

pub const TARGET_RESOLUTION: Vec2<i32> = Vec2 {x: 256, y: 240};
pub const SCREEN_SIZE: Vec2<i32> = Vec2 {x: 1024, y: 960};
pub const RAM_SIZE: usize = 250_000;
pub const STACK_SIZE: usize = 8192;