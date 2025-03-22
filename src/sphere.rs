use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::{Material, Metal},
    vec3::{Color, Point3, Vec3},
};

#[allow(dead_code)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Default::default(),
            radius: 1.0,
            mat: Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 1.0)),
        }
    }
}

impl Hittable for Sphere {
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
