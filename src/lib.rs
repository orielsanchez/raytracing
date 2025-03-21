pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod vec3;

use std::fmt::Write as FmtWrite;
use std::io::Write;

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
