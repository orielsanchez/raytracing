//! Ray representation for the raytracer.
//!
//! This module provides a `Ray` struct that represents a ray in 3D space,
//! defined by its origin point and direction vector. Rays are the fundamental
//! primitive used for ray tracing, representing the path of light through
//! the scene.

use crate::vec3::Point3;
use crate::vec3::Vec3;

/// A ray in 3D space.
///
/// A ray is defined by:
/// - An origin point where the ray starts
/// - A direction vector indicating the ray's direction
///
/// The ray can be parameterized by a distance t, where any point on the ray
/// can be expressed as: origin + direction * t
#[derive(Debug, Copy, Clone, Default)]
pub struct Ray {
    /// The starting point of the ray
    origin: Point3,
    /// The direction vector of the ray (should be normalized)
    direction: Vec3,
}

impl Ray {
    /// Creates a new ray with the given origin and direction.
    ///
    /// # Arguments
    ///
    /// * `origin` - The starting point of the ray
    /// * `direction` - The direction vector of the ray
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Returns the point at distance t along the ray.
    ///
    /// # Arguments
    ///
    /// * `t` - The distance along the ray
    ///
    /// # Returns
    ///
    /// The point at distance t from the ray's origin
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    /// Returns the origin point of the ray
    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    /// Returns the direction vector of the ray
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}
