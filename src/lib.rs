pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod vec3;

use core::f64;
use std::fmt::Write as FmtWrite;
use std::io::Write;

use hittable::{HitRecord, Hittable};
use interval::Interval;
use ray::Ray;
use vec3::Color;

pub fn write_color<T: Write>(
    out: &mut T,
    pixel_color: &vec3::Color,
) -> Result<usize, std::io::Error> {
    let mut str = String::new();

    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;

    writeln!(str, "{} {} {}", rbyte, gbyte, bbyte).expect("Error formatting write");

    out.write(str.as_bytes())
}

pub fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
    let t: Option<HitRecord> = world.hit(r, Interval::new(0.0, f64::INFINITY));
    match t {
        Some(t) => 0.5 * (t.normal + Color::new(1.0, 1.0, 1.0)),
        None => {
            let unit_direction = r.direction().unit_vector();
            let a = (unit_direction.y() + 1.0) * 0.5;
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }
}
