use num_traits::{Num, NumCast};
use raylib::prelude::Vector2;

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T: Num + NumCast> {
    pub x: T,
    pub y: T,
}

impl<T> From<Vec2<T>> for Vector2
where T: NumCast + Copy + Num
{
    fn from(v: Vec2<T>) -> Self {
        Vector2::new(
            NumCast::from(v.x).unwrap_or(0.0),
            NumCast::from(v.y).unwrap_or(0.0),
        )
    }
}

impl<T> From<Vector2> for Vec2<T>
where T: Num + NumCast + Copy
{
    fn from(v: Vector2) -> Self {
        Self {
            x: NumCast::from(v.x).unwrap_or(NumCast::from(0).unwrap()),
            y: NumCast::from(v.y).unwrap_or(NumCast::from(0).unwrap()),
        }
    }
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
