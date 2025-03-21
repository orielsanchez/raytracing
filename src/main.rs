use std::{io, sync::Arc};

use raytracing::{
    hittable_list::HittableList,
    ray::Ray,
    ray_color,
    sphere::Sphere,
    vec3::{Point3, Vec3},
    write_color,
};

#[allow(dead_code)]
fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1024;

    const CANDIDATE_IMAGE_HEIGHT: f64 = IMAGE_WIDTH as f64 / ASPECT_RATIO;
    const IMAGE_HEIGHT: u32 = match CANDIDATE_IMAGE_HEIGHT < 1.0 {
        true => 1,
        false => CANDIDATE_IMAGE_HEIGHT as u32,
    };

    // World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
    let camera_center = Point3::default();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, viewport_height, 0.0);

    // Calculuate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    println!("P3\n {IMAGE_WIDTH} {IMAGE_HEIGHT} \n255");

    for j in 0..IMAGE_HEIGHT {
        eprintln!("\rScanlines remaining: {} ", (IMAGE_HEIGHT - j));
        for i in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc
                + (pixel_delta_u * i as f64)
                + (pixel_delta_v * (IMAGE_HEIGHT - j - 1) as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r, &world);
            write_color(&mut io::stdout(), &pixel_color).expect("Error writing to output");
        }
    }
    eprintln!("\rDone.");
}
