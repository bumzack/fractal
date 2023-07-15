use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use log::{error, info};

use crate::color::{color16, color256, Color, BLACK};
use crate::complex::ComplexNumber;
use crate::fractal_image::FractalImage;
use crate::utils::save_png;

pub fn calc_fractal_color(
    x: u32,
    y: u32,
    upper_left: &ComplexNumber,
    x_delta: f32,
    y_delta: f32,
    max_iterations: u32,
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
    width: u32,
    max_iterations: u32,
    colors: u32,
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
    let height = (x_diff * width as f32 / y_diff).round() as u32;

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
    width: u32,
    max_iterations: u32,
    colors: u32,
) -> (FractalImage, u128, usize) {
    let cores = num_cpus::get() - 2;

    let start = Instant::now();

    let x_diff = z1.a.abs() + z2.a.abs();
    let y_diff = z1.b.abs() + z2.b.abs();

    // x_diff : y_diff = width : height
    // height = x_diff*width / y_diff
    let height = (x_diff * width as f32 / y_diff).round() as u32;

    let x_delta = x_diff / width as f32;
    let y_delta = y_diff / height as f32;

    info!("x_diff {},  y_diff {},   x_delta {},   y_delta {}   width {}  height {},  max_iterations {}",x_diff, y_diff, x_delta, y_delta, width,  height, max_iterations);

    let pixels = vec![Color::default(); width as usize * height as usize];

    let mut threads = vec![];

    let y_global = 0;

    let pixels = Arc::new(Mutex::new(pixels));
    let y_global = Arc::new(Mutex::new(y_global));

    for _ in 0..cores {
        let colors: Vec<Color> = match colors {
            16 => color16(),
            256 => color256(),
            _ => panic!("number of colors not supported {}", colors),
        };

        let z1 = z1.clone();
        let mut pixels_thread = vec![];

        let pixels = Arc::clone(&pixels);
        let y_global = Arc::clone(&y_global);

        let thread_join_handle = thread::spawn(move || {
            let start = Instant::now();
            let mut calculated_rows = 0;
            let mut y_thread = 0;

            while *y_global.lock().unwrap() < height {
                {
                    let mut y_global = y_global.lock().unwrap();
                    if *y_global < height {
                        y_thread = *y_global;
                        *y_global += 1;
                    }
                }
                // y_global is unlocked

                if y_thread < height {
                    info!(
                        "hi from thread {:?} - calculating row {}",
                        thread::current().id(),
                        y_thread
                    );

                    pixels_thread.clear();
                    for x in 0..width {
                        let p = calc_fractal_color(
                            x,
                            y_thread,
                            &z1,
                            x_delta,
                            y_delta,
                            max_iterations,
                            &colors,
                        );
                        pixels_thread.push(p);
                    }

                    {
                        let mut p = pixels.lock().unwrap();
                        for i in 0..width {
                            let idx = y_thread * width + i;
                            let p = &mut *p;
                            let pixel = &mut p[idx as usize];
                            pixel.r = pixels_thread[i as usize].r;
                            pixel.g = pixels_thread[i as usize].g;
                            pixel.b = pixels_thread[i as usize].b;
                        }
                    }
                }

                calculated_rows += 1;
            }

            let duration = start.elapsed().as_millis();
            let msg = format!(
                "hi from thread {:?} - i spent {} ms working on {} rows of the fractal",
                thread::current().id(),
                duration,
                calculated_rows
            );

            (msg, duration, calculated_rows)
        });
        threads.push(thread_join_handle);
    }

    for t in threads {
        let res = t.join();
        match res {
            Ok(s) => info!(
                "thread successfully joined with message '{}',   thread worked for {} ms on {} rows",
                s.0, s.1,s.2
            ),
            Err(e) => error!("thread returned an error {:?}", e),
        }
    }

    let duration = start.elapsed().as_millis();

    let pixels = pixels.lock().unwrap().clone();

    save_png(&pixels, width, height);

    let fractal = FractalImage {
        width,
        height,
        pixels,
    };

    (fractal, duration, cores)
}
