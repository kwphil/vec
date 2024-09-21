use num::Integer;

mod test;
mod vec2;
mod vec3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2<T> 
where 
    T: Integer + Copy,
{
    pub x: T,
    pub y: T,
}

/* impl for Vec2<T>
 * 
 * pub fn new() -> Self
 * returns an empty Vec2
 * 
 * pub fn from(x: T, y: T) -> Self
 * sets x and y at declaration
 * 
 * pub fn add(other: Vec2<T>) -> Vec2<T>
 * adds the x and y
 * 
 * pub fn sub(other: Vec2<T>) -> Vec2<T>
 * subs the x and y
 */

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3<T>
where
    T: Integer + Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
}
