pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod vec3;

use std::fmt::Write as FmtWrite;
use std::io::Write;

use interval::Interval;
use rand::Rng;

pub fn write_color<T: Write>(
    out: &mut T,
    pixel_color: &vec3::Color,
) -> Result<usize, std::io::Error> {
    let mut str = String::new();

    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let intensity = Interval::new(0.000, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as i32;
    let gbyte = (256.0 * intensity.clamp(g)) as i32;
    let bbyte = (256.0 * intensity.clamp(b)) as i32;

    writeln!(str, "{} {} {}", rbyte, gbyte, bbyte).expect("Error formatting write");

    out.write(str.as_bytes())
}

pub fn random_double() -> f64 {
    rand::rng().random()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
