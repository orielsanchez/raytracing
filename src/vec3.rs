//! Three-dimensional vector implementation for the raytracer.
//!
//! This module provides a `Vec3` struct that serves as the foundation for
//! 3D vectors, points, and colors in the raytracer. It implements common
//! vector operations and provides utility functions for random vector generation
//! and geometric calculations.

use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use crate::random_double;
use crate::random_double_range;

/// Type alias for using Vec3 as a point in 3D space
pub type Point3 = Vec3;
/// Type alias for using Vec3 as an RGB color
pub type Color = Vec3;

/// A three-dimensional vector with double-precision components.
///
/// This struct is used throughout the raytracer for representing:
/// - 3D vectors
/// - Points in 3D space
/// - RGB colors
///
/// The components are stored in a fixed-size array for efficient access
/// and SIMD-friendly operations.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    /// The x, y, z components of the vector
    e: [f64; 3],
}

impl Vec3 {
    /// Creates a new vector with the given components.
    ///
    /// # Arguments
    ///
    /// * `e0` - The x component
    /// * `e1` - The y component
    /// * `e2` - The z component
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    /// Returns the x component of the vector
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    /// Returns the y component of the vector
    pub fn y(&self) -> f64 {
        self.e[1]
    }

    /// Returns the z component of the vector
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    /// Computes the dot product of two vectors
    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
    }

    /// Computes the cross product of two vectors
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    /// Returns a unit vector in the same direction as this vector
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    /// Returns the squared length of the vector
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    /// Checks if the vector is close to zero
    ///
    /// Returns true if all components are less than 1e-8 in absolute value
    pub fn near_zero(&self) -> bool {
        let s = 1.0e-8;
        (f64::abs(self.e[0]) < s) && (f64::abs(self.e[1]) < s) && (f64::abs(self.e[2]) < s)
    }

    /// Generates a random vector with components in [0,1)
    pub fn random_vec() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }

    /// Generates a random vector with components in [min,max)
    pub fn random_vec_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }

    /// Generates a random unit vector (uniformly distributed on unit sphere)
    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_vec_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1.0e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    /// Generates a random vector in the unit disk (x,y plane)
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                random_double_range(-1.0, 1.0),
                random_double_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Generates a random vector on the hemisphere defined by the normal
    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        match on_unit_sphere.dot(normal) > 0.0 {
            true => on_unit_sphere,
            false => -on_unit_sphere,
        }
    }

    /// Reflects a vector about a normal vector
    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        (*v) - 2.0 * v.dot(n) * (*n)
    }

    /// Refracts a vector through a surface with given refractive index ratio
    ///
    /// # Arguments
    ///
    /// * `uv` - The incoming unit vector
    /// * `n` - The surface normal
    /// * `etai_over_etat` - The ratio of refractive indices (n1/n2)
    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(-uv.dot(n), 1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel = -f64::abs(1.0 - r_out_perp.length_squared()).sqrt() * *n;
        r_out_perp + r_out_parallel
    }

    /// Returns the length (magnitude) of the vector
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}
