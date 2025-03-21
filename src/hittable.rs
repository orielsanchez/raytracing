use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
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

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
