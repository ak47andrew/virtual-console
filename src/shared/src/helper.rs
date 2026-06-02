use num_traits::{Num, NumCast};

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T: Num + NumCast> {
    pub x: T,
    pub y: T,
}

impl<T: Num + NumCast + Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn cast<U: Num + NumCast + Copy>(self) -> Vec2<U> {
        Vec2 {
            x: NumCast::from(self.x).unwrap(),
            y: NumCast::from(self.y).unwrap(),
        }
    }
}