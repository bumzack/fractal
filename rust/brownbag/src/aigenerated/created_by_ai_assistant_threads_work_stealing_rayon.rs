use std::time::Instant;

use num_complex::Complex;
use rayon::prelude::*;

#[derive(Clone, Copy)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

fn mandelbrot_set(c: Complex<f64>, limit: u32) -> Pixel {
    let mut z = Complex::new(0.0, 0.0);
    let mut ct = 0;

    while ct < limit && z.norm_sqr() <= 4.0 {
        z = z * z + c;
        ct += 1;
    }

    if ct == limit {
        Pixel { r: 0, g: 0, b: 0 }
    } else {
        Pixel {
            r: (ct * 2) as u8,
            g: (ct * 3) as u8,
            b: (ct * 5) as u8,
        }
    }
}

fn create_fractal(width: usize, height: usize, limit: u32) -> Vec<Pixel> {
    let scalex = 3.0 / width as f64;
    let scaley = 3.0 / height as f64;

    (0..width * height)
        .into_par_iter()
        .map(|i| {
            let y = i / width;
            let x = i % width;
            let cx = x as f64 * scalex - 2.0;
            let cy = y as f64 * scaley - 1.5;
            mandelbrot_set(Complex::new(cx, cy), limit)
        })
        .collect()
}

pub fn created_by_ai_assistant_threads_work_stealing_rayon() {
    let start = Instant::now();
    let _fractal = create_fractal(800, 600, 100);
    let duration = start.elapsed();
    println!("Time elapsed in fractal creation is: {:?}", duration);
}
