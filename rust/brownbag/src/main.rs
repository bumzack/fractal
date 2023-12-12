use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crate::created_by_ai_assistant_mpsc::created_by_ai_using_mpsc;
use crate::created_by_ai_assistant_rayon::created_by_ai_assistant_rayon;
use crate::created_by_ai_assistant_threads::created_by_ai_assistant_threads;
use crate::created_by_ai_assistant_threads_work_stealing_rayon::created_by_ai_assistant_threads_work_stealing_rayon;
use crate::fractal::{calc_color, Color, ASCII_RESET_BACKGROUND, BLACK};
use crate::utils::write_to_ppm;

mod complex;
mod created_by_ai_assistant_mpsc;
mod created_by_ai_assistant_rayon;
mod created_by_ai_assistant_threads;
mod created_by_ai_assistant_threads_work_stealing_rayon;
mod fractal;
mod palette;
mod utils;

fn main() {
    let width = 4096;
    let height = 3072;
    let max_iterations = 100_000;

    let width = 320;
    let height = 240;
    let max_iterations = 1_000;

    created_by_ai_using_mpsc(width, height, max_iterations as u32, -0.8, 0.0);

    // crashes
    // /created_by_ai_assistant_rayon(width, height, max_iterations as u32);

    // crashes
    // created_by_ai_assistant_threads();

    // crashes
    // created_by_ai_assistant_threads();

    created_by_ai_assistant_threads_work_stealing_rayon();

    // single_threaded(width, height, max_iterations);
    multi_threaded(width, height, max_iterations);
}

fn multi_threaded(width: usize, height: usize, max_iterations: usize) {
    let start = Instant::now();
    let pixels = vec![BLACK; width * height];
    let cores = num_cpus::get();
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
