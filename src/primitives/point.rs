//! Generic point trait definition and implementation of trait for main point structures
//! natively supported in this library.

use glam::{Vec2, Vec3, Vec4};
use nalgebra::{Vector2, Vector3, Vector4};

/// A type that can be used as a point in spline definition and computation.
///
/// Implementing this trait allows user-defined point types to participate
/// in spline computation, enabling support for 2D, 3D, or any
/// custom coordinate space.
pub trait Point {
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

// `splinex` internal data structures

/// Defines a two-dimensional points
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point for Point2D {
    fn distance(&self, other: &Self) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }

    fn lerp(&self, other: &Self, t: f32) -> Self {
        assert!(
            (0.0..=1.0).contains(&t),
            "`t` must be in [0.0, 1.0], got {t}"
        );

        Self {
            x: t * self.x + (1.0 - t) * other.x,
            y: t * self.y + (1.0 - t) * other.y,
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
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2) + (other.z - self.z).powi(2))
            .sqrt()
    }

    fn lerp(&self, other: &Self, t: f32) -> Self {
        assert!(
            (0.0..=1.0).contains(&t),
            "`t` must be in [0.0, 1.0], got {t}"
        );

        Self {
            x: t * self.x + (1.0 - t) * other.x,
            y: t * self.y + (1.0 - t) * other.y,
            z: t * self.z + (1.0 - t) * other.z,
        }
    }
}

// Natively supported data structures as Point

// glam support
impl Point for Vec2 {
    fn distance(&self, other: &Self) -> f32 {
        (*self - *other).length()
    }

    fn lerp(&self, other: &Self, t: f32) -> Self {
        assert!(
            (0.0..=1.0).contains(&t),
            "`t` must be in [0.0, 1.0], got {t}"
        );
        Self::lerp(*self, *other, t)
    }
}

impl Point for Vec3 {
    fn distance(&self, other: &Self) -> f32 {
        (*self - *other).length()
    }

    fn lerp(&self, other: &Self, t: f32) -> Self {
        assert!(
            (0.0..=1.0).contains(&t),
            "`t` must be in [0.0, 1.0], got {t}"
        );
        Self::lerp(*self, *other, t)
    }
}

impl Point for Vec4 {
    fn distance(&self, other: &Self) -> f32 {
        (*self - *other).length()
    }

    fn lerp(&self, other: &Self, t: f32) -> Self {
        assert!(
            (0.0..=1.0).contains(&t),
            "`t` must be in [0.0, 1.0], got {t}"
        );
        Self::lerp(*self, *other, t)
    }
}

// nalgebra support

impl Point for Vector2<f32> {
    fn distance(&self, other: &Self) -> f32 {
        self.metric_distance(other)
    }
    fn lerp(&self, other: &Self, t: f32) -> Self {
        assert!(
            (0.0..=1.0).contains(&t),
            "`t` must be in [0.0, 1.0], got {t}"
        );
        self.lerp(other, t)
    }
}

impl Point for Vector3<f32> {
    fn distance(&self, other: &Self) -> f32 {
        self.metric_distance(other)
    }
    fn lerp(&self, other: &Self, t: f32) -> Self {
        assert!(
            (0.0..=1.0).contains(&t),
            "`t` must be in [0.0, 1.0], got {t}"
        );
        self.lerp(other, t)
    }
}

impl Point for Vector4<f32> {
    fn distance(&self, other: &Self) -> f32 {
        self.metric_distance(other)
    }
    fn lerp(&self, other: &Self, t: f32) -> Self {
        assert!(
            (0.0..=1.0).contains(&t),
            "`t` must be in [0.0, 1.0], got {t}"
        );
        self.lerp(other, t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Asserts that a Point2D can be created
    #[test]
    fn test_point_2d_creation() {
        let tested_point = Point2D { x: 1.0, y: 2.0 };

        assert_eq!(tested_point.x, 1.0);
        assert_eq!(tested_point.y, 2.0);
    }

    /// Asserts that a Point2D can be modified
    #[test]
    fn test_point_2d_mutation() {
        let mut tested_point = Point2D { x: 1.0, y: 2.0 };

        tested_point.x = 3.0;
        assert_eq!(tested_point.x, 3.0);
    }

    /// Asserts that distance between two Point2D can be computed
    #[test]
    fn test_point_2d_distance() {
        let a = Point2D { x: 1.0, y: 2.0 };
        let b = Point2D { x: 4.0, y: 6.0 };

        let distance_from_a = a.distance(&b);
        let distance_from_b = b.distance(&a);

        // tests euclidean distance symmetry
        assert_eq!(distance_from_a, distance_from_b);
        assert_eq!(distance_from_a, 5.0);
    }

    /// Asserts that valid linear interpolations between two Point2D can be computed
    #[test]
    fn test_point_2d_lerp() {
        let a = Point2D { x: 1.0, y: 2.0 };
        let b = Point2D { x: 4.0, y: 6.0 };

        assert_eq!(a.lerp(&b, 1.0), Point2D { x: 1.0, y: 2.0 });
        assert_eq!(a.lerp(&b, 0.0), Point2D { x: 4.0, y: 6.0 });
        assert_eq!(a.lerp(&b, 0.5), Point2D { x: 2.5, y: 4.0 });
    }

    /// Asserts that lerp panic when given a wrong input parameter for point 2D
    #[test]
    #[should_panic(expected = "`t` must be in [0.0, 1.0], got 1.1")]
    fn test_point_2d_lerp_panic() {
        let a = Point2D { x: 1.0, y: 2.0 };
        let b = Point2D { x: 4.0, y: 6.0 };

        a.lerp(&b, 1.1);
    }

    /// Asserts that a Point3D can be created
    #[test]
    fn test_point_3d_creation() {
        let tested_point = Point3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(tested_point.x, 1.0);
        assert_eq!(tested_point.y, 2.0);
        assert_eq!(tested_point.z, 3.0);
    }

    /// Asserts that a Point3D can be modified
    #[test]
    fn test_point_3d_mutation() {
        let mut tested_point = Point3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        tested_point.z = 4.0;
        assert_eq!(tested_point.z, 4.0);
    }

    /// Asserts that distance between two Point3D can be computed
    #[test]
    fn test_point_3d_distance() {
        let a = Point3D {
            x: 4.0,
            y: 1.0,
            z: 3.0,
        };
        let b = Point3D {
            x: 6.0,
            y: 4.0,
            z: 9.0,
        };

        let distance_from_a = a.distance(&b);
        let distance_from_b = b.distance(&a);

        // tests euclidean distance symmetry
        assert_eq!(distance_from_a, distance_from_b);
        assert_eq!(distance_from_a, 7.0);
    }

    /// Asserts that valid linear interpolations between two Point3D can be computed
    #[test]
    fn test_point_3d_lerp() {
        let a = Point3D {
            x: 4.0,
            y: 1.0,
            z: 3.0,
        };
        let b = Point3D {
            x: 6.0,
            y: 4.0,
            z: 9.0,
        };

        assert_eq!(
            a.lerp(&b, 1.0),
            Point3D {
                x: 4.0,
                y: 1.0,
                z: 3.0
            }
        );
        assert_eq!(
            a.lerp(&b, 0.0),
            Point3D {
                x: 6.0,
                y: 4.0,
                z: 9.0
            }
        );
        assert_eq!(
            a.lerp(&b, 0.5),
            Point3D {
                x: 5.0,
                y: 2.5,
                z: 6.0
            }
        );
    }

    /// Asserts that lerp panic when given a wrong input parameter for Point3D
    #[test]
    #[should_panic(expected = "`t` must be in [0.0, 1.0], got 1.1")]
    fn test_point_3d_lerp_panic() {
        let a = Point3D {
            x: 4.0,
            y: 1.0,
            z: 3.0,
        };
        let b = Point3D {
            x: 6.0,
            y: 4.0,
            z: 9.0,
        };

        a.lerp(&b, 1.1);
    }

    /// Asserts that glam::Vec2 is supported as a Point
    #[test]
    fn test_basic_vec2d_glam_support() {
        let a = Vec2 { x: 1.0, y: 2.0 };
        let b = Vec2 { x: 4.0, y: 6.0 };

        assert_eq!(<Vec2 as Point>::distance(&a, &b), 5.0);
    }

    /// Asserts that glam::Vec2 panics when a wrong parameter is given to lerp
    #[test]
    #[should_panic(expected = "`t` must be in [0.0, 1.0], got 1.1")]
    fn test_basic_vec2d_glam_lerp_panic() {
        let a = Vec2 { x: 1.0, y: 2.0 };
        let b = Vec2 { x: 4.0, y: 6.0 };

        <Vec2 as Point>::lerp(&a, &b, 1.1);
    }

    /// Asserts that glam::Vec2 is supported as a Point
    #[test]
    fn test_basic_vec3d_glam_support() {
        let a = Vec3 {
            x: 4.0,
            y: 1.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 6.0,
            y: 4.0,
            z: 9.0,
        };

        assert_eq!(<Vec3 as Point>::distance(&a, &b), 7.0);
    }

    #[test]
    #[should_panic(expected = "`t` must be in [0.0, 1.0], got 1.1")]
    fn test_basic_vec3d_glam_lerp_panic() {
        let a = Vec3 {
            x: 4.0,
            y: 1.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 6.0,
            y: 4.0,
            z: 9.0,
        };

        <Vec3 as Point>::lerp(&a, &b, 1.1);
    }
}
