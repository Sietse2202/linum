use alloc::vec::Vec;
use num_traits::{Num, NumCast};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec3D<T: Num + NumCast + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub fn vec3d<T: Num + NumCast + Copy>(x: T, y: T, z: T) -> Vec3D<T> {
    Vec3D { x, y, z }
}

impl<T: Num + NumCast + Copy> Vec3D<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3D<T> {
        Vec3D { x, y, z }
    }

    pub fn cast<U: Num + NumCast + Copy>(self) -> Option<Vec3D<U>> {
        Some(
            Vec3D {
                x: U::from(self.x).expect("Could not cast types"),
                y: U::from(self.y).expect("Could not cast types"),
                z: U::from(self.z).expect("Could not cast types"),
            }
        )
    }
}

impl<T: Num + NumCast + Copy> Default for Vec3D<T> {
    fn default() -> Self {
        Vec3D::new(T::zero(), T::zero(), T::zero())
    }
}

impl<T: Num + NumCast + Copy> From<(T, T, T)> for Vec3D<T> {
    fn from(value: (T, T, T)) -> Self {
        Vec3D {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl<T: Num + NumCast + Copy> From<[T; 3]> for Vec3D<T> {
    fn from(value: [T; 3]) -> Self {
        Vec3D {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl<T: Num + NumCast + Copy> From<Vec<T>> for Vec3D<T> {
    fn from(value: Vec<T>) -> Self {
        if value.len() != 3 {
            panic!("Vec3D requires exactly 3 elements");
        }
        Vec3D {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}
impl<T: Num + NumCast + Copy> core::ops::Mul<T> for Vec3D<T> {
    type Output = Vec3D<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Num + NumCast + Copy> core::ops::Div<T> for Vec3D<T> {
    type Output = Vec3D<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec3D {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Num + NumCast + Copy> core::ops::Add for Vec3D<T> {
    type Output = Vec3D<T>;
    
    fn add(self, rhs: Self) -> Self::Output {
        Vec3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Num + NumCast + Copy> core::ops::Sub for Vec3D<T> {
    type Output = Vec3D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
