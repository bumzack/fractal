use crate::color::Color;
use crate::complex::ComplexNumber;
use chrono::Utc;
use image::{ImageBuffer, RgbImage};
use log::{error, info};
use std::time::Instant;

pub fn save_png(pixels: &[Color], width: u32, height: u32) {
    let start = Instant::now();
    let mut x = 0;
    let mut y = 0;
    let mut image: RgbImage = ImageBuffer::new(width, height);

    for p in pixels.iter() {
        let pixel = image::Rgb([p.r, p.g, p.b]);
        // info!("pixels_vec = {:?}, pixel = {:?}", p, pixel);
        image.put_pixel(x, y as u32, pixel);
        x += 1;
        if x % width == 0 {
            y += 1;
            x = 0;
        }
    }
    let now = Utc::now();
    let filename = format!(
        "fractal_{}_{}_{}.png",
        width,
        height,
        now.timestamp_millis()
    );
    let res = image.save(filename);
    let duration = start.elapsed().as_millis();
    match res {
        Ok(_) => info!("save ok. took {} ms", duration),
        Err(e) => error!("error saving file {}. took {} ms", e, duration),
    }
}

pub fn save_png2(
    pixels: &[Color],
    width: u32,
    height: u32,
    center: &ComplexNumber,
    zoom: f64,
    max_iterations: u32,
    name: String,
) {
    let start = Instant::now();
    let mut x = 0;
    let mut y = 0;
    let mut image: RgbImage = ImageBuffer::new(width, height);

    for p in pixels.iter() {
        let pixel = image::Rgb([p.r, p.g, p.b]);
        // info!("pixels_vec = {:?}, pixel = {:?}", p, pixel);
        image.put_pixel(x, y as u32, pixel);
        x += 1;
        if x % width == 0 {
            y += 1;
            x = 0;
        }
    }
    let now = Utc::now();
    let c = format!("center_a_{}_b_{}", center.a, center.b);
    let filename = format!(
        "{}_fractal_{}___{}x{}_center_{}_zoom_{}_max_iter_{}.png",
        name,
        now.timestamp_millis(),
        width,
        height,
        c,
        zoom,
        max_iterations,
    );
    let res = image.save(filename);
    let duration = start.elapsed().as_millis();
    match res {
        Ok(_) => info!("save ok. took {} ms", duration),
        Err(e) => error!("error saving file {}. took {} ms", e, duration),
    }
}
