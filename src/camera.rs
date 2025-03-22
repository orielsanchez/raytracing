use std::{f32::consts::PI, f64, io};

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    random_double,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
    write_color,
};

pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: u32,       // Rendered image width in pixel count
    pub samples_per_pixel: u32, // Count of random samples for each pixel
    pub max_depth: u32,         // Maximum number of ray bounces into scene
    pub vfov: f64,              // Vertical view angle (field of view)
    pub lookfrom: Point3,       // Point camera is looking from
    pub lookat: Point3,         // Point camera is looking at
    pub vup: Vec3,              // Camera-relative "up" direction
    pub defocus_angle: f64,     // Variation angle of rays through each pixel
    pub focus_dist: f64,        // Distance from camera lookfrom point to plane of perfect focus

    image_height: u32,        // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Point3,           // Camera center
    pixel00_loc: Point3,      // Location of pixel 0, 0
    pixel_delta_u: Vec3,      // Offset to pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
    u: Vec3,                  // Camera frame basis vector right
    v: Vec3,                  // Camera frame basis vector up
    w: Vec3,                  // Camera frame basis vector back
    defocus_disk_u: Vec3,     // Defocus disk horizontal radius
    defocus_disk_v: Vec3,     // Defocus disk vertical radius
}

impl Default for Camera {
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

    pub fn render<T: Hittable>(&mut self, world: &T) {
        Self::initialize(self);
        println!("P3\n {0} {1} \n255", self.image_width, self.image_height);

        for j in (0..self.image_height).rev() {
            eprintln!("\rScanlines remaining: {} ", j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
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

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
}

fn degrees_to_radians(vfov: f64) -> f64 {
    vfov * PI as f64 / 180.0
}
