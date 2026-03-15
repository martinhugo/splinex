//! Generic point trait definition and implementation of trait for main point structures
//! natively supported in this library.

use glam::{Vec2, Vec3, Vec4};

/// A type that can be used as a point in spline definition and computation.
///
/// Implementing this trait allows user-defined point types to participate
/// in spline computation, enabling support for 2D, 3D, or any
/// custom coordinate space.
pub trait Point{

    /// Returns the distance between `self` and `other`
    fn distance(&self, other: &Self) -> f32;

    /// Linearly interpolates between `self` and `other`
    ///
    /// Returns t*self + (1-t)*other, where `t = 0.0` yields
    /// `other` and `t = 1.0` yields self.
    ///
    /// # Panics
    ///
    /// Implementation may panic if `t` is outside `[0.0, 1.0]`.
    fn lerp(&self, other: &Self, t: f32) -> Self;
}

// `Splinex` internal data structures

/// Defines a two-dimensional points
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f32,
    pub y: f32
}

impl Point for Point2D {

    fn distance(&self, other: &Self) -> f32 {
        ((other.x - self.x).powi(2) + (other.y-self.y).powi(2)).sqrt()
    }

    fn lerp(&self, other: &Self, t: f32) -> Self {
        assert!((0.0..=1.0).contains(&t), "`t` must be in [0.0, 1.0], got {t}");

         Self {
            x: t*self.x + (1.0-t)*other.x,
            y: t*self.y + (1.0-t)*other.y,
        }
    }
}


/// Defines a three-dimensional points
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point for Point3D {

    fn distance(&self, other: &Self) -> f32 {
        (
            (other.x - self.x).powi(2)
            + (other.y-self.y).powi(2)
            + (other.z - self.z).powi(2)
        ).sqrt()
    }

    fn lerp(&self, other: &Self, t: f32) -> Self {
        assert!((0.0..=1.0).contains(&t), "`t` must be in [0.0, 1.0], got {t}");

        Self {
            x: t*self.x + (1.0-t)*other.x,
            y: t*self.y + (1.0-t)*other.y,
            z: t*self.z + (1.0-t)*other.z,
        }
    }
}

// Natively supported data structures as Point

// glam support
impl Point for Vec2{
    fn distance(&self, other: &Self) -> f32 {
        (*self - *other).length()
    }

    fn lerp(&self, other: &Self, t: f32) -> Self {
        assert!((0.0..=1.0).contains(&t), "`t` must be in [0.0, 1.0], got {t}");
        Self::lerp(*self, *other, t)
    }
}

impl Point for Vec3{
    fn distance(&self, other: &Self) -> f32 {
        (*self - *other).length()
    }

    fn lerp(&self, other: &Self, t: f32) -> Self {
        assert!((0.0..=1.0).contains(&t), "`t` must be in [0.0, 1.0], got {t}");
        Self::lerp(*self, *other, t)
    }
}

impl Point for Vec4{
    fn distance(&self, other: &Self) -> f32 {
        (*self - *other).length()
    }

    fn lerp(&self, other: &Self, t: f32) -> Self {
        assert!((0.0..=1.0).contains(&t), "`t` must be in [0.0, 1.0], got {t}");
        Self::lerp(*self, *other, t)
    }
}

