use std::{f64, io};

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    random_double,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
    write_color,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: i32,
    pixel_samples_scale: f64,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            image_height: 100,
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
            pixel_samples_scale: Default::default(),
        }
    }
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: i32,
        image_height: u32,
        center: Point3,
        pixel00_loc: Point3,
        pixel_delta_u: Vec3,
        pixel_delta_v: Vec3,
        pixel_samples_scale: f64,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale,
        }
    }

    pub fn render<T: Hittable>(&mut self, world: &T) {
        Self::initialize(self);
        println!("P3\n {0} {1} \n255", self.image_width, self.image_height);

        for j in (0..self.image_height).rev() {
            eprintln!("\rScanlines remaining: {} ", j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, world);
                }

                write_color(&mut io::stdout(), &(self.pixel_samples_scale * pixel_color))
                    .expect("Error writing to output");
            }
        }
        eprintln!("\rDone.");
    }

    fn initialize(&mut self) {
        let candidate_image_height = self.image_width as f64 / self.aspect_ratio;
        self.image_height = match candidate_image_height < 1.0 {
            true => 1,
            false => candidate_image_height as u32,
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        self.center = Point3::default();

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, viewport_height, 0.0);

        // Calculuate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
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

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}
