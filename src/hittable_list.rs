use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};

#[allow(dead_code)]
struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

#[allow(dead_code)]
impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut closest_so_far = ray_tmax;
        let mut hit_record = None;
        for object in &self.objects {
            if let Some(rec) = object.hit(r, ray_tmin, closest_so_far) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }
}
