use super::Vec3;
use num::Integer;

impl<T> Vec3<T> 
where
    T: Integer + Copy,
{
    pub fn new() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn from(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn add(self, other: Vec3<T>) -> Vec3<T> {
        Vec3::from(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(self, other: Vec3<T>) -> Vec3<T> {
        Vec3::from(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}