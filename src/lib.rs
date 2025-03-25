//! A physically-based raytracer implemented in Rust.
//!
//! This crate implements a Monte Carlo raytracer that supports:
//! - Diffuse materials (Lambertian)
//! - Metal materials with configurable fuzz
//! - Dielectric materials (glass)
//! - Camera with depth of field
//! - Anti-aliasing
//! - Gamma correction
//!
//! The raytracer follows physically-based rendering principles and uses Monte Carlo
//! integration for accurate light transport simulation.

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

/// Converts a linear color component to gamma space (gamma 2).
///
/// This function applies the square root transformation to convert from linear
/// color space to gamma space, which is more perceptually uniform.
///
/// # Arguments
///
/// * `linear_component` - A color component in linear space (0.0 to 1.0)
///
/// # Returns
///
/// The color component in gamma space. Returns 0.0 for negative values.
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    match linear_component > 0.0 {
        true => linear_component.sqrt(),
        false => 0.0,
    }
}

/// Writes a color to an output stream in PPM format.
///
/// This function converts a color from linear space to gamma space and writes
/// it as RGB values in the range [0, 255] to the specified output stream.
///
/// # Arguments
///
/// * `out` - The output stream to write to
/// * `pixel_color` - The color to write
///
/// # Returns
///
/// The number of bytes written or an error if writing fails
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

/// Generates a random double-precision float in the range [0, 1).
///
/// # Returns
///
/// A random float between 0.0 (inclusive) and 1.0 (exclusive)
pub fn random_double() -> f64 {
    rand::rng().random()
}

/// Generates a random double-precision float in the specified range.
///
/// # Arguments
///
/// * `min` - The minimum value (inclusive)
/// * `max` - The maximum value (exclusive)
///
/// # Returns
///
/// A random float between min (inclusive) and max (exclusive)
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
