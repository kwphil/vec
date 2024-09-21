#[cfg(test)]

use crate::*;

mod tests {
    use super::*;

    #[test]
    fn nothin() {}

    #[test]
    fn vec2() {
        let mut a = Vec2::<i32>::new();

        assert_eq!(a, Vec2::from(0, 0));

        let b = Vec2::<i32>::from(3, 4);

        a = Vec2 {
            x: 1,
            y: 3,
        };

        assert_eq!(a.add(b), Vec2::from(4, 7));
    }

    #[test]
    fn vec3() {
        let mut a = Vec3::<i32>::new();

        assert_eq!(a, Vec3::from(0, 0, 0));

        let b = Vec3::<i32>::from(3, 4, 5);

        a = Vec3 {
            x: 1,
            y: 3,
            z: 5,
        };

        assert_eq!(a.add(b), Vec3::from(4, 7, 10));
    }
}