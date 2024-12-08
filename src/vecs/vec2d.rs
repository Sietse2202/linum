use crate::NumVec;
use alloc::vec;
use alloc::vec::Vec;
use num_traits::Float;

/// Basic 2D Vector
/// 
/// # Examples
/// ```Rust
/// use compass::prelude::*
/// fn main() {
///     
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2D<T: NumVec> {
    pub x: T,
    pub y: T,
}

pub fn vec2d<T: NumVec>(x: T, y: T) -> Vec2D<T> {
    Vec2D { x, y }
}

impl<T: NumVec> Vec2D<T> {
    pub fn new(x: T, y: T) -> Vec2D<T> {
        Vec2D { x, y }
    }

    pub fn cast<U: NumVec>(self) -> Option<Vec2D<U>> {
        Some(
            Vec2D {
                x: U::from(self.x)?,
                y: U::from(self.y)?,
            }
        )
    }

    /// Due to type casting to f64, using this isn't advised, please use the `magnitude` method instead.
    pub fn magnitude_cast(&self) -> f64 {
        let x: f64 = num_traits::NumCast::from(self.x).unwrap_or(0.0);
        let y: f64 = num_traits::NumCast::from(self.y).unwrap_or(0.0);
        (x * x + y * y).sqrt()
    }

    pub fn normalize(&self) -> Option<Vec2D<T>> where T: Float {
        let mag = self.magnitude();
        if mag == T::zero() {
            None
        } else {
            Some(Vec2D {
                x: self.x / mag,
                y: self.y / mag,
            })
        }
    }

    pub fn dot(&self, other: &Vec2D<T>) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn angle_between(&self, other: &Vec2D<T>) -> T where T: Float {
        (self.dot(other) / (self.magnitude() * other.magnitude())).acos()
    }

    pub fn is_zero(&self) -> bool {
        self.x == T::zero() && self.y == T::zero()
    }

    pub fn pseudo_cross(&self, other: &Vec2D<T>) -> T {
        self.x * other.y - self.y * other.x
    }

    pub fn lerp(&self, other: &Vec2D<T>, t: f64) -> Vec2D<T> where T: Float {
        let t = T::from(t.clamp(0.0, 1.0)).unwrap_or(T::zero());
        Vec2D {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }
}

impl<T: NumVec> Vec2D<T> where T: Float {
    /// Returns the length of the given vector, only for `Vec<T> where T: Float`
    pub fn magnitude(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    /// Returns a new vector that is rotated by `angle` degrees
    pub fn rotate_deg(&self, angle: T) -> Vec2D<T> {
        let angle = angle.to_radians();
        self.rotate_rad(angle)
    }

    /// Rotates the given vector in place by `angle` degrees
    pub fn rotate_deg_in_place(&mut self, angle: T) {
        let angle = angle.to_radians();
        self.rotate_rad_in_place(angle);
    }
    
    /// Returns a new vector rotated by `radians` radians
    pub fn rotate_rad(&self, radians: T) -> Vec2D<T> {
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();
        Vec2D {
            x: self.x * cos_theta - self.y * sin_theta,
            y: self.x * sin_theta + self.y * cos_theta,
        }
    }
    
    pub fn rotate_rad_in_place(&mut self, radians: T) {
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();
        let new_x = self.x * cos_theta - self.y * sin_theta;
        let new_y = self.x * sin_theta + self.y * cos_theta;
        self.x = new_x;
        self.y = new_y;
    }
}

impl<T: NumVec> Default for Vec2D<T> {
    fn default() -> Self {
        Vec2D::new(T::zero(), T::zero())
    }
}

impl<T: NumVec> From<(T,T)> for Vec2D<T> {
    fn from(value: (T, T)) -> Self {
        Vec2D {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T: NumVec> From<[T; 2]> for Vec2D<T> {
    fn from(value: [T; 2]) -> Self {
        Vec2D {
            x: value[0],
            y: value[1],
        }
    }
}

impl<T: NumVec> From<&[T; 2]> for Vec2D<T> {
    fn from(value: &[T; 2]) -> Self {
        Vec2D {
            x: value[0],
            y: value[1],
        }
    }
}

impl<T: NumVec> TryFrom<Vec<T>> for Vec2D<T> {
    type Error = alloc::string::String;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.len() == 2 {
            Ok(Vec2D {
                x: value[0],
                y: value[1],
            })
        } else {
            Err(alloc::format!("Expected Vec of length 2, got {}", value.len()))
        }
    }
}


impl<T: NumVec> From<Vec2D<T>> for (T,T) {
    fn from(value: Vec2D<T>) -> Self {
        (value.x, value.y)
    }
}

impl<T: NumVec> From<Vec2D<T>> for [T; 2] {
    fn from(value: Vec2D<T>) -> Self {
        [value.x, value.y]
    }
}

impl<T: NumVec> From<Vec2D<T>> for Vec<T> {
    fn from(value: Vec2D<T>) -> Self {
        vec![value.x, value.y]
    }
}

impl<T: NumVec> core::ops::Mul<T> for Vec2D<T> {
    type Output = Vec2D<T>;

    /// This returns the vector multiplied by the given scalar
    fn mul(self, rhs: T) -> Self::Output {
        Vec2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: NumVec> core::ops::Div<T> for Vec2D<T> {
    type Output = Vec2D<T>;

    /// This returns the vector divided by the given scalar
    fn div(self, rhs: T) -> Self::Output {
        Vec2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: NumVec> core::ops::Add for Vec2D<T> {
    type Output = Vec2D<T>;

    /// Adds the two vectors and returns the sum
    fn add(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: NumVec> core::ops::Sub for Vec2D<T> {
    type Output = Vec2D<T>;

    /// Subtracts the two vectors and returns the result
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
