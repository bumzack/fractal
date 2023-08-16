use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{fs, thread};

use chrono::Utc;
use image::{ImageBuffer, RgbImage};
use log::{error, info};

use crate::color::{Color, BLACK};
use crate::fractal_image::FractalImage;
use crate::palette::read_palette;
use crate::rational::complex_rational_numbers::ComplexRationalNumber;
use crate::rational::rational_numbers::RationalNumber;

pub fn calc_multi_threaded_rational(
    center: &ComplexRationalNumber,
    complex_width: RationalNumber,
    zoom: RationalNumber,
    width: u32,
    height: u32,
    max_iterations: u32,
    colors: u32,
    name: String,
) -> (FractalImage, u128, usize) {
    let complex_width = &complex_width / &zoom;
    let ratio = RationalNumber {
        num: width as i128,
        denom: height as i128,
    };
    let complex_height = &complex_width / &ratio;

    let palette = read_palette();

    print_debug_rational(
        width,
        height,
        &zoom,
        &center,
        &complex_width,
        &complex_height,
        &ratio,
        max_iterations,
    );

    let cores = num_cpus::get();

    let start = Instant::now();

    let two = RationalNumber { num: 2, denom: 1 };

    let re_min = &center.a - &(&complex_width / &two);
    let re_max = &center.a + &(&complex_width / &two);

    let img_min = &center.b - &(&complex_height / &two);
    let img_max = &center.b + &(&complex_height / &two);

    let w = RationalNumber {
        num: width as i128,
        denom: 1,
    };
    let h = RationalNumber {
        num: height as i128,
        denom: 1,
    };
    let x_delta = &(&re_max - &re_min) / &w;
    let y_delta = &(&img_max - &img_min) / &h;

    info!("re_min {re_min}, re_max {re_max},  img_min {img_min}   img_max {img_max}  x_delta {x_delta}  y_delta  {y_delta} ");

    info!("x_delta {},   y_delta {}   width {}  height {},  max_iterations {},  re_min {}, re_max {}, img_min {}, img_max {}" ,
        x_delta, y_delta, width,  height, max_iterations, re_min, re_max, img_min, img_max);

    let pixels = vec![Color::default(); width as usize * height as usize];

    let mut threads = vec![];

    let y_global = 0;

    let pixels = Arc::new(Mutex::new(pixels));
    let y_global = Arc::new(Mutex::new(y_global));

    for _ in 0..cores {
        let colors: Vec<Color> = match colors {
            16 => palette.get("wild.map").unwrap().clone(),
            256 => palette.get("wild.map").unwrap().clone(),
            _ => panic!("number of colors not supported {}", colors),
        };

        // let z1 = z1.clone();
        let mut pixels_thread = vec![Color::default(); width as usize];

        let pixels = Arc::clone(&pixels);
        let y_global = Arc::clone(&y_global);

        let re_min = re_min.clone();
        // let re_max = re_max.clone();
        let img_min = img_min.clone();
        // let img_max = img_max.clone();
        let x_delta = x_delta.clone();
        let y_delta = y_delta.clone();

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
                    println!("calculating   y_thread: {}", y_thread);

                    for x in 0..width {
                        let p = calc_fractal_color_rational(
                            x,
                            y_thread,
                            re_min.clone(),
                            img_min.clone(),
                            x_delta.clone(),
                            y_delta.clone(),
                            max_iterations,
                            &colors,
                        );
                        pixels_thread[x as usize].r = p.r;
                        pixels_thread[x as usize].g = p.g;
                        pixels_thread[x as usize].b = p.b;
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
    let mutex = Arc::try_unwrap(pixels).unwrap();
    let pixelssss = mutex.into_inner().unwrap();

    let tl = ComplexRationalNumber {
        a: re_min.clone(),
        b: img_max.clone(),
    };
    let br = ComplexRationalNumber {
        a: re_max.clone(),
        b: img_min.clone(),
    };

    save_png_rational(
        &pixelssss,
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
        pixels: pixelssss,
    };

    (fractal, duration, cores)
}

fn print_debug_rational(
    width: u32,
    height: u32,
    zoom: &RationalNumber,
    center: &ComplexRationalNumber,
    complex_width: &RationalNumber,
    complex_height: &RationalNumber,
    ratio: &RationalNumber,
    max_iterations: u32,
) {
    info!("width {width}, height: {height}, zoom  {zoom},  complex_width {complex_width},  complex_height {complex_height}   ratio {ratio},  center {center},  max_iterations {max_iterations}");
}

pub fn save_png_rational(
    pixels: &[Color],
    width: u32,
    height: u32,
    _center: &ComplexRationalNumber,
    _tl: &ComplexRationalNumber,
    _br: &ComplexRationalNumber,
    _zoom: RationalNumber,
    _max_iterations: u32,
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
    // let c = format!("center_a_{}_b_{}", center.a, center.b);

    let path = env!("CARGO_MANIFEST_DIR");
    // println!("CARGO_MANIFEST_DIR   {path}");
    let path = format!("{}/../../images/{}", path, name);
    fs::create_dir_all(&path).expect("create dir should work");

    let filename = format!(
        "{}/{}_fractal_{}___{}x{}.png",
        path,
        name,
        now.timestamp_millis(),
        width,
        height,
    );
    let res = image.save(filename);
    let duration = start.elapsed().as_millis();
    match res {
        Ok(_) => info!("save ok. took {} ms", duration),
        Err(e) => error!("error saving file {}. took {} ms", e, duration),
    }
}

pub fn calc_fractal_color_rational(
    x: u32,
    y: u32,
    re_min: RationalNumber,
    img_min: RationalNumber,
    x_delta: RationalNumber,
    y_delta: RationalNumber,
    max_iterations: u32,
    colors: &Vec<Color>,
) -> Color {
    let mut cnt_iterations = 0;
    let x = RationalNumber {
        num: x as i128,
        denom: 1,
    };
    let y = RationalNumber {
        num: y as i128,
        denom: 1,
    };
    let c = ComplexRationalNumber {
        a: re_min + x_delta * x,
        b: img_min + y_delta * y,
    };

    println!("c = {}", c);

    let mut z = ComplexRationalNumber::default();
    let radius_4 = RationalNumber { num: 4, denom: 1 };

    println!("initial z = {}", z);
    while z.length_squared() < radius_4 && cnt_iterations < max_iterations {
        z = z.pow2() + &c;
        println!("z_new = {}", z);

        cnt_iterations += 1;
    }
    info!(
        "z = {}, c = {} ,  cnt_iterations {}, max_iterations {}",
        &z, &c, cnt_iterations, max_iterations
    );

    if cnt_iterations >= max_iterations {
        info!(
            "BLACK       z = {}, c = {} ,  cnt_iterations {}, max_iterations {}",
            &z, &c, cnt_iterations, max_iterations
        );
        BLACK
    } else {
        let idx = cnt_iterations as usize % colors.len();
        let c: &Color = colors.get(idx).unwrap();
        info!(
            "color    idx {}   z = {}, c = {} ,  cnt_iterations {}, max_iterations {}",
            idx, &z, &c, cnt_iterations, max_iterations
        );
        c.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::fractal::calc_fractal_color;
    use crate::palette::read_palette;
    use crate::rational::rational_fractal_calculation::calc_fractal_color_rational;
    use crate::rational::rational_numbers::RationalNumber;

    #[test]
    fn test_calc() {
        let x = 200;
        let y = 200;
        let re_min = RationalNumber { num: 12, denom: 10 };
        let img_min = RationalNumber { num: 12, denom: 10 };
        let x_delta = RationalNumber { num: 12, denom: 10 };
        let y_delta = RationalNumber { num: 12, denom: 10 };

        let max_iterations = 100;

        let palette = read_palette();
        let colors: &Vec<Color> = palette.get("wild.map").unwrap();

        let c = calc_fractal_color_rational(
            x,
            y,
            re_min,
            img_min,
            x_delta,
            y_delta,
            max_iterations,
            colors,
        );

        println!("calculated color {}", c);
    }

    #[test]
    fn test_calc_f64() {
        let x = 200;
        let y = 200;
        let re_min = 1.2;
        let img_min = 1.2;
        let x_delta = 1.2;
        let y_delta = 1.2;

        let max_iterations = 100;

        let palette = read_palette();
        let colors: &Vec<Color> = palette.get("wild.map").unwrap();

        let c = calc_fractal_color(
            x,
            y,
            re_min,
            img_min,
            x_delta,
            y_delta,
            max_iterations,
            colors,
        );

        println!("calculated color {}", c);
    }
}
