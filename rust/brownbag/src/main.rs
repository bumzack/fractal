use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::fractal::ColorEnum::BLACK;
use crate::fractal::{calc_color, ColorEnum, ASCII_RESET_BACKGROUND};

mod complex;
mod fractal;

fn main() {
    // single_threaded();

    let width = 240;
    let height = 120;
    let max_iterations = 100;

    multi_threaded(width, height, max_iterations);
    single_threaded(width, height, max_iterations);
}

fn multi_threaded(width: usize, height: usize, max_iterations: usize) {
    let start = Instant::now();

    let pixels = vec![BLACK; width * height];

    let cores = 4;

    let mut threads = vec![];

    let y_global = 0;

    let y_global = Arc::new(Mutex::new(y_global));
    let pixels = Arc::new(Mutex::new(pixels));

    for _ in 0..cores {
        let y_global = y_global.clone();
        let pixels = pixels.clone();

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

                if y < height {}
                let mut row = vec![];

                for x in 0..width {
                    row.push(calc_color(x, y, width, height, max_iterations));
                }

                let mut p = pixels.lock().unwrap();

                for x in 0..width {
                    let idx = y * width + x;
                    let pixel = row[x];
                    p[idx] = pixel;
                }
            }

            id
        });

        threads.push(t);
    }
    println!("after starting the threads\n\n\n\n");

    for t in threads {
        let res = t.join();
        match res {
            Ok(id) => println!("thread {:?} finished", id),
            Err(e) => println!("thread returned an error {:?}", e),
        }
    }

    let mutex = Arc::try_unwrap(pixels).unwrap();
    let pixelssss = mutex.into_inner().unwrap();

    let fractal_image = FractalImage {
        width,
        height,
        pixels: pixelssss,
    };

    let duration = start.elapsed();

    println!("duration {}", duration.as_millis());

    println!("fractal image {}", fractal_image);
}

fn _multi_threaded2() {
    let start = Instant::now();

    let width = 80;
    let height = 40;
    let max_iterations = 1000;
    let mut pixels = vec![BLACK; width * height];

    let cores = 4;

    let mut threads = vec![];

    for _ in 0..cores {
        let t = thread::spawn(|| {
            let id = thread::current().id();
            println!("hi from thread {:?}", id);
            thread::sleep(Duration::from_millis(500));
            println!("by from thread {:?}", id);
            // id
        });

        threads.push(t);
    }
    println!("after starting the threads");

    for t in threads {
        let res = t.join();
        match res {
            Ok(id) => println!("thread {:?} finished", id),
            Err(e) => println!("thread returned an error {:?}", e),
        }
    }

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            pixels[idx] = calc_color(x, y, width, height, max_iterations);
        }
    }

    let fractal_image = FractalImage {
        width,
        height,
        pixels,
    };

    let duration = start.elapsed();

    // println!("duration {}", duration.as_millis());

    // println!("fractal image {}", fractal_image);
}

fn single_threaded(width: usize, height: usize, max_iterations: usize) {
    let start = Instant::now();

    let mut pixels = vec![BLACK; width * height];

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let color = calc_color(x, y, width, height, max_iterations);
            pixels[idx] = color;
        }
    }

    let fractal_image = FractalImage {
        width,
        height,
        pixels,
    };

    let duration = start.elapsed();

    println!("duration {}", duration.as_millis());

    println!("fractal image {}", fractal_image);
}

#[derive(Debug)]
struct FractalImage {
    width: usize,
    height: usize,
    pixels: Vec<ColorEnum>,
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
