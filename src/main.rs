use std::io;

use raytracing::{vec3::Color, write_color};




#[allow(dead_code)]
fn main() {
    // Image
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // Render
    println!("P3\n {IMAGE_WIDTH} {IMAGE_HEIGHT} \n255");

    for j in 0..IMAGE_HEIGHT {
        eprintln!("\rScanlines remaining: {} ", (IMAGE_HEIGHT - j));
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.0,
            );
            write_color(&mut io::stdout(), &pixel_color).expect("Error writing to output");
        }
    }
    eprintln!("\rDone.");
}
