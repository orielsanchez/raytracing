//! Sphere primitive for the raytracer.
//!
//! This module provides a `Sphere` struct that represents a sphere in 3D space.
//! It implements the `Hittable` trait, providing ray-sphere intersection
//! testing using the quadratic formula.

use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::{Material, Metal},
    vec3::{Color, Point3, Vec3},
};

/// A sphere in 3D space.
///
/// A sphere is defined by:
/// - A center point
/// - A radius
/// - A material that determines how it interacts with light
#[allow(dead_code)]
pub struct Sphere {
    /// The center point of the sphere
    center: Point3,
    /// The radius of the sphere
    radius: f64,
    /// The material of the sphere
    mat: Arc<dyn Material>,
}

impl Sphere {
    /// Creates a new sphere with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `center` - The center point of the sphere
    /// * `radius` - The radius of the sphere
    /// * `mat` - The material of the sphere
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }

    /// Returns the center point of the sphere
    pub fn center(&self) -> Vec3 {
        self.center
    }

    /// Returns the radius of the sphere
    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Default for Sphere {
    /// Creates a default sphere at the origin with radius 1.0 and a default metal material.
    fn default() -> Self {
        Self {
            center: Default::default(),
            radius: 1.0,
            mat: Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 1.0)),
        }
    }
}

impl Hittable for Sphere {
    /// Determines if a ray intersects with the sphere.
    ///
    /// This implementation uses the quadratic formula to find the intersection
    /// points between the ray and the sphere. It finds the nearest intersection
    /// point that lies within the given ray interval.
    ///
    /// # Arguments
    ///
    /// * `r` - The ray to test for intersection
    /// * `ray_t` - The interval along the ray to check for intersection
    ///
    /// # Returns
    ///
    /// If there is an intersection, returns a `HitRecord` containing the
    /// intersection details. Otherwise returns `None`.
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center() - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius() * self.radius();

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut hit_record = HitRecord {
            t: root,
            p: r.at(root),
            normal: Vec3::default(),
            front_face: false,
            mat: Some(self.mat.clone()),
        };

        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(r, &outward_normal);

        Some(hit_record)
    }
}
