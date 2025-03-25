//! Hit record and hittable trait for the raytracer.
//!
//! This module provides the fundamental abstractions for ray-object intersection
//! in the raytracer. It defines the `HitRecord` struct that stores information
//! about a ray-object intersection, and the `Hittable` trait that all
//! intersectable objects must implement.

use std::sync::Arc;

use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

/// A record of a ray-object intersection.
///
/// This struct stores all the information about where and how a ray
/// intersects with an object in the scene, including:
/// - The point of intersection
/// - The surface normal at the intersection
/// - The material of the intersected object
/// - The distance along the ray to the intersection
/// - Whether the ray hit the front or back face of the object
#[allow(dead_code)]
#[derive(Default)]
pub struct HitRecord {
    /// The point where the ray intersects the object
    pub p: Point3,
    /// The surface normal at the point of intersection
    pub normal: Vec3,
    /// The material of the intersected object
    pub mat: Option<Arc<dyn Material>>,
    /// The distance along the ray to the intersection point
    pub t: f64,
    /// Whether the ray hit the front face of the object
    pub front_face: bool,
}

impl HitRecord {
    /// Creates a new hit record with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `p` - The point of intersection
    /// * `normal` - The surface normal at the intersection
    /// * `mat` - The material of the intersected object
    /// * `t` - The distance along the ray to the intersection
    /// * `front_face` - Whether the ray hit the front face
    pub fn new(p: Point3, normal: Vec3, mat: Arc<dyn Material>, t: f64, front_face: bool) -> Self {
        Self {
            p,
            normal,
            mat: Some(mat),
            t,
            front_face,
        }
    }

    /// Sets the normal vector based on the ray direction and outward normal.
    ///
    /// This method determines whether the ray hit the front or back face
    /// of the object and sets the normal vector accordingly. The normal
    /// will always point against the ray direction.
    ///
    /// # Arguments
    ///
    /// * `r` - The ray that hit the object
    /// * `outward_normal` - The outward-facing normal of the object (assumed to be unit length)
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector
        // the parameter `outward_normal` is assumed to have unit length.

        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -(*outward_normal),
        }
    }
}

/// A trait for objects that can be intersected by rays.
///
/// This trait must be implemented by any object that can be intersected
/// by rays in the scene. It provides a single method `hit` that determines
/// if a ray intersects with the object within a given interval.
pub trait Hittable: Send + Sync {
    /// Determines if a ray intersects with the object.
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
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
