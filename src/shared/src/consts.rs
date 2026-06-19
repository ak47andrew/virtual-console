use crate::helper::Vec2;

pub const TARGET_RESOLUTION: Vec2<u64> = Vec2 {x: 256, y: 240};
pub const SCREEN_SIZE: Vec2<u64> = Vec2 {x: 1024, y: 960};
pub const MAX_INSTRUCTIONS_RAN_CHUNK: u64 = 69_000_000;  // https://youtu.be/s3T5ZrlMhDI?si=Quo8VZjwbSDIdtZz&t=27