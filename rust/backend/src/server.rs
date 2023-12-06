use std::time::Instant;

use crossbeam_channel::unbounded;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use log::{error, info};
use serde_json::json;
use warp::{Filter, Reply};
use warp::reply::json;
use warp::ws::{Message, WebSocket};

use common::color::Color;
use common::complex::ComplexNumber;
use common::fractal_calculation::{
    calc_multi_threaded, calc_multi_threaded_crossbeam_tiles, calc_rayon, calc_single_threaded,
};
use common::fractal_image::FractalImage;
use common::image_tile::TileData;
use common::models::{
    FractalRequest, FractalResponse, WebSocketCommand, WebSocketRequest, WebSocketResponse,
};
use common::utils::save_png2;

use crate::utils;

pub fn routes() -> impl Filter<Extract=(impl Reply, ), Error=warp::Rejection> + Clone {
    let server_source = warp::path!("api" / "singlethreaded");
    let single_threaded = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|req: FractalRequest| {
            info!("POST api/singlethreaded  req {:?}", &req);
            handle_request_single_threaded(req)
        });

    let server_source = warp::path!("api" / "multithreaded");
    let multi_threaded = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|req: FractalRequest| {
            info!("POST api/multithreaded  req {:?}", &req);
            handle_request_multi_threaded(req)
        });

    let server_source = warp::path!("api" / "rayon");
    let multi_threaded_rayon = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|req: FractalRequest| {
            info!("POST api/rayon.   req {:?}", &req);
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
    let (fractal, duration) = calc_single_threaded(
        &req.center,
        req.complex_width,
        req.zoom,
        req.width,
        req.height,
        req.max_iterations,
        req.colors,
        req.name,
    );

    let response = FractalResponse {
        duration_calculation: format!("calculation single threaded took {:0.2} ms", duration),
        fractal,
        duration_ms: duration,
    };
    let res = json(&response);

    info!("calculation single threaded took {:0.2} ms", duration);
    Ok(res)
}

pub async fn handle_request_multi_threaded(req: FractalRequest) -> utils::Result<impl Reply> {
    let (fractal, duration, cores) = calc_multi_threaded(
        &req.center,
        req.complex_width,
        req.zoom,
        req.width,
        req.height,
        req.max_iterations,
        req.colors,
        req.name,
    );

    let response = FractalResponse {
        duration_calculation: format!(
            "calculation  multi_threaded using plain threads  took {:0.2} ms using {} cores",
            duration, cores
        ),
        fractal,
        duration_ms: duration,
    };
    let res = json(&response);

    info!(
        "calculation multi_threaded  using plain threads  took {:0.2} ms",
        duration
    );
    Ok(res)
}

pub async fn handle_request_rayon(req: FractalRequest) -> utils::Result<impl Reply> {
    let (fractal, duration) = calc_rayon(
        &req.center,
        req.complex_width,
        req.zoom,
        req.width,
        req.height,
        req.max_iterations,
        req.colors,
        req.name,
    );

    let response = FractalResponse {
        duration_calculation: format!("calculation  rayon threaded took {:0.2} ms", duration, ),
        fractal,
        duration_ms: duration,
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

    //let w = websocket_rx.next().await.unwrap();

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

                            //  tokio thread that calls a method which produces the tiles
                            tokio::task::spawn(async move {
                                let start = Instant::now();
                                calc_multi_threaded_crossbeam_tiles(
                                    &re.center,
                                    re.complex_width,
                                    re.zoom,
                                    re.width,
                                    re.height,
                                    re.max_iterations,
                                    re.colors,
                                    re.name.clone(),
                                    re.x_tiles,
                                    re.y_tiles,
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
                                let re = fractal_request.clone();

                                // let mut cnt = 1;
                                let mut fractal_image = FractalImage {
                                    width: re.width,
                                    height: re.height,
                                    pixels: vec![Color::default(); (re.width * re.height) as usize],
                                };

                                while let Ok(tile_data) = recv_crossbeam_channel.recv() {
                                    info!("warp backend got a tile idx {}", tile_data.get_idx());

                                    tile_data.get_points().iter().for_each(|p| {
                                        let idx = (p.get_y() * re.width + p.get_x()) as usize;
                                        fractal_image.pixels[idx] = p.get_color().clone();
                                    });
                                    let start = Instant::now();
                                    let websocket_response = WebSocketResponse {
                                        tile: Some(tile_data),
                                    };
                                    let tile_data_json = json!(websocket_response).to_string();
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
                                let tl = ComplexNumber::default();
                                let br = ComplexNumber::default();

                                save_png2(
                                    &fractal_image.pixels,
                                    fractal_image.width,
                                    fractal_image.height,
                                    &re.center,
                                    &tl,
                                    &br,
                                    re.zoom,
                                    re.max_iterations,
                                    re.name,
                                );
                            });
                        }
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
