use chrono::Utc;
use log::{error, info};
use tokio::time::Instant;
use warp::Rejection;
use crate::color::Color;
use image::ImageBuffer;
use image::RgbImage;

pub type Result<T> = std::result::Result<T, Rejection>;

pub fn save_png(pixels: &Vec<Color>, width: u16, height: u16) {
    let start = Instant::now();
    let mut x = 0;
    let mut y = 0;
    let mut image: RgbImage = ImageBuffer::new(width as u32, height as u32);

    for p in pixels.iter() {
        let pixel = image::Rgb([p.r, p.g, p.b]);
        // println!("pixels_vec = {:?}, pixel = {:?}", p, pixel);
        image.put_pixel(x as u32, y as u32, pixel);
        x += 1;
        if x % width == 0 {
            y += 1;
            x = 0;
        }
    }
    let now = Utc::now();
    let filename = format!(
        "fractal_{}_{}_{}.png",
        width,
        height,
        now.timestamp_millis()
    );
    let res = image.save(filename);
    let duration =  start.elapsed().as_millis();
    match res {
        Ok(_) => info!("save ok. took {} ms", duration),
        Err(e) => error!("error saving file {}. took {} ms", e,duration),
    }
}


pub fn cors() -> warp::cors::Builder {
    warp::cors()
        .allow_any_origin()
        .expose_headers(vec![
            "x-duration",
            "x-provided-by",
            "x-initiated-by",
            "x-processed-by",
        ])
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "content-type",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Access-Control-Allow-Headers",
            "Access-Control-Allow-Methods",
            "Access-Control-Allow-Origin",
            "Access-Control-Expose-Headers",
            "Access-Control-Request-Headers",
            "Access-Control-Request-Methods",
            "Accept-Encoding",
            "Accept-Language",
            "Accept-Post",
            "Access-Control-Allow-Credentials",
            "keep-alive",
            "x-duration",
            "x-provided-by",
            "x-initiated-by",
            "x-processed-by",
        ])
        .allow_methods(vec!["POST", "GET", "OPTIONS", "PUT", "DELETE", "HEAD"])
}
