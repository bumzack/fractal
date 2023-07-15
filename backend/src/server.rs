use std::time::Instant;
use log::info;
use warp::{Filter, Reply};
use warp::reply::json;
use crate::color::{Color, color16, color256};
use crate::fractal::calc_fractal_color;
use crate::fractal_image::FractalImage;
use crate::models::Request;
use crate::{models, utils};
use crate::utils::save_png;

pub fn routes() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
    let server_source = warp::path!("api" / "singlethreaded");
    let single_threaded = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|body: Request| {
            // info!("POST proxythingi/server/source");
            handle_request_single_threaded(body)
        });

    let server_source = warp::path!("api" / "multithreaded");
    let multi_threaded = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|body| {
            // info!("POST proxythingi/server/source");
            handle_request_single_threaded(body)
        });

    let server_source = warp::path!("api" / "rayon");
    let multi_threaded_rayon = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|body| {
            // info!("POST proxythingi/server/source");
            handle_request_single_threaded(body)
        });

    let server_source = warp::path!("api" / "crossbeamtiles");
    let multi_threaded_crossbeam_tiles = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|body| {
            // info!("POST proxythingi/server/source");
            handle_request_single_threaded(body)
        });

    single_threaded
        .or(multi_threaded)
        .or(multi_threaded_rayon)
        .or(multi_threaded_crossbeam_tiles)
}

pub async fn handle_request_single_threaded(req: Request) -> utils::Result<impl Reply> {
    let colors:Vec<Color> = match req.colors {
        16 =>color16(),
        256 => color256(),
        _ => panic!("number of colors not supported {}",  req.colors),
    };

    let start = Instant::now();

    let x_diff = req.z1.a.abs() + req.z2.a.abs();
    let y_diff = req.z1.b.abs() + req.z2.b.abs();

    // x_diff : y_diff = width : height
    // height = x_diff*width / y_diff
    let height = (x_diff * req.width as f32 / y_diff).round() as u16;

    let x_delta = x_diff / req.width as f32;
    let y_delta = y_diff / height as f32;

    info!("x_diff {},  y_diff {},   x_delta {},   y_delta {}   width {}  height {},  max_iterations {}",
    x_diff, y_diff, x_delta, y_delta, req.width,  height, req.max_iterations);

    let mut pixels = vec![];

    for y in 0..height {
        for x in 0..req.width {
            let p = calc_fractal_color(
                x,
                y,
                &req.z1,
                x_delta,
                y_delta,
                req.max_iterations,
                &colors ,
            );
            pixels.push(p);
        }
    }

    let duration = start.elapsed().as_millis();

    save_png(&pixels, req.width, height);

    let fractal = FractalImage {
        width: req.width,
        height,
        pixels,
    };
    let response = models::Response {
        duration: format!("calculation single threaded took {:0.2} ms", duration),
        fractal,
    };
    let res = json(&response);

    info!("calculation single threaded took {:0.2} ms", duration);
    Ok(res)
}

