use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crossbeam_channel::Sender;
use log::{error, info};
use rayon::prelude::IntoParallelRefMutIterator;
use rayon::prelude::ParallelIterator;

use crate::color::{color16, color256, Color};
use crate::complex::ComplexNumber;
use crate::fractal::calc_fractal_color;
use crate::fractal_image::FractalImage;
use crate::image_tile::{tiles, TileData, TileDataPoint};
use crate::palette::read_palette;
use crate::rayon_image::Pixel;
use crate::utils::save_png2;

pub fn calc_single_threaded(
    center: &ComplexNumber,
    complex_width: f64,
    zoom: f64,
    width: u32,
    height: u32,
    max_iterations: u32,
    colors: u32,
    name: String,
) -> (FractalImage, u128) {
    let complex_width = complex_width / zoom;
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

    let mut pixels = vec![];

    let colors: Vec<Color> = match colors {
        16 => color16(),
        256 => color256(),
        _ => panic!("number of colors not supported {}", colors),
    };

    for y in 0..height {
        for x in 0..width {
            let p = calc_fractal_color(
                x,
                y,
                re_min,
                img_min,
                x_delta,
                y_delta,
                max_iterations,
                &colors,
            );
            pixels.push(p);
        }
    }

    let duration = start.elapsed().as_millis();

    let tl = ComplexNumber {
        a: re_min,
        b: img_max,
    };
    let br = ComplexNumber {
        a: re_max,
        b: img_min,
    };

    save_png2(
        &pixels,
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
        pixels: pixels,
    };

    (fractal, duration)
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
    let complex_width = complex_width / zoom;
    let ratio = width as f64 / height as f64;
    let complex_height = complex_width / ratio;

    let palette = read_palette();

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
            16 => palette.get("wild.map").unwrap().clone(),
            256 => palette.get("neon.map").unwrap().clone(),
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
                        let p = calc_fractal_color(
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

    let tl = ComplexNumber {
        a: re_min,
        b: img_max,
    };
    let br = ComplexNumber {
        a: re_max,
        b: img_min,
    };

    save_png2(
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

pub fn calc_multi_threaded_crossbeam_tiles(
    center: &ComplexNumber,
    complex_width: f64,
    zoom: f64,
    width: u32,
    height: u32,
    max_iterations: u32,
    colors: u32,
    _name: String,
    x_tiles: u32,
    y_tiles: u32,
    sender: Sender<TileData>,
) {
    let cores = num_cpus::get();
    let start = Instant::now();

    let complex_width = complex_width / zoom;
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

    let re_min = center.a - complex_width / 2.0;
    let re_max = center.a + complex_width / 2.0;

    let img_min = center.b - complex_height / 2.0;
    let img_max = center.b + complex_height / 2.0;

    let x_delta = (re_max - re_min) / width as f64;
    let y_delta = (img_max - img_min) / height as f64;

    info!("re_min {re_min}, re_max {re_max},  img_min {img_min}   img_max {img_max}  x_delta {x_delta}  y_delta  {y_delta} ");

    info!("x_delta {},   y_delta {}   width {}  height {},  max_iterations {},  re_min {}, re_max {}, img_min {}, img_max {}" ,
        x_delta, y_delta, width,  height, max_iterations, re_min, re_max, img_min, img_max);

    //let pixels = vec![Color::default(); width as usize * height as usize];

    // let y_global = 0;

    let tiles = tiles(width, height, x_tiles, y_tiles);
    let tiles = Arc::new(Mutex::new(tiles));

    crossbeam::scope(|s| {
        let mut children = vec![];

        for _ in 0..cores {
            let sender_thread = sender.clone();
            let cloned_tiles = Arc::clone(&tiles);
            let colors: Vec<Color> = match colors {
                16 => color16(),
                256 => color256(),
                _ => panic!("number of colors not supported {}", colors),
            };

            children.push(s.spawn(move |_| {
                let mut cnt_tiles = 0;

                while cloned_tiles.lock().unwrap().peekable().peek().is_some() {
                    let tile_candidate;
                    {
                        tile_candidate = cloned_tiles.lock().unwrap().next();
                    }
                    match tile_candidate {
                        Some(ref tile) => {
                            let mut pixels = vec![];

                            cnt_tiles += 1;
                            for y in tile.y_from()..tile.y_to() {
                                for x in tile.x_from()..tile.x_to() {
                                    // info!("thread_id {:?}   raytracing pixel:  {}/{} ", thread::current().id(), x, y);
                                    let c = calc_fractal_color(
                                        x as u32,
                                        y as u32,
                                        re_min,
                                        img_min,
                                        x_delta,
                                        y_delta,
                                        max_iterations,
                                        &colors,
                                    );
                                    let tile_data_point = TileDataPoint::new(x as u32, y as u32, c);
                                    pixels.push(tile_data_point);
                                }
                            }

                            let tile_data = TileData::new(tile.get_idx(), pixels);
                            let idx = tile_data.get_idx();
                            match sender_thread.send(tile_data) {
                                Ok(_) => {
                                    info!("calc_multi_threaded_crossbeam_tiles:  sending  tile idx {}", idx);
                                }
                                Err(e) => {
                                    info!("calc_multi_threaded_crossbeam_tiles:  error sending a tile    {:?}", e.to_string());
                                }
                            };
                        }
                        None => {
                            info!(" no more tiles for thread {:?}", thread::current().id());
                        }
                    };
                }

                (thread::current().id(), cnt_tiles)
            }));
        }

        for child in children {
            let dur = start.elapsed().as_micros();
            let (thread_id, cnt_tiles) = child.join().unwrap();
            info!(
                "child thread {:?} finished. run for {} ms , processed {:?} tiles",
                thread_id, dur, cnt_tiles
            );
        }
        let duration = start.elapsed().as_millis();
        info!("duration {} ms", duration);
    })
        .expect("TODO: something went wrong");
}

pub fn calc_rayon(
    center: &ComplexNumber,
    complex_width: f64,
    zoom: f64,
    width: u32,
    height: u32,
    max_iterations: u32,
    colors: u32,
    name: String,
) -> (FractalImage, u128) {
    let colors: Vec<Color> = match colors {
        16 => color16(),
        256 => color256(),
        _ => panic!("number of colors not supported {}", colors),
    };

    let complex_width = complex_width / zoom;
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

    let mut pixels = vec![];
    for y in 0..height {
        for x in 0..width {
            let p = Pixel {
                color: Default::default(),
                x,
                y,
            };

            pixels.push(p);
        }
    }

    pixels.par_iter_mut().for_each(|p| {
        let x = p.x;
        let y = p.y;
        let color = calc_fractal_color(
            x as u32,
            y as u32,
            re_min,
            img_min,
            x_delta,
            y_delta,
            max_iterations,
            &colors,
        );
        p.color = color;
    });

    let pixels: Vec<Color> = pixels.iter().map(|p| p.color.clone()).collect();

    let duration = start.elapsed().as_millis();

    let tl = ComplexNumber {
        a: re_min,
        b: img_max,
    };
    let br = ComplexNumber {
        a: re_max,
        b: img_min,
    };

    save_png2(
        &pixels,
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
        pixels,
    };

    (fractal, duration)
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
