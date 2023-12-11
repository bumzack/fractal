use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crate::fractal::{calc_color, Color, ASCII_RESET_BACKGROUND, BLACK};
use crate::utils::write_to_ppm;

mod complex;
mod fractal;
mod utils;

fn main() {
    let width = 200;
    let height = 160;
    let max_iterations = 1_000;

    single_threaded(width, height, max_iterations);
    multi_threaded(width, height, max_iterations);
}

fn multi_threaded(width: usize, height: usize, max_iterations: usize) {
    let start = Instant::now();
    let pixels = vec![BLACK; width * height];
    let cores = 8;
    let mut threads = vec![];
    let y_global = 0;
    let y_global = Arc::new(Mutex::new(y_global));
    let pixels = Arc::new(Mutex::new(pixels));

    for _ in 0..cores {
        let y_global = y_global.clone();
        let pixels = pixels.clone();
        let mut processed_rows = 0;

        let t = thread::spawn(move || {
            let id = thread::current().id();
            println!("hi from thread {:?}", id);
            let mut y = 0;
            while *y_global.lock().unwrap() < height {
                {
                    let mut y_global = y_global.lock().unwrap();
                    if *y_global < height {
                        y = *y_global;
                        *y_global += 1;
                    }
                }
                if y < height {
                    let mut row = vec![];

                    for x in 0..width {
                        row.push(calc_color(x, y, width, height, max_iterations));
                    }

                    {
                        let mut p = pixels.lock().unwrap();
                        for x in 0..width {
                            let idx = y * width + x;
                            p[idx].r = row[x].r;
                            p[idx].g = row[x].g;
                            p[idx].b = row[x].b;
                        }
                    }

                    processed_rows += 1;
                }
            }
            (id, processed_rows)
        });

        threads.push(t);
    }
    println!("after starting the threads\n\n\n\n");

    for t in threads {
        let res = t.join();
        match res {
            Ok(thread_return_value) => println!(
                "thread {:?} finished.  processed {}",
                thread_return_value.0, thread_return_value.1
            ),
            Err(e) => println!("thread returned an error {:?}", e),
        }
    }

    let mutex = Arc::into_inner(pixels).unwrap();
    let pixels = mutex.into_inner().unwrap();

    let fractal_image = FractalImage {
        width,
        height,
        pixels,
    };

    let duration = start.elapsed();
    println!("multi threaded duration: {} ms", duration.as_millis());
    // println!("fractal image {}", fractal_image);

    write_to_ppm(
        &fractal_image,
        &format!(
            "multi_threaded_{}x{}_max_iter_{}",
            width, height, max_iterations
        ),
    );
}

fn single_threaded(width: usize, height: usize, max_iterations: usize) {
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

#[derive(Debug)]
pub struct FractalImage {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Display for FractalImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        res = format!("{}\n", res);
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.width * y + x;
                // write space
                res = format!("{}{} ", res, self.pixels[idx]);
            }
            res = format!("{}{} \n", res, ASCII_RESET_BACKGROUND,);
        }
        write!(f, "{}", res)
    }
}
