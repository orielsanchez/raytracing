use std::fmt::Write as FmtWrite;
use std::io::Write;

use vec3::Vec3;

mod vec3;
type Color = Vec3;
pub fn write_color<T: Write>(out: &mut T, pixel_color: &Color) -> Result<usize, std::io::Error> {
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
