use crate::NumVec;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Index;
use num_traits::Float;

/// Basic 3D Vector
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec3D<T: NumVec> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub fn vec3d<T: NumVec>(x: T, y: T, z: T) -> Vec3D<T> {
    Vec3D { x, y, z }
}

impl<T: NumVec> Vec3D<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3D<T> {
        Vec3D { x, y, z }
    }

    pub fn cast<U: NumVec>(self) -> Option<Vec3D<U>> {
        Some(
            Vec3D {
                x: U::from(self.x)?,
                y: U::from(self.y)?,
                z: U::from(self.z)?,
            }
        )
    }

    /// Due to type casting to f64, using this isn't advised, please use the `magnitude` method instead.
    pub fn magnitude_cast(&self) -> f64 {
        let casted: Vec3D<f64> = self.cast().unwrap();
        let x: f64 = casted.x;
        let y: f64 = casted.y;
        let z: f64 = casted.z;
        (x * x + y * y + z * z).sqrt()
    }

    pub fn normalize(&self) -> Option<Vec3D<T>> where T: Float {
        let mag = self.magnitude();
        if mag == T::zero() {
            None
        } else {
            Some(Vec3D {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            })
        }
    }

    /// Dot-Product of the two vectors
    pub fn dot(&self, other: &Vec3D<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T: NumVec> Vec3D<T> where T: Float {
    pub fn magnitude(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn angle_between(&self, other: &Vec3D<T>) -> T {
        (self.dot(other) / (self.magnitude() * other.magnitude())).acos()
    }
}

impl<T: NumVec> Index<usize> for Vec3D<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds: {}", index),
        }
    }
}

impl<T: NumVec> Index<i32> for Vec3D<T> {
    type Output = T;

    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds: {}", index),
        }
    }
}

impl<T: NumVec> Default for Vec3D<T> {
    fn default() -> Self {
        Vec3D::new(T::zero(), T::zero(), T::zero())
    }
}

impl<T: NumVec> From<(T, T, T)> for Vec3D<T> {
    fn from(value: (T, T, T)) -> Self {
        Vec3D {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl<T: NumVec> From<[T; 3]> for Vec3D<T> {
    fn from(value: [T; 3]) -> Self {
        Vec3D {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl<T: NumVec> TryFrom<Vec<T>> for Vec3D<T> {
    type Error = alloc::string::String;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.len() == 3 {
            Ok(Vec3D {
                x: value[0],
                y: value[1],
                z: value[2],
            })
        } else {
            Err(alloc::format!("Expected Vec of length 3, got {}", value.len()))
        }
    }
}

impl<T: NumVec> From<Vec3D<T>> for (T,T,T) {
    fn from(value: Vec3D<T>) -> Self {
        (value.x, value.y, value.z)
    }
}

impl<T: NumVec> From<Vec3D<T>> for [T; 3] {
    fn from(value: Vec3D<T>) -> Self {
        [value.x, value.y, value.z]
    }
}

impl<T: NumVec> From<Vec3D<T>> for Vec<T> {
    fn from(value: Vec3D<T>) -> Self {
        vec![value.x, value.y, value.z]
    }
}

impl<T: NumVec> core::ops::Mul<Self> for Vec3D<T> {
    type Output = Vec3D<T>;

    /// Cross-Product of the two vectors
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3D {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<T: NumVec> core::ops::Mul<T> for Vec3D<T> {
    type Output = Vec3D<T>;

    /// Multiplies the Vector by the Scalar
    fn mul(self, rhs: T) -> Self::Output {
        Vec3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: NumVec> core::ops::Add for Vec3D<T> {
    type Output = Vec3D<T>;
    
    fn add(self, rhs: Self) -> Self::Output {
        Vec3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: NumVec> core::ops::Sub for Vec3D<T> {
    type Output = Vec3D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
