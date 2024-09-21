use super::Vec2;
use num::Integer;

impl<T> Vec2<T> 
where
    T: Integer + Copy,
{
    pub fn new() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn from(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn add(self, other: Vec2<T>) -> Vec2<T> {
        Vec2::from(self.x + other.x, self.y + other.y)
    }

    pub fn sub(self, other: Vec2<T>) -> Vec2<T> {
        Vec2::from(self.x - other.x, self.y - other.y)
    }
}