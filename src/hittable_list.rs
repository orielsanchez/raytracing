//! Collection of hittable objects for the raytracer.
//!
//! This module provides a `HittableList` struct that manages a collection of
//! hittable objects in the scene. It implements the `Hittable` trait itself,
//! allowing it to be used as a container for other hittable objects while
//! maintaining the same interface.

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
};

/// A collection of hittable objects in the scene.
///
/// This struct maintains a list of hittable objects and provides methods
/// to add and remove objects. It implements the `Hittable` trait, allowing
/// it to be used as a container for other hittable objects while maintaining
/// the same interface.
pub struct HittableList {
    /// The list of hittable objects
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    /// Creates a new empty hittable list.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Adds a hittable object to the list.
    ///
    /// # Arguments
    ///
    /// * `object` - The hittable object to add
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    /// Removes all objects from the list.
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Default for HittableList {
    /// Creates a default empty hittable list.
    fn default() -> Self {
        HittableList::new()
    }
}

impl Hittable for HittableList {
    /// Determines if a ray intersects with any object in the list.
    ///
    /// This implementation finds the closest intersection among all objects
    /// in the list. It maintains the closest intersection found so far and
    /// updates the ray interval accordingly to ensure we only find the
    /// closest intersection.
    ///
    /// # Arguments
    ///
    /// * `r` - The ray to test for intersection
    /// * `ray_t` - The interval along the ray to check for intersection
    ///
    /// # Returns
    ///
    /// If there is an intersection with any object, returns a `HitRecord`
    /// containing the details of the closest intersection. Otherwise returns
    /// `None`.
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hit_record = None;
        for object in &self.objects {
            if let Some(rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }
}
