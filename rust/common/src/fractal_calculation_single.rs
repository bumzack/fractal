use std::time::Instant;

use log::info;

use crate::color::{Color, color16, color256};
use crate::complex::ComplexNumber;
use crate::fractal::calc_fractal_color;
use crate::fractal_image::FractalImage;
use crate::utils::{print_debug, save_png2};

pub fn calc_single_threaded(
    center: &ComplexNumber,
    complex_width: f64,
    zoom: f64,
    width: u32,
    height: u32,
    max_iterations: u32,
    colors: u32,
    name: String,
) -> (FractalImage, u128) {
    let complex_width = complex_width / zoom;
    let ratio = width as f64 / height as f64;
    let complex_height = complex_width / ratio;

    print_debug(
        width,
        height,
        zoom,
        &center,
        complex_width,
        complex_height,
        ratio,
        max_iterations,
    );

    let start = Instant::now();

    let re_min = center.a - complex_width / 2.0;
    let re_max = center.a + complex_width / 2.0;

    let img_min = center.b - complex_height / 2.0;
    let img_max = center.b + complex_height / 2.0;

    let x_delta = (re_max - re_min) / width as f64;
    let y_delta = (img_max - img_min) / height as f64;

    info!("re_min {re_min}, re_max {re_max},  img_min {img_min}   img_max {img_max}  x_delta {x_delta}  y_delta  {y_delta} ");

    info!("x_delta {},   y_delta {}   width {}  height {},  max_iterations {},  re_min {}, re_max {}, img_min {}, img_max {}" ,
        x_delta, y_delta, width,  height, max_iterations, re_min, re_max, img_min, img_max);

    let mut pixels = vec![];

    let colors: Vec<Color> = match colors {
        16 => color16(),
        256 => color256(),
        _ => panic!("number of colors not supported {}", colors),
    };

    for y in 0..height {
        for x in 0..width {
            let p = calc_fractal_color(
                x,
                y,
                re_min,
                img_min,
                x_delta,
                y_delta,
                max_iterations,
                &colors,
            );
            pixels.push(p);
        }
    }

    let duration = start.elapsed().as_millis();

    let tl = ComplexNumber {
        a: re_min,
        b: img_max,
    };
    let br = ComplexNumber {
        a: re_max,
        b: img_min,
    };

    save_png2(
        &pixels,
        width,
        height,
        &center,
        &tl,
        &br,
        zoom,
        max_iterations,
        name,
    );

    let fractal = FractalImage {
        width,
        height,
        pixels: pixels,
    };

    (fractal, duration)
}
