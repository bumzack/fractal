use std::cmp::Reverse;
use std::fs::{read_dir, DirEntry};

use chrono::{DateTime, NaiveDateTime, Utc};
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
        warp::path("images")
            .and(warp::fs::dir(path))
            .map(|reply: warp::filters::fs::File| {
                info!("GET  api/image   filename   {:?}", reply.path().as_os_str());
                reply.into_response()
            });

    let path = format!(
        "{}/../image_server_frontend/dist",
        env!("CARGO_MANIFEST_DIR")
    );
    // let path = "/images";
    info!("path  {}", path);
    let frontend =
        warp::path("static")
            .and(warp::fs::dir(path))
            .map(|reply: warp::filters::fs::File| {
                info!("GET  / frontend   filename  {:?}", reply.path().as_os_str());
                if reply.path().ends_with(".js") {
                    warp::reply::with_header(reply, "Content-Type", "text/javascript")
                        .into_response()
                } else {
                    reply.into_response()
                }
            });

    images.or(image).or(frontend)
}

pub async fn get_images() -> utils::Result<impl Reply> {
    let path = format!("{}/images", env!("CARGO_MANIFEST_DIR"));
    let mut paths: Vec<_> = read_dir(path).unwrap().map(|r| r.unwrap()).collect();

    paths.sort_by_key(|dir| Reverse(convert_created_to_u64(dir)));

    let mut images = vec![];

    let server = "http://varnish.bumzack.at";
    let mut id = 1;
    for path in paths {
        let buf = path.path();
        let filename = buf.file_name().unwrap().to_str().unwrap();
        let p = buf.display().to_string();

        let ts = convert_from_filename(filename) as i64;
        let naive = NaiveDateTime::from_timestamp_opt(ts, 0).expect("should be a date");
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

        // Format the datetime how you want
        let newdate = datetime.format("%H:%M:%S,%d.%m.%Y").to_string();

        println!("Name: {}       newdate {}", &p, &newdate);

        if p.contains(".png") {
            let url = format!("{}/images/{}", server, filename);
            let image = Image {
                filename: filename.to_string(),
                prompt: filename.to_string(),
                created_at: newdate,
                url,
                id,
                timestamp: convert_from_filename(filename),
            };
            println!("image {:?}", &image);
            id += 1;
            images.push(image);
        }
    }
    let images = Images { images };

    let res = json(&images);

    Ok(res)
}

fn convert_created_to_u64(dir: &DirEntry) -> u64 {
    let string = dir.file_name();
    let filename = string.to_str().unwrap();

    convert_from_filename(filename)
}

fn convert_from_filename(filename: &str) -> u64 {
    let idx = filename.find("_1");

    let timestamp = match idx {
        None => 0,
        Some(idx) => {
            let (_, timestamp) = filename.split_at(idx + 1);
            let idx = timestamp.find(".png").unwrap();
            let (timestamp, _) = timestamp.split_at(idx);
            let idx = timestamp.find(".").unwrap();
            let (timestamp, _) = timestamp.split_at(idx);
            let num: u64 = timestamp.parse().unwrap();
            println!(
                "Filename: {}    --->   timestamp string '{}'   timestamp as f64 {}",
                &filename, &timestamp, num
            );
            num
        }
    };

    println!("Filename: {}    --->   timestamp {}", &filename, &timestamp);

    timestamp
}
