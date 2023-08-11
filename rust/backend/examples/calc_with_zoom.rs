use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use log::{error, info};

use common::color::{color16, color256, Color, BLACK};
use common::complex::ComplexNumber;
use common::fractal::calc_fractal_color2;
use common::fractal_image::FractalImage;
use common::utils::save_png2;

fn main() {
    flower();
    tendrils();
    julia_island();
    seahorse_valley();
    sun();
    tree();
    starfish();
}

fn flower() {
    let center = ComplexNumber {
        a: -1.999985882,
        b: 0.0,
    };

    let zoom = 1.0;
    let max_iterations: u32 = 50000;
    let zoom_factor = 1.01;
    let max_zoom_factor = 50_000_000_000.0;

    render(
        &center,
        zoom,
        max_iterations,
        zoom_factor,
        max_zoom_factor,
        "flower",
    );
}

fn tendrils() {
    let center = ComplexNumber {
        a: -0.226266648,
        b: 1.11617444,
    };

    let zoom = 1.0;
    let max_iterations: u32 = 50_000;
    let zoom_factor = 1.01;
    let max_zoom_factor = 50_000_000_000.0;

    render(
        &center,
        zoom,
        max_iterations,
        zoom_factor,
        max_zoom_factor,
        "tendrils",
    );
}

fn julia_island() {
    let center = ComplexNumber {
        a: -1.768778833,
        b: -0.001738996,
    };

    let zoom = 1.0;
    let max_iterations: u32 = 50_000;
    let zoom_factor = 1.01;
    let max_zoom_factor = 50_000_000_000.0;

    render(
        &center,
        zoom,
        max_iterations,
        zoom_factor,
        max_zoom_factor,
        "julia_island",
    );
}

fn seahorse_valley() {
    let center = ComplexNumber {
        a: -0.743517833,
        b: -0.127094578,
    };

    let zoom = 1.0;
    let max_iterations: u32 = 50_000;
    let zoom_factor = 1.01;
    let max_zoom_factor = 50_000_000_000.0;

    render(
        &center,
        zoom,
        max_iterations,
        zoom_factor,
        max_zoom_factor,
        "seahorse_valley",
    );
}

fn starfish() {
    let center = ComplexNumber {
        a: -0.3740041393,
        b: 0.659792175,
    };

    let zoom = 1.0;
    let max_iterations: u32 = 50_000;
    let zoom_factor = 1.01;
    let max_zoom_factor = 50_000_000_000.0;

    render(
        &center,
        zoom,
        max_iterations,
        zoom_factor,
        max_zoom_factor,
        "starfish",
    );
}

fn sun() {
    let center = ComplexNumber {
        a: -0.776592847,
        b: -0.136640848,
    };

    let zoom = 1.0;
    let max_iterations: u32 = 50_000;
    let zoom_factor = 1.01;
    let max_zoom_factor = 50_000_000_000.0;

    render(
        &center,
        zoom,
        max_iterations,
        zoom_factor,
        max_zoom_factor,
        "sun",
    );
}

fn tree() {
    let center = ComplexNumber {
        a: -1.940157343,
        b: -1. / 1250000.0,
    };

    let zoom = 1.0;
    let max_iterations: u32 = 50_000;
    let zoom_factor = 1.01;
    let max_zoom_factor = 50_000_000_000.0;

    render(
        &center,
        zoom,
        max_iterations,
        zoom_factor,
        max_zoom_factor,
        "tree",
    );
}

fn render(
    center: &ComplexNumber,
    mut zoom: f64,
    max_iterations: u32,
    zoom_factor: f64,
    max_zoom_factor: f64,
    name: &str,
) {
    let width: u32 = 4096;
    let height: u32 = 2160;

    // -4.0 ... 1.3
    let complex_width = 5.3;

    let start = Instant::now();

    while zoom < max_zoom_factor {
        // 6591292.0
        let c_w = complex_width / zoom;
        let (image, duration, cores) = calc_multi_threaded(
            &center,
            c_w,
            zoom,
            width,
            height,
            max_iterations,
            256,
            name.to_string(),
        );

        println!("name:  {name} duration {duration},   cores {cores},     zoom {zoom}");
        zoom = zoom * zoom_factor;
    }

    println!("rendering took {} seconds", start.elapsed().as_secs_f64());
}

fn calc_fractal(
    center: &ComplexNumber,
    zoom: f64,
    width: u32,
    height: u32,
    complex_width: f64,
    complex_height: f64,
    max_iterations: u32,
) {
    let re_min = center.a - complex_width / 2.0;
    let re_max = center.a + complex_width / 2.0;

    let img_min = center.b - complex_height / 2.0;
    let img_max = center.b + complex_height / 2.0;

    let x_delta = (re_max - re_min) / width as f64;
    let y_delta = (img_max - img_min) / height as f64;

    info!("re_min {re_min}, re_max {re_max},  img_min {img_min}   img_max {img_max}  x_delta {x_delta}  y_delta  {y_delta} ");

    let mut pixels: Vec<Color> = vec![BLACK; (width * height) as usize];
    for y in 0..height {
        for x in 0..width {
            let c = calc_fractal_color2(
                x,
                y,
                re_min,
                img_min,
                x_delta,
                y_delta,
                max_iterations,
                &color256(),
            );
            let idx = (y * width + x) as usize;
            pixels[idx] = c;

            if x % 50 == 0 {
                info!("x {x}, y {y}");
            }
        }
    }

    save_png2(
        &pixels,
        width,
        height,
        &center,
        zoom,
        max_iterations,
        "single_threaded".to_string(),
    );
}

fn print_debug(
    width: u32,
    height: u32,
    zoom: f64,
    center: &ComplexNumber,
    complex_width: f64,
    complex_height: f64,
    ratio: f64,
    max_iterations: u32,
) {
    info!("width {width}, height: {height}, zoom  {zoom},  complex_width {complex_width},  complex_height {complex_height}   ratio {ratio},  center {center},  max_iterations {max_iterations}");
}

pub fn calc_multi_threaded(
    center: &ComplexNumber,
    complex_width: f64,
    zoom: f64,
    width: u32,
    height: u32,
    max_iterations: u32,
    colors: u32,
    name: String,
) -> (FractalImage, u128, usize) {
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

    let cores = num_cpus::get();

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

        // let z1 = z1.clone();
        let mut pixels_thread = vec![Color::default(); width as usize];

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
                    for x in 0..width {
                        let p = calc_fractal_color2(
                            x,
                            y_thread,
                            re_min,
                            img_min,
                            x_delta,
                            y_delta,
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

    save_png2(
        &pixelssss,
        width,
        height,
        &center,
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
