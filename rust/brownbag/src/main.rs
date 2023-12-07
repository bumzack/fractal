use std::fmt::{Display, Formatter};
use std::thread;
use std::time::Instant;

use crate::fractal::ColorEnum::BLACK;
use crate::fractal::{calc_color, ColorEnum, ASCII_RESET_BACKGROUND};

mod complex;
mod fractal;

fn main() {
    single_threaded();
}

fn multi_threaded() {
    let start = Instant::now();

    let width = 80;
    let height = 40;
    let max_iterations = 1000;
    let mut pixels = vec![BLACK; width * height];

    let cores = 4;

    let mut threads = vec![];

    for _ in 0..cores {
        let t = thread::spawn(|| thread::current().id());

        threads.push(t);
    }

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

fn single_threaded() {
    let start = Instant::now();

    let width = 80;
    let height = 40;
    let max_iterations = 1000;
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
            res = format!("{}{}   \n", ASCII_RESET_BACKGROUND, res);
        }
        write!(f, "{}", res)
    }
}
