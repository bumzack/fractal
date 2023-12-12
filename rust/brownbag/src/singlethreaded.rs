use std::time::Instant;

use crate::fractal::{BLACK, calc_color, FractalImage};
use crate::utils::write_to_ppm;

pub fn single_threaded(width: usize, height: usize, max_iterations: usize) {
    let start = Instant::now();

    let mut pixels = vec![BLACK; width * height];

    let mut y = 0;
    while y < height {
        for x in 0..width {
            let idx = y * width + x;
            let color = calc_color(x, y, width, height, max_iterations);
            pixels[idx].r = color.r;
            pixels[idx].g = color.g;
            pixels[idx].b = color.b;
        }
        y += 1;
    }

    let fractal_image = FractalImage {
        width,
        height,
        pixels,
    };

    let duration = start.elapsed();

    println!("single threaded duration: {} ms", duration.as_millis());
    // println!("fractal image {}", fractal_image);

    write_to_ppm(
        &fractal_image,
        &format!(
            "single_threaded_{}x{}_max_iter_{}",
            width, height, max_iterations
        ),
    );
}
