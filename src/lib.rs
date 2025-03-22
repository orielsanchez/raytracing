pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

use std::fmt::Write as FmtWrite;
use std::io::Write;

use interval::Interval;
use rand::Rng;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    match linear_component > 0.0 {
        true => linear_component.sqrt(),
        false => 0.0,
    }
}

pub fn write_color<T: Write>(
    out: &mut T,
    pixel_color: &vec3::Color,
) -> Result<usize, std::io::Error> {
    let mut str = String::new();

    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Apply linear to gamma transform for gamma 2
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // translate the [0,1] component values to the byte range [0, 255].
    let intensity = Interval::new(0.000, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as i32;
    let gbyte = (256.0 * intensity.clamp(g)) as i32;
    let bbyte = (256.0 * intensity.clamp(b)) as i32;

    // Write out the pixel color components.
    writeln!(str, "{} {} {}", rbyte, gbyte, bbyte).expect("Error formatting write");
    out.write(str.as_bytes())
}

pub fn random_double() -> f64 {
    rand::rng().random()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
