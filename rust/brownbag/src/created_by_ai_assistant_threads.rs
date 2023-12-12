use num_complex::Complex;
use std::sync::{mpsc, Arc};
use std::thread;

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
            r: ct as u8 * 21,
            g: ct as u8 * 34,
            b: ct as u8 * 45,
        }
    }
}

fn worker(start_row: usize, end_row: usize, tx: mpsc::Sender<Vec<Pixel>>) {
    let scalex = 3.0 / 800 as f64;
    let scaley = 3.0 / 600 as f64;

    let mut pixels = vec![];

    for y in start_row..end_row {
        for x in 0..800 {
            let c = Complex::new(x as f64 * scalex - 2.0, y as f64 * scaley - 1.5);
            pixels.push(mandelbrot_set(c, 100));
        }
    }
    tx.send(pixels).unwrap();
}

pub fn created_by_ai_assistant_threads() {
    let (tx, rx) = mpsc::channel();

    for i in 0..600 {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            worker(i, i + 1, thread_tx);
        });
    }

    let mut img_data = Vec::with_capacity(800 * 600);
    for _ in 0..600 {
        img_data.push(rx.recv().unwrap());
    }
}
