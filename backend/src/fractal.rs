use std::thread;
use std::time::{Duration, Instant};

use log::{error, info};

use crate::color::{color16, color256, Color, BLACK};
use crate::complex::ComplexNumber;
use crate::fractal_image::FractalImage;
use crate::utils::save_png;

pub fn calc_fractal_color(
    x: u16,
    y: u16,
    upper_left: &ComplexNumber,
    x_delta: f32,
    y_delta: f32,
    max_iterations: u16,
    colors: &Vec<Color>,
) -> Color {
    let mut cnt_iterations = 0;
    let c = ComplexNumber {
        a: upper_left.a + x as f32 * x_delta,
        b: upper_left.b - (y) as f32 * y_delta,
    };

    // info!("c = {}", &c);

    let mut z = ComplexNumber::default();
    while z.length_squared() < 4.0 && cnt_iterations < max_iterations {
        z = z.pow2() + &c;
        cnt_iterations += 1;
    }
    //info!("z = {}, c = {} ,  cnt_iterations {}, max_iterations {}", &z, &c, cnt_iterations, max_iterations);

    if cnt_iterations >= max_iterations {
        //  info!("BLACK       z = {}, c = {} ,  cnt_iterations {}, max_iterations {}", &z, &c, cnt_iterations, max_iterations);
        BLACK
    } else {
        let idx = cnt_iterations as usize % colors.len();
        let c: &Color = colors.get(idx).unwrap();
        //  info!("color    idx {}   z = {}, c = {} ,  cnt_iterations {}, max_iterations {}",idx, &z, &c, cnt_iterations, max_iterations);
        c.clone()
    }
}

pub fn calc_single_threaded(
    z1: &ComplexNumber,
    z2: &ComplexNumber,
    width: u16,
    max_iterations: u16,
    colors: u16,
) -> (FractalImage, u128) {
    let colors: Vec<Color> = match colors {
        16 => color16(),
        256 => color256(),
        _ => panic!("number of colors not supported {}", colors),
    };

    let start = Instant::now();

    let x_diff = z1.a.abs() + z2.a.abs();
    let y_diff = z1.b.abs() + z2.b.abs();

    // x_diff : y_diff = width : height
    // height = x_diff*width / y_diff
    let height = (x_diff * width as f32 / y_diff).round() as u16;

    let x_delta = x_diff / width as f32;
    let y_delta = y_diff / height as f32;

    info!("x_diff {},  y_diff {},   x_delta {},   y_delta {}   width {}  height {},  max_iterations {}",
    x_diff, y_diff, x_delta, y_delta, width,  height, max_iterations);

    let mut pixels = vec![];

    for y in 0..height {
        for x in 0..width {
            let p = calc_fractal_color(x, y, z1, x_delta, y_delta, max_iterations, &colors);
            pixels.push(p);
        }
    }

    let duration = start.elapsed().as_millis();

    save_png(&pixels, width, height);

    let fractal = FractalImage {
        width,
        height,
        pixels,
    };

    (fractal, duration)
}

pub fn calc_multi_threaded(
    z1: &ComplexNumber,
    z2: &ComplexNumber,
    width: u16,
    max_iterations: u16,
    colors: u16,
) -> (FractalImage, u128, usize) {
    let cores = num_cpus::get() - 2;
    let colors: Vec<Color> = match colors {
        16 => color16(),
        256 => color256(),
        _ => panic!("number of colors not supported {}", colors),
    };

    let start = Instant::now();

    let x_diff = z1.a.abs() + z2.a.abs();
    let y_diff = z1.b.abs() + z2.b.abs();

    // x_diff : y_diff = width : height
    // height = x_diff*width / y_diff
    let height = (x_diff * width as f32 / y_diff).round() as u16;

    let x_delta = x_diff / width as f32;
    let y_delta = y_diff / height as f32;

    info!("x_diff {},  y_diff {},   x_delta {},   y_delta {}   width {}  height {},  max_iterations {}",
    x_diff, y_diff, x_delta, y_delta, width,  height, max_iterations);

    let mut pixels = vec![];

    let mut threads = vec![];

    for _ in 0..cores {
        let thread_join_handle = thread::spawn(move || {
            info!(
                "hi from thread {:?} - sleeping for 1 second",
                thread::current().id()
            );
            thread::sleep(Duration::from_secs(1));
            format!(
                "hi from thread {:?} - awoke from 1 second sleep",
                thread::current().id()
            )
        });
        threads.push(thread_join_handle);
    }

    for t in threads {
        let res = t.join();
        match res {
            Ok(s) => info!("thread successfully joined with message '{}'", s),
            Err(e) => error!("thread returned an error {:?}", e),
        }
    }

    for y in 0..height {
        for x in 0..width {
            let p = calc_fractal_color(x, y, z1, x_delta, y_delta, max_iterations, &colors);
            pixels.push(p);
        }
    }

    let duration = start.elapsed().as_millis();

    save_png(&pixels, width, height);

    let fractal = FractalImage {
        width,
        height,
        pixels,
    };

    (fractal, duration, cores)
}
