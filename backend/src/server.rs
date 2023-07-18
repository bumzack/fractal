use std::time::Instant;

use common::color::Color;
use common::fractal_image::FractalImage;
use crossbeam_channel::unbounded;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use log::{error, info};
use serde_json::json;
use warp::reply::json;
use warp::ws::{Message, WebSocket};
use warp::{Filter, Reply};

use common::image_tile::TileData;
use common::models::{
    FractalRequest, FractalResponse, WebSocketCommand, WebSocketRequest, WebSocketResponse,
};

use crate::fractal::{
    calc_image_height, calc_multi_threaded, calc_multi_threaded_crossbeam_tiles, calc_rayon,
    calc_single_threaded,
};
use crate::utils;
use crate::utils::save_png;

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    let server_source = warp::path!("api" / "singlethreaded");
    let single_threaded = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|req: FractalRequest| {
            info!("POST api/singlethreaded");
            handle_request_single_threaded(req)
        });

    let server_source = warp::path!("api" / "multithreaded");
    let multi_threaded = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|req: FractalRequest| {
            info!("POST api/multithreaded");
            handle_request_multi_threaded(req)
        });

    let server_source = warp::path!("api" / "rayon");
    let multi_threaded_rayon = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|req: FractalRequest| {
            info!("POST api/rayon");
            handle_request_rayon(req)
        });

    let server_source = warp::path!("api" / "crossbeamtiles");
    let multi_threaded_crossbeam_tiles = server_source.and(warp::ws()).map(|ws: warp::ws::Ws| {
        info!("websocket api/crossbeamtiles");
        ws.on_upgrade(move |socket| handle_request_crossbeam_tiles(socket))
    });

    single_threaded
        .or(multi_threaded)
        .or(multi_threaded_rayon)
        .or(multi_threaded_crossbeam_tiles)
}

pub async fn handle_request_single_threaded(req: FractalRequest) -> utils::Result<impl Reply> {
    let (fractal, duration) =
        calc_single_threaded(&req.z1, &req.z2, req.width, req.max_iterations, req.colors);

    let response = FractalResponse {
        duration: format!("calculation single threaded took {:0.2} ms", duration),
        fractal,
    };
    let res = json(&response);

    info!("calculation single threaded took {:0.2} ms", duration);
    Ok(res)
}

pub async fn handle_request_multi_threaded(req: FractalRequest) -> utils::Result<impl Reply> {
    let (fractal, duration, cores) =
        calc_multi_threaded(&req.z1, &req.z2, req.width, req.max_iterations, req.colors);

    let response = FractalResponse {
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

pub async fn handle_request_rayon(req: FractalRequest) -> utils::Result<impl Reply> {
    let (fractal, duration) =
        calc_rayon(&req.z1, &req.z2, req.width, req.max_iterations, req.colors);

    let response = FractalResponse {
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
    let (sender_crossbeam_channel, recv_crossbeam_channel) = unbounded::<TileData>();

    match w {
        Ok(msg) => {
            if msg.is_text() {
                let txt = msg.to_str().unwrap();
                info!("got a text message '{}'", txt);

                let web_socket_request: serde_json::error::Result<WebSocketRequest> =
                    serde_json::from_str(txt);

                if let Ok(req) = web_socket_request {
                    info!("got a web_socket_request    {:?}", &req);

                    match req.command {
                        WebSocketCommand::GETHEIGHT(fractal_request) => {
                            let start = Instant::now();

                            info!(
                                "client wants to the height a web_socket_request    {:?}",
                                &fractal_request
                            );

                            let height = calc_image_height(
                                fractal_request.width,
                                &fractal_request.z1,
                                &fractal_request.z2,
                            );

                            let response = WebSocketResponse {
                                height: Some(height),
                                tile: None,
                            };
                            let json = serde_json::to_string(&response).unwrap();
                            let msg = Message::text(&json);
                            let dur = start.elapsed().as_millis();
                            info!("wrapping in message took: {} ms", dur);

                            websocket_tx
                                .send(msg)
                                .unwrap_or_else(|e| {
                                    error!("websocket send error: {}", e);
                                })
                                .await;
                        }
                        _ => panic!("not the right request"),
                    }
                };
            } else if msg.is_close() {
                info!("got a close message '{}'", msg.to_str().unwrap());
            } else {
                error!("got an undefined message")
            }
        }
        Err(err) => {
            panic!("not a valid request struct   err {}", err)
        }
    }

    let w = websocket_rx.next().await.unwrap();

    match w {
        Ok(msg) => {
            if msg.is_text() {
                let txt = msg.to_str().unwrap();
                info!("got a text message '{}'", txt);

                let web_socket_request: serde_json::error::Result<WebSocketRequest> =
                    serde_json::from_str(txt);

                if let Ok(req) = web_socket_request {
                    info!("got a web_socket_request    {:?}", &req);

                    match req.command {
                        WebSocketCommand::RENDERFRACTAL(fractal_request) => {
                            //let sender_crossbeam_channel = sender_crossbeam_channel.clone();
                            //let recv_crossbeam_channel = recv_crossbeam_channel.clone();
                            info!(
                                "client wants to the start rendering an image    {:?}",
                                &fractal_request
                            );

                            let re = fractal_request.clone();

                            let z1 = re.z1.clone();
                            let z2 = re.z2.clone();
                            let width = re.width;
                            let max_iterations = re.max_iterations;
                            let colors = re.colors;
                            let x_tiles = re.x_tiles;
                            let y_tiles = re.y_tiles;

                            //  tokio thread that calls amethod which produces the tiles
                            tokio::task::spawn(async move {
                                let start = Instant::now();
                                calc_multi_threaded_crossbeam_tiles(
                                    &z1,
                                    &z2,
                                    width,
                                    max_iterations,
                                    colors,
                                    x_tiles,
                                    y_tiles,
                                    sender_crossbeam_channel,
                                );
                                let dur = start.elapsed().as_millis();
                                info!(
                                    "async handle_request_crossbeam_tiles  multi core duration: {} ms",
                                      dur
                                );
                            });

                            // tokio task which collects all tiles, sends them to the client via the websocket sender
                            // and finally saves the fractal as PNG
                            tokio::task::spawn(async move {
                                let z1 = re.z1;
                                let z2 = re.z2;
                                let width = re.width;
                                let max_iterations = re.max_iterations;
                                let colors = re.colors;
                                let x_tiles = re.x_tiles;
                                let y_tiles = re.y_tiles;

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

                                while let Ok(tile_data) = recv_crossbeam_channel.recv() {
                                    info!("warp backend got a tile idx {}", tile_data.get_idx());

                                    tile_data.get_points().iter().for_each(|p| {
                                        fractal_image.pixels
                                            [p.get_y() * width as usize + p.get_x()] =
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

                                save_png(
                                    &fractal_image.pixels,
                                    fractal_image.width,
                                    fractal_image.height,
                                );
                            });
                        }
                        _ => panic!("not the right request"),
                    }
                };
            } else if msg.is_close() {
                info!("got a close message '{}'", msg.to_str().unwrap());
            } else {
                error!("got an undefined message")
            }
        }
        Err(err) => {
            panic!("not a valid request struct   err {}", err)
        }
    }
}
