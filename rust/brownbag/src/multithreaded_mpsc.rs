use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{JoinHandle, ThreadId};
use std::time::Instant;

use crossbeam_channel::{Receiver, Sender, unbounded};

use crate::fractal::{BLACK, calc_color, Color, FractalImage};
use crate::tile::{CanvasTile, TileData, TileDataPoint};
use crate::utils::write_to_ppm;

pub fn multi_threaded_mpsc(width: usize, height: usize, max_iterations: usize) {
    let (tx, rx) = unbounded::<TileData>();

    let start = Instant::now();
    let cores = num_cpus::get();
    // let cores = 2;

    let x_tiles = 100;
    let y_tiles = 100;

    let x_tiles = width as u32;
    let y_tiles = 50;

    let tiles = tiles(width as u32, height as u32, x_tiles, y_tiles);
    let tiles = Arc::new(Mutex::new(tiles));

    let x = crossbeam::scope(|s| {
        let children = start_calc_threads(width, height, max_iterations, tx, cores as i32, &tiles);

        let receiver_thread = start_receiver_thread(width, height, rx);

        wait_for_calc_threads(start, children);
        let pixels = wait_for_receiver_thread(receiver_thread);
        let fractal_image = FractalImage {
            width,
            height,
            pixels,
        };

        fractal_image
    })
        .expect("MPSC stuff crashed");

    let duration = start.elapsed();
    println!("multi threaded mpsc  duration: {} ms", duration.as_millis());
    // println!("fractal image {}", fractal_image);

    write_to_ppm(
        &x,
        &format!(
            "multi_threaded_mpsc_{}x{}_max_iter_{}",
            width, height, max_iterations
        ),
    );
}

fn wait_for_receiver_thread(receiver_thread: JoinHandle<Vec<Color>>) -> Vec<Color> {
    let pixels = match receiver_thread.join() {
        Ok(res) => {
            //  println!("receiver thread finished");
            res
        }
        Err(e) => {
            println!("receiver thread crashed {:?}", e);
            vec![]
        }
    };
    pixels
}

fn wait_for_calc_threads(start: Instant, children: Vec<JoinHandle<(ThreadId, i32)>>) {
    for child in children {
        let dur = start.elapsed().as_millis();
        let (thread_id, cnt_tiles) = child.join().unwrap();
        // println!(
        //     "child thread {:?} finished. run for {} ms , processed {:?} tiles",
        //     thread_id, dur, cnt_tiles
        // );
    }
}

fn start_receiver_thread(
    width: usize,
    height: usize,
    rx: Receiver<TileData>,
) -> JoinHandle<Vec<Color>> {
    let receiver_thread = thread::spawn(move || {
        let mut pixels = vec![BLACK; width * height];
        while let Ok(tile_data) = rx.recv() {
            // let tile_data = rx.recv().expect("should be a tile");
            // println!("receiver thread got a tile idx {}", tile_data.get_idx());
            tile_data.get_points().iter().for_each(|p| {
                let idx = (p.get_y() * width as u32 + p.get_x()) as usize;
                pixels[idx] = p.get_color().clone();
            });
            // cnt += 1;
        }
        //  println!("receiver thread is finished");
        pixels
    });
    receiver_thread
}

fn start_calc_threads(
    width: usize,
    height: usize,
    max_iterations: usize,
    tx: Sender<TileData>,
    cores: i32,
    tiles: &Arc<Mutex<CanvasTile>>,
) -> Vec<JoinHandle<(ThreadId, i32)>> {
    let mut children = vec![];
    for _ in 0..cores {
        let sender_thread = tx.clone();
        let cloned_tiles = Arc::clone(&tiles);

        let t = thread::spawn(move || {
            let id = thread::current().id();

            let mut cnt_tiles = 0;

            while cloned_tiles.lock().unwrap().peekable().peek().is_some() {
                let tile_candidate;
                {
                    tile_candidate = cloned_tiles.lock().unwrap().next();
                }
                match tile_candidate {
                    Some(ref tile) => {
                        let mut pixels = vec![];

                        // println!(
                        //     "thread_id {:?}   tile.idx  {} ",
                        //     thread::current().id(),
                        //     tile.get_idx()
                        // );

                        cnt_tiles += 1;
                        for y in tile.y_from()..tile.y_to() {
                            for x in tile.x_from()..tile.x_to() {
                                // println!("thread_id {:?}   raytracing pixel:  {}/{} ", thread::current().id(), x, y);
                                let c = calc_color(x, y, width, height, max_iterations);
                                let tile_data_point = TileDataPoint::new(x as u32, y as u32, c);
                                pixels.push(tile_data_point);
                            }
                        }

                        let tile_data = TileData::new(tile.get_idx(), pixels);
                        let idx = tile_data.get_idx();
                        match sender_thread.send(tile_data) {
                            Ok(_) => {
                                // println!(
                                //     "calc_multi_threaded_crossbeam_tiles:  sending  tile idx {}",
                                //     idx
                                // );
                            }
                            Err(e) => {
                                println!("calc_multi_threaded_crossbeam_tiles:  error sending a tile    {:?}", e.to_string());
                            }
                        };
                    }
                    None => {
                        //  println!("no more tiles for thread {:?}", id);
                    }
                };
            }

            // println!("calc thread  {:?} is finished", id);
            (id, cnt_tiles)
        });
        children.push(t);
    }
    children
}

fn tiles(width: u32, height: u32, x_tiles: u32, y_tiles: u32) -> CanvasTile {
    let c = CanvasTile {
        x_inc: (width / x_tiles) as usize,
        y_inc: (height / y_tiles) as usize,
        width: width as usize,
        height: height as usize,
        x: 0,
        y: 0,
        idx: 0,
    };
    // println!("canvas tile  {:?}", &c);
    c
}
