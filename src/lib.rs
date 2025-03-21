pub mod ray;
pub mod vec3;
use std::fmt::Write as FmtWrite;
use std::io::Write;

use ray::Ray;
use vec3::{Color, Point3, Vec3};

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

pub fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &r);
    match t > 0.0 {
        true => {
            let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
            return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
        }
        false => {
            let unit_direction = r.direction().unit_vector();
            let a = (unit_direction.y() + 1.0) * 0.5;
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - r.origin();
    let a = r.direction().dot(&r.direction());
    let b = -2.0 * r.direction().dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    match discriminant < 0.0 {
        true => return -1.0,
        false => return (-b - discriminant.sqrt()) / (2.0 * a),
    }
}
