use std::fs::File;
use std::io::Write;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;

use num_complex::Complex;

#[derive(Clone, Copy)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

type ComplexCoord = Complex<f64>;

fn mandelbrot_set(c: ComplexCoord, limit: u32) -> Pixel {
    let mut z = Complex::new(0.0, 0.0);
    let mut ct = 0_u32;
    while ct < limit && z.norm_sqr() <= 4.0 {
        z = z * z + c;
        ct += 1;
    }
    if ct == limit {
        Pixel { r: 0, g: 0, b: 0 }
    } else {
        Pixel {
            r: (ct * 2) as u8,
            g: (ct * 3) as u8,
            b: (ct * 5) as u8,
        }
    }
}

fn worker(
    row: usize,
    width: usize,
    center: ComplexCoord,
    zoom: f64,
    max_iter: u32,
    tx: mpsc::Sender<(usize, Vec<Pixel>)>,
) {
    let mut pixels = Vec::new();
    let im = (row as f64 - center.im) / zoom;
    for x in 0..width {
        let re = (x as f64 - center.re) / zoom;
        let c = Complex::new(re, im);
        pixels.push(mandelbrot_set(c, max_iter));
    }
    tx.send((row, pixels)).unwrap();
}

pub fn created_by_ai_using_mpsc(
    width: usize,
    height: usize,
    max_iter: u32,
    center_re: f64,
    center_img: f64,
) {
    let center = Complex::new(center_re, center_img);
    let zoom = 1.0;

    let (tx, rx) = mpsc::channel();

    let start_time = Instant::now();

    for i in 0..height {
        let tx = tx.clone();
        thread::spawn(move || worker(i, width, center, zoom, max_iter, tx));
    }

    let mut img_data = vec![vec!(Pixel { r: 0, g: 0, b: 0 }; width); height];
    for _ in 0..height {
        let (row, pixels) = rx.recv().unwrap();
        for (column, pixel) in pixels.iter().enumerate() {
            img_data[row][column] = *pixel;
        }
    }

    let elapsed = start_time.elapsed();
    println!("Total time: {:?}", elapsed);
    // Further code to write the img_data to a file.

    write_to_ppm(&img_data, width, height, "created_by_ai_using_mpsc");
}

fn write_to_ppm(fractal_image: &Vec<Vec<Pixel>>, width: usize, height: usize, filename: &str) {
    let full_path = format!("{}/{}.ppm", env!("CARGO_MANIFEST_DIR"), filename);
    let mut file = File::create(full_path).expect("cant create file");

    let data = "P3\n";
    let _ = file.write(data.as_ref()).expect("Unable to write file");
    let data = format!("{} {}\n", width, height);
    let _ = file.write(data.as_ref()).expect("Unable to write file");
    let data = format!("255\n");
    let _ = file.write(data.as_ref()).expect("Unable to write file");

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let _ = file
                .write(
                    format!(
                        "{} {} {} ",
                        fractal_image[y][x].r, fractal_image[y][x].g, fractal_image[y][x].b
                    )
                    .as_ref(),
                )
                .expect("Unable to write pixel to file");

            // line should not be longer than 70 characters - "255 255 255 " = 12 characters
            if (idx + 1) % 5 == 0 {
                let _ = file
                    .write("\n".as_ref())
                    .expect("Unable to write pixel to file");
            }
        }
    }
}
