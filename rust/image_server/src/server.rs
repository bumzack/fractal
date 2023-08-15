use std::fs::read_dir;

use chrono::{DateTime, Utc};
use log::info;
use warp::reply::json;
use warp::{Filter, Reply};

use common::imageserver::models::imageservermodels::{Image, Images};

use crate::utils;

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    let server_source = warp::path!("api" / "images");
    let images = server_source.and(warp::get()).and_then(|| {
        info!("GET  api/images ");
        get_images()
    });

    let path = format!("{}/images", env!("CARGO_MANIFEST_DIR"));
    // let path = "/images";
    info!("path  {}", path);

    // let server_source = warp::path!("static");
    let image =
        warp::path("static")
            .and(warp::fs::dir(path))
            .map(|reply: warp::filters::fs::File| {
                info!("GET  api/image   filename   {:?}", reply.path().as_os_str());
                reply.into_response()
            });

    images.or(image)
}

pub async fn get_images() -> utils::Result<impl Reply> {
    let path = format!("{}/images", env!("CARGO_MANIFEST_DIR"));
    let paths = read_dir(path).unwrap();
    let mut images = vec![];

    let server = "http://localhost:3100";
    let mut id = 1;
    for path in paths {
        let entry = path.unwrap();
        let buf = entry.path();
        let filename = buf.file_name().unwrap().to_str().unwrap();
        let p = buf.display().to_string();
        println!("Name: {}", &p);
        if p.contains(".png") {
            let systime = entry.metadata().unwrap().created().unwrap();
            let datetime: DateTime<Utc> = systime.into();
            let url = format!("{}/static/{}", server, filename);
            let image = Image {
                filename: filename.to_string(),
                prompt: filename.to_string(),
                created_at: datetime.to_rfc3339(),
                url,
                id,
            };
            id += 1;
            images.push(image);
        }
    }
    let images = Images { images };

    let res = json(&images);

    Ok(res)
}
