use std::time::Instant;

use crossbeam_channel::unbounded;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use log::{error, info};
use serde_json::json;
use warp::reply::json;
use warp::ws::{Message, WebSocket};
use warp::{Filter, Reply};

use crate::color::{Color, BLACK, BLUE, NAVY, RED, YELLOW};
use crate::fractal::{
    calc_multi_threaded, calc_multi_threaded_crossbeam_tiles, calc_rayon, calc_single_threaded,
};
use crate::fractal_image::FractalImage;
use crate::image_tile::TileData;
use crate::index_html::INDEX_HTML;
use crate::models::{Request, Response};
use crate::utils::save_png;
use crate::{models, utils};

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    let server_source = warp::path!("api" / "singlethreaded");
    let single_threaded = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|req: Request| {
            info!("POST api/singlethreaded");
            handle_request_single_threaded(req)
        });

    let server_source = warp::path!("api" / "multithreaded");
    let multi_threaded = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|req| {
            info!("POST api/multithreaded");
            handle_request_multi_threaded(req)
        });

    let server_source = warp::path!("api" / "rayon");
    let multi_threaded_rayon = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|req| {
            info!("POST api/rayon");
            handle_request_rayon(req)
        });

    let server_source = warp::path!("api" / "crossbeamtiles");
    let multi_threaded_crossbeam_tiles = server_source.and(warp::ws()).map(|ws: warp::ws::Ws| {
        info!("websocket api/crossbeamtiles");
        ws.on_upgrade(move |socket| handle_request_crossbeam_tiles(socket))
    });

    let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));

    index
        .or(multi_threaded)
        .or(multi_threaded_rayon)
        .or(multi_threaded_crossbeam_tiles)
        .or(single_threaded)
}

pub async fn handle_request_single_threaded(req: Request) -> utils::Result<impl Reply> {
    let (fractal, duration) =
        calc_single_threaded(&req.z1, &req.z2, req.width, req.max_iterations, req.colors);

    let response = models::Response {
        duration: format!("calculation single threaded took {:0.2} ms", duration),
        fractal,
    };
    let res = json(&response);

    info!("calculation single threaded took {:0.2} ms", duration);
    Ok(res)
}

pub async fn handle_request_multi_threaded(req: Request) -> utils::Result<impl Reply> {
    let (fractal, duration, cores) =
        calc_multi_threaded(&req.z1, &req.z2, req.width, req.max_iterations, req.colors);

    let response = models::Response {
        duration: format!(
            "calculation  multi_threaded using plain threads  took {:0.2} ms using {}",
            duration, cores
        ),
        fractal,
    };
    let res = json(&response);

    info!(
        "calculation multi_threaded  using plain threads  took {:0.2} ms",
        duration
    );
    Ok(res)
}

pub async fn handle_request_rayon(req: Request) -> utils::Result<impl Reply> {
    let (fractal, duration) =
        calc_rayon(&req.z1, &req.z2, req.width, req.max_iterations, req.colors);

    let response = models::Response {
        duration: format!("calculation  rayon threaded took {:0.2} ms", duration,),
        fractal,
    };
    let res = json(&response);

    info!(
        "calculation multi threaded with rayon took {:0.2} ms",
        duration
    );
    Ok(res)
}

async fn handle_request_crossbeam_tiles(ws: WebSocket) {
    let (mut websocket_tx, mut websocket_rx) = ws.split();

    // wait for a message, which contains infos about the scene
    let w = websocket_rx.next().await.unwrap();

    match w {
        Ok(msg) => {
            if msg.is_binary() {
                info!("got a binary message '{}'", msg.to_str().unwrap());
            } else if msg.is_ping() {
                info!("got a ping message '{}'", msg.to_str().unwrap());
            } else if msg.is_pong() {
                info!("got a ping message '{}'", msg.to_str().unwrap());
            } else if msg.is_text() {
                info!("got a text message '{}'", msg.to_str().unwrap());
            } else if msg.is_close() {
                info!("got a close message '{}'", msg.to_str().unwrap());
            } else {
                error!("got an undefined message")
            }
        }
        Err(e) => panic!("got an error from websocket {:?}", e),
    }

    let mut pixels = vec![];
    for _ in 0..10_000 {
        pixels.push(YELLOW);
    }
    let fractal = FractalImage {
        width: 100,
        height: 100,
        pixels,
    };

    let duration = "it took 23 ms".to_string();
    let res = Response { duration, fractal };
    let res = json!(res).to_string();
    let msg = Message::text(res);
    websocket_tx
        .send(msg)
        .unwrap_or_else(|e| {
            error!("websocket send error: {}", e);
        })
        .await;
}

async fn handle_request_crossbeam_tiles1(ws: WebSocket) {
    let (mut websocket_tx, mut websocket_rx) = ws.split();

    // wait for a message, which contains infos about the scene
    let w = websocket_rx.next().await.unwrap();

    match w {
        Ok(msg) => {
            info!("got a message  {:?}", msg.to_str());
            let request: Request = serde_json::from_str(msg.to_str().unwrap()).unwrap();
            info!("request {:?}", &request);

            let z1 = request.z1;
            let z2 = request.z2;
            let max_iterations = request.max_iterations;
            let colors = request.colors;
            let x_tiles = request.x_tiles;
            let y_tiles = request.y_tiles;
            let width = request.width;

            let (s, recv_web_sockets) = unbounded::<TileData>();

            let zz1 = z1.clone();
            let zz2 = z2.clone();

            tokio::task::spawn(async move {
                let start = Instant::now();
                calc_multi_threaded_crossbeam_tiles(
                    &zz1,
                    &zz2,
                    width,
                    max_iterations,
                    colors,
                    x_tiles,
                    y_tiles,
                    s,
                );
                let dur = start.elapsed().as_millis();
                info!(
                    "async handle_request_crossbeam_tiles  multi core duration: {} ms",
                    dur
                );
            });

            tokio::task::spawn(async move {
                let x_diff = z1.a.abs() + z2.a.abs();
                let y_diff = z1.b.abs() + z2.b.abs();

                // x_diff : y_diff = width : height
                // height = x_diff*width / y_diff
                let height = (x_diff * width as f32 / y_diff).round() as u32;

                // let mut cnt = 1;
                let mut fractal_image = FractalImage {
                    width,
                    height,
                    pixels: vec![Color::default(); (width * height) as usize],
                };

                loop {
                    let td = recv_web_sockets.recv();
                    match td {
                        Ok(tile_data) => {
                            info!("warp backend got a tile idx {}", tile_data.get_idx());

                            tile_data.get_points().iter().for_each(|p| {
                                fractal_image.pixels[p.get_y() * width as usize + p.get_x()] =
                                    p.get_color().clone();
                            });
                            let start = Instant::now();
                            let tile_data_json = json!(tile_data).to_string();
                            let dur = start.elapsed().as_millis();
                            info!("serialization took: {} ms", dur);
                            let start = Instant::now();
                            let msg = Message::text(tile_data_json);
                            let dur = start.elapsed().as_millis();
                            info!("wrapping in message took: {} ms", dur);
                            websocket_tx
                                .send(msg)
                                .unwrap_or_else(|e| {
                                    error!("websocket send error: {}", e);
                                })
                                .await;
                            // cnt += 1;
                        }
                        Err(e) => {
                            info!("no more tiles available: {}", e);
                            break;
                        }
                    }
                }
                save_png(
                    &fractal_image.pixels,
                    fractal_image.width,
                    fractal_image.height,
                );
            });
        }
        _ => {}
    }
}
