//! Material implementations for the raytracer.
//!
//! This module provides the `Material` trait and its implementations for
//! different types of materials:
//! - `Lambertian`: Diffuse materials that scatter light uniformly
//! - `Metal`: Reflective materials with optional fuzziness
//! - `Dielectric`: Transparent materials that refract light

use crate::{
    hittable::HitRecord,
    random_double,
    ray::Ray,
    vec3::{Color, Vec3},
};

/// A trait for materials that can scatter light.
///
/// This trait defines how materials interact with light rays in the scene.
/// When a ray hits a material, it can be scattered in a new direction with
/// some attenuation of its color.
pub trait Material: Send + Sync {
    /// Determines how a ray is scattered when it hits the material.
    ///
    /// # Arguments
    ///
    /// * `r_in` - The incoming ray
    /// * `rec` - The hit record containing information about the intersection
    /// * `attenuation` - The color attenuation of the scattered ray
    /// * `scattered` - The scattered ray
    ///
    /// # Returns
    ///
    /// True if the ray was scattered, false if it was absorbed
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

/// A diffuse material that scatters light uniformly.
///
/// Lambertian materials scatter incoming light in random directions
/// with a cosine distribution, which gives them a matte appearance.
pub struct Lambertian {
    /// The color reflectance of the material (0.0 to 1.0 for each component)
    albedo: Color,
}

impl Lambertian {
    /// Creates a new Lambertian material with the given albedo.
    ///
    /// # Arguments
    ///
    /// * `albedo` - The color reflectance of the material
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    /// Scatters the ray in a random direction with cosine distribution.
    ///
    /// The scattered direction is computed by adding a random unit vector
    /// to the surface normal. If the resulting direction is near zero,
    /// the normal is used instead to prevent numerical issues.
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

/// A reflective material that can have fuzzy reflections.
///
/// Metal materials reflect incoming rays with optional fuzziness
/// controlled by the fuzz parameter.
pub struct Metal {
    /// The color reflectance of the metal
    pub albedo: Color,
    /// The amount of fuzziness in reflections (0.0 to 1.0)
    fuzz: f64,
}

impl Metal {
    /// Creates a new metal material with the given albedo and fuzz.
    ///
    /// # Arguments
    ///
    /// * `albedo` - The color reflectance of the metal
    /// * `f` - The amount of fuzziness (clamped to \[0,1\])
    pub fn new(albedo: Color, f: f64) -> Self {
        Self {
            albedo,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    /// Reflects the ray with optional fuzziness.
    ///
    /// The ray is reflected about the surface normal, and then a random
    /// vector scaled by the fuzz factor is added to create fuzzy reflections.
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&r_in.direction().unit_vector(), &rec.normal);

        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_unit_vector());
        scattered.direction().dot(&rec.normal) > 0.0
    }
}

/// A transparent material that refracts light.
///
/// Dielectric materials (like glass) can both reflect and refract light
/// based on their refractive index and the angle of incidence.
pub struct Dielectric {
    /// The refractive index of the material
    refraction_index: f64,
}

impl Dielectric {
    /// Creates a new dielectric material with the given refractive index.
    ///
    /// # Arguments
    ///
    /// * `refraction_index` - The refractive index of the material
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    /// Calculates the reflectance using Schlick's approximation.
    ///
    /// # Arguments
    ///
    /// * `cosine` - The cosine of the angle of incidence
    /// * `refraction_index` - The refractive index ratio
    ///
    /// # Returns
    ///
    /// The probability of reflection
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    /// Handles both reflection and refraction of the ray.
    ///
    /// The material can either reflect or refract the ray based on:
    /// - The angle of incidence
    /// - The refractive indices of the materials
    /// - Fresnel reflection (using Schlick's approximation)
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = match rec.front_face {
            true => 1.0 / self.refraction_index,
            false => self.refraction_index,
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = f64::min(-unit_direction.dot(&rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let direction = match cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            true => Vec3::reflect(&unit_direction, &rec.normal),
            false => Vec3::refract(&unit_direction, &rec.normal, ri),
        };
        *scattered = Ray::new(rec.p, direction);

        true
    }
}
