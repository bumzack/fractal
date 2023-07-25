use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crossbeam_channel::Sender;
use log::{error, info};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefMutIterator;

use common::color::{color16, color256, Color, BLACK};
use common::complex::ComplexNumber;
use common::fractal_image::FractalImage;
use common::image_tile::{tiles, TileData, TileDataPoint};
use common::rayon_image::Pixel;

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

#[allow(dead_code)]
pub fn calc_multi_threaded_slow(
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

pub fn calc_multi_threaded(
    z1: &ComplexNumber,
    z2: &ComplexNumber,
    width: u32,
    max_iterations: u32,
    colors: u32,
) -> (FractalImage, u128, usize) {
    let cores = num_cpus::get();

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
                            &z1,
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

    save_png(&pixelssss, width, height);

    let fractal = FractalImage {
        width,
        height,
        pixels: pixelssss,
    };

    (fractal, duration, cores)
}

pub fn calc_image_height(width: u32, z1: &ComplexNumber, z2: &ComplexNumber) -> u32 {
    let x_diff = z1.a.abs() + z2.a.abs();
    let y_diff = z1.b.abs() + z2.b.abs();

    (x_diff * width as f32 / y_diff).round() as u32
}

pub fn calc_multi_threaded_crossbeam_tiles(
    z1: &ComplexNumber,
    z2: &ComplexNumber,
    width: u32,
    max_iterations: u32,
    colors: u32,
    x_tiles: u32,
    y_tiles: u32,
    sender: Sender<TileData>,
) {
    let cores = num_cpus::get() - 2;

    let start = Instant::now();

    let x_diff = z1.a.abs() + z2.a.abs();
    let y_diff = z1.b.abs() + z2.b.abs();

    // x_diff : y_diff = width : height
    // height = x_diff*width / y_diff
    let height = calc_image_height(width, z1, z2);

    let x_delta = x_diff / width as f32;
    let y_delta = y_diff / height as f32;

    info!("x_diff {},  y_diff {},   x_delta {},   y_delta {}   width {}  height {},  max_iterations {}",x_diff, y_diff, x_delta, y_delta, width,  height, max_iterations);

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
                                    let c = calc_fractal_color(x as u32, y as u32, z1, x_delta, y_delta, max_iterations, &colors);
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

// pub fn collect_tiles_to_canvas(r: Receiver<TileData>, width: usize, height: usize) -> FractalImage {
//     // let mut canvas = Canvas::new(width, height);
//     //
//     // r.iter().for_each(|tile_data| {
//     //     //  info!("collect_tiles_to_canvas   got a tile idx {}", tile_data.get_idx());
//     //     tile_data.get_points().iter().for_each(|p| {
//     //         canvas.write_pixel(p.get_x(), p.get_y(), p.get_color());
//     //     })
//     // });
//
//     FractalImage::default()
// }

pub fn calc_rayon(
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

    info!("x_diff {},  y_diff {},   x_delta {},   y_delta {}   width {}  height {},  max_iterations {}",x_diff, y_diff, x_delta, y_delta, width,  height, max_iterations);

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
        let color = calc_fractal_color(x, y, z1, x_delta, y_delta, max_iterations, &colors);
        p.color = color;
    });

    let pixels: Vec<Color> = pixels.iter().map(|p| p.color.clone()).collect();

    let duration = start.elapsed().as_millis();

    save_png(&pixels, width, height);

    let fractal = FractalImage {
        width,
        height,
        pixels,
    };

    (fractal, duration)
}
