fn main() {
    // Image
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // Render
    println!("P3\n {IMAGE_WIDTH} {IMAGE_HEIGHT} \n255");

    for j in 0..IMAGE_HEIGHT {
        eprintln!("\rScanlines remaining: {} ", (IMAGE_HEIGHT - j));
        for i in 0.. IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{ir} {ig} {ib}");

        }
    }
    eprintln!("\rDone.");
}
