use num_complex::Complex;
use rayon::prelude::*;
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone)]
pub struct Image {
    pixels: Arc<Vec<Pixel>>,
    width: usize,
    height: usize,
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
            r: ct as u8 * 21,
            g: ct as u8 * 34,
            b: ct as u8 * 45,
        }
    }
}

pub fn created_by_ai_assistant_rayon(width: usize, height: usize, limit: u32) -> Image {
    let scalex = 3.0 / width as f64;
    let scaley = 3.0 / height as f64;

    let start = Instant::now();

    let raw_pixels: Vec<_> = (0..width * height)
        .into_par_iter()
        .map(|i| {
            let y = i / width;
            let x = i % width;
            let cx = x as f64 * scalex - 2.0;
            let cy = y as f64 * scaley - 1.5;
            mandelbrot_set(Complex::new(cx, cy), limit)
        })
        .collect();
    let duration = start.elapsed();
    println!("Time elapsed in fractal creation is: {:?}", duration);

    Image {
        pixels: Arc::new(raw_pixels),
        width,
        height,
    }
}
