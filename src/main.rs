use std::sync::Arc;

use raytracing::{camera::Camera, hittable_list::HittableList, sphere::Sphere, vec3::Point3};

#[allow(dead_code)]
fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1024;
    cam.samples_per_pixel = 100;
    cam.render(&world);
}
