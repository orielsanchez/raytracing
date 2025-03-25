//! Camera implementation for the raytracer.
//!
//! This module provides a `Camera` struct that handles the generation of rays
//! for rendering the scene. It supports features like:
//! - Configurable field of view
//! - Depth of field
//! - Anti-aliasing through multiple samples per pixel
//! - Background color gradient
//! - Parallel rendering using rayon

use std::{f32::consts::PI, f64, io};

use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    random_double,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
    write_color,
};

/// A camera that generates rays for rendering the scene.
///
/// The camera is defined by its position, orientation, and various rendering
/// parameters. It uses a coordinate system with basis vectors u, v, w to
/// generate rays through a viewport and into the scene.
pub struct Camera {
    /// Ratio of image width over height
    pub aspect_ratio: f64,
    /// Rendered image width in pixel count
    pub image_width: u32,
    /// Count of random samples for each pixel
    pub samples_per_pixel: u32,
    /// Maximum number of ray bounces into scene
    pub max_depth: u32,
    /// Vertical view angle (field of view)
    pub vfov: f64,
    /// Point camera is looking from
    pub lookfrom: Point3,
    /// Point camera is looking at
    pub lookat: Point3,
    /// Camera-relative "up" direction
    pub vup: Vec3,
    /// Variation angle of rays through each pixel
    pub defocus_angle: f64,
    /// Distance from camera lookfrom point to plane of perfect focus
    pub focus_dist: f64,

    /// Rendered image height
    image_height: u32,
    /// Color scale factor for a sum of pixel samples
    pixel_samples_scale: f64,
    /// Camera center
    center: Point3,
    /// Location of pixel 0, 0
    pixel00_loc: Point3,
    /// Offset to pixel to the right
    pixel_delta_u: Vec3,
    /// Offset to pixel below
    pixel_delta_v: Vec3,
    /// Camera frame basis vector right
    u: Vec3,
    /// Camera frame basis vector up
    v: Vec3,
    /// Camera frame basis vector back
    w: Vec3,
    /// Defocus disk horizontal radius
    defocus_disk_u: Vec3,
    /// Defocus disk vertical radius
    defocus_disk_v: Vec3,
}

impl Default for Camera {
    /// Creates a default camera with standard parameters.
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Default::default(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            image_height: 100,
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
            pixel_samples_scale: Default::default(),
            u: Default::default(),
            v: Default::default(),
            w: Default::default(),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_u: Default::default(),
            defocus_disk_v: Default::default(),
        }
    }
}

impl Camera {
    /// Creates a new camera with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `aspect_ratio` - Ratio of image width over height
    /// * `image_width` - Rendered image width in pixels
    /// * `samples_per_pixel` - Number of random samples per pixel
    /// * `max_depth` - Maximum number of ray bounces
    /// * `vfov` - Vertical field of view in degrees
    /// * `lookfrom` - Camera position
    /// * `lookat` - Point camera is looking at
    /// * `vup` - Camera-relative up direction
    /// * `defocus_angle` - Variation angle of rays through each pixel
    /// * `focus_dist` - Distance to plane of perfect focus
    /// * `image_height` - Rendered image height in pixels
    /// * `pixel_samples_scale` - Color scale factor for pixel samples
    /// * `center` - Camera center point
    /// * `pixel00_loc` - Location of pixel 0, 0
    /// * `pixel_delta_u` - Offset to pixel to the right
    /// * `pixel_delta_v` - Offset to pixel below
    /// * `u` - Camera frame basis vector right
    /// * `v` - Camera frame basis vector up
    /// * `w` - Camera frame basis vector back
    /// * `defocus_disk_u` - Defocus disk horizontal radius
    /// * `defocus_disk_v` - Defocus disk vertical radius
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
        image_height: u32,
        pixel_samples_scale: f64,
        center: Point3,
        pixel00_loc: Point3,
        pixel_delta_u: Vec3,
        pixel_delta_v: Vec3,
        u: Vec3,
        v: Vec3,
        w: Vec3,
        defocus_disk_u: Vec3,
        defocus_disk_v: Vec3,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            image_height,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    /// Renders the scene to stdout in PPM format.
    ///
    /// This method performs the actual rendering of the scene, using
    /// parallel processing to generate the image. For each pixel, it:
    /// 1. Generates multiple random samples
    /// 2. Traces rays through the scene
    /// 3. Accumulates the color contributions
    /// 4. Applies gamma correction
    /// 5. Writes the result to stdout
    ///
    /// # Arguments
    ///
    /// * `world` - The scene to render
    pub fn render<T: Hittable>(&mut self, world: &T) {
        Self::initialize(self);
        println!("P3\n {0} {1} \n255", self.image_width, self.image_height);

        for j in (0..self.image_height).rev() {
            eprintln!("\rScanlines remaining: {} ", j);
            let pixel_colors: Vec<_> = (0..self.image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Color::default();
                    for _ in 0..self.samples_per_pixel {
                        let u = (i as f64) + random_double() / (self.image_width - 1) as f64;
                        let v = (j as f64) + random_double() / (self.image_height - 1) as f64;
                        let r = self.get_ray(u as u32, v as u32);
                        pixel_color += Self::ray_color(&r, self.max_depth, world);
                    }
                    pixel_color
                })
                .collect();

            for pixel_color in pixel_colors {
                write_color(&mut io::stdout(), &(self.pixel_samples_scale * pixel_color))
                    .expect("Error writing to output");
            }
        }
        eprintln!("\rDone.");
    }

    /// Initializes the camera's internal state.
    ///
    /// This method sets up the camera's coordinate system and calculates
    /// various parameters needed for ray generation, including:
    /// - Viewport dimensions
    /// - Camera basis vectors
    /// - Pixel deltas
    /// - Defocus disk parameters
    fn initialize(&mut self) {
        let candidate_image_height = self.image_width as f64 / self.aspect_ratio;
        self.image_height = match candidate_image_height < 1.0 {
            true => 1,
            false => candidate_image_height as u32,
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        // Determine viewport dimensions.
        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * self.v;

        // Calculuate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        // Calculate the camera foucs disk basis vectors
        let defocus_radius =
            self.focus_dist * f64::tan(degrees_to_radians(self.defocus_angle / 2.0));

        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    /// Generates a ray for a given pixel.
    ///
    /// This method constructs a ray from the camera through the specified
    /// pixel, taking into account depth of field if enabled.
    ///
    /// # Arguments
    ///
    /// * `i` - The pixel's x coordinate
    /// * `j` - The pixel's y coordinate
    ///
    /// # Returns
    ///
    /// A ray from the camera through the pixel
    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly sampled point around the pixel location i,j.

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = match self.defocus_angle <= 0.0 {
            true => self.center,
            false => self.defocus_disk_sample(),
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    /// Computes the color of a ray through the scene.
    ///
    /// This method recursively traces a ray through the scene, handling
    /// reflection, refraction, and background color. It implements the
    /// Monte Carlo path tracing algorithm.
    ///
    /// # Arguments
    ///
    /// * `r` - The ray to trace
    /// * `depth` - The current recursion depth
    /// * `world` - The scene to trace through
    ///
    /// # Returns
    ///
    /// The color contribution of the ray
    pub fn ray_color<T: Hittable>(r: &Ray, depth: u32, world: &T) -> Color {
        if depth == 0 {
            return Color::default();
        }
        let rec: Option<HitRecord> = world.hit(r, Interval::new(0.001, f64::INFINITY));
        match rec {
            Some(rec) => {
                let mut scattered = Ray::default();
                let mut attenuation = Color::default();

                match rec
                    .mat
                    .as_ref()
                    .unwrap()
                    .scatter(r, &rec, &mut attenuation, &mut scattered)
                {
                    true => attenuation * Self::ray_color(&scattered, depth - 1, world),
                    false => Color::default(),
                }
            }
            None => {
                let unit_direction = r.direction().unit_vector();
                let a = (unit_direction.y() + 1.0) * 0.5;
                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
            }
        }
    }

    /// Generates a random offset within a pixel.
    ///
    /// # Returns
    ///
    /// A random 2D offset in the range [-0.5, 0.5]
    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    /// Generates a random point in the camera's defocus disk.
    ///
    /// # Returns
    ///
    /// A random point within the defocus disk
    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
}

/// Converts degrees to radians.
///
/// # Arguments
///
/// * `vfov` - The angle in degrees
///
/// # Returns
///
/// The angle in radians
fn degrees_to_radians(vfov: f64) -> f64 {
    vfov * PI as f64 / 180.0
}
