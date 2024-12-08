#![no_std]
extern crate alloc;

use num_traits::{Num, NumCast};

pub mod vecs;

pub mod prelude {
    #[allow(unused_imports)]
    pub use crate::vecs::vec2d::*;
    #[allow(unused_imports)]
    pub use crate::vecs::vec3d::*;
    #[allow(unused_imports)]
    pub use crate::NumVec;
}

pub trait NumVec: Num + NumCast + Copy {}
impl<T: Num + NumCast + Copy> NumVec for T {}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use crate::prelude::*;

    #[test]
    fn vec2d_test() {
        {
            let vec1 = Vec2D::new(2.0, 3.0);
            let vec2 = Vec2D::from((2, 3));
            
            assert_eq!(vec1, vec2.cast().unwrap());
        }
        {
            let vec1: Vec2D<u8> = Vec2D::default();
            let vec2 = Vec2D::from((0u8, 0u8));
            
            assert_eq!(vec1, vec2);
        }
        {
            let vec1 = Vec2D::new(2u16, 3u16);
            let vec2 = Vec2D::from([4f64,5f64]);
            let result = vec1.cast().unwrap() + vec2;
            
            assert_eq!(result, Vec2D::new(6f64, 8f64));
        }
        {
            let vec = vec2d(4u16, 3u16);
            let scalar = 6u16;
            
            assert_eq!(vec * scalar, vec2d(24u16, 18u16));
        }
        {
            let vec = vec2d(4f32, 3f32);
            
            assert_eq!(vec.magnitude(), 5f32);
        }
        {
            let normal_vec1 = alloc::vec![3u16, 7u16];
            let normal_vec2 = alloc::vec::Vec::from(vec2d(3u16, 7u16));
            assert_eq!(normal_vec1, normal_vec2);
        }
        {
            let slice = [4f64, 8f64];
            let vec = Vec2D::from(slice);
            
            assert_eq!(vec, vec2d(4f64, 8f64));
        }
    }
    
    #[test]
    fn vec3d_test() {
        {
            let vec1 = Vec3D::new(2.0, 3.0, 5.0);
            let vec2 = Vec3D::from((2u128, 3u128, 5u128));
            
            assert_eq!(vec1, vec2.cast().unwrap());
        }
        {
            let vec1: Vec3D<u8> = Vec3D::default();
            let vec2 = Vec3D::from([0u8, 0u8, 0u8]);
            
            assert_eq!(vec1, vec2);
        }
        {
            let normal_vec = alloc::vec![4u16, 5u16, 3u16];
            
            let vec1 = vec3d(2i64, 3i64, 13i64);
            let vec2 = Vec3D::from(normal_vec);
            let result = vec1.cast().unwrap() + vec2;
            
            assert_eq!(result, vec3d(6u16, 8u16, 16u16));
        }
        {
            let vec =vec3d(2i64, 3i64, 13i64);
            let scalar = 5i64;
            
            assert_eq!(vec * scalar, vec3d(10i64, 15i64, 65i64));
        }
    }
}