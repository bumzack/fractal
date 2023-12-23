use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crossbeam_channel::Sender;
use log::info;

use crate::color::{Color, color16, color256};
use crate::complex::ComplexNumber;
use crate::fractal::calc_fractal_color;
use crate::image_tile::{TileData, TileDataPoint, tiles};
use crate::utils::print_debug;

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
 
