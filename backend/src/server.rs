use log::info;
use warp::reply::json;
use warp::{Filter, Reply};

use crate::fractal::{calc_multi_threaded, calc_single_threaded};
use crate::models::Request;
use crate::{models, utils};

pub fn routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
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
            handle_request_single_threaded(req)
        });

    let server_source = warp::path!("api" / "crossbeamtiles");
    let multi_threaded_crossbeam_tiles = server_source
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|req| {
            info!("POST api/crossbeamtiles");
            handle_request_single_threaded(req)
        });

    single_threaded
        .or(multi_threaded)
        .or(multi_threaded_rayon)
        .or(multi_threaded_crossbeam_tiles)
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
            "calculation  multi_threaded threaded took {:0.2} ms using {}",
            duration, cores
        ),
        fractal,
    };
    let res = json(&response);

    info!("calculation single threaded took {:0.2} ms", duration);
    Ok(res)
}
