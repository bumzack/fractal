use std::fs::create_dir_all;
use std::{fs::File, io::Write, time::Instant};

use chrono::Utc;
use serde::Serialize;
use serde_derive::Deserialize;
use serde_json::json;

use common::models::FractalRequest;
use common::{complex::ComplexNumber, fractal_calculation::calc_multi_threaded};

fn main() {
    flower(false);
    tendrils(false);
    seahorse_valley(false);
    sun(false);
    tree(false);
    starfish(false);
    julia_island(false);
}

fn flower(debug: bool) {
    let (req, zoom_factor, max_zoom_factor) = common::fractal_templates::flower(debug);
    render(req, zoom_factor, max_zoom_factor);
}

fn tendrils(debug: bool) {
    let (req, zoom_factor, max_zoom_factor) = common::fractal_templates::tendrils(debug);
    render(req, zoom_factor, max_zoom_factor);
}

fn julia_island(debug: bool) {
    let (req, zoom_factor, max_zoom_factor) = common::fractal_templates::julia_island(debug);
    render(req, zoom_factor, max_zoom_factor);
}

fn seahorse_valley(debug: bool) {
    let (req, zoom_factor, max_zoom_factor) = common::fractal_templates::seahorse_valley(debug);
    render(req, zoom_factor, max_zoom_factor);
}

fn starfish(debug: bool) {
    let (req, zoom_factor, max_zoom_factor) = common::fractal_templates::starfish(debug);
    render(req, zoom_factor, max_zoom_factor);
}

fn sun(debug: bool) {
    let (req, zoom_factor, max_zoom_factor) = common::fractal_templates::sun(debug);
    render(req, zoom_factor, max_zoom_factor);
}

fn tree(debug: bool) {
    let (req, zoom_factor, max_zoom_factor) = common::fractal_templates::tree(debug);
    render(req, zoom_factor, max_zoom_factor);
}

#[derive(Serialize, Deserialize)]
struct FractalData {
    req: FractalRequest,
    zoom_factor: f64,
    max_zoom_factor: f64,
    tl: ComplexNumber,
    br: ComplexNumber,
}

fn render(mut req: FractalRequest, zoom_factor: f64, max_zoom_factor: f64) {
    let width: u32 = req.width;
    let height: u32 = req.height;

    // -4.0 ... 1.3
    let complex_width = req.complex_width;

    let start = Instant::now();

    while req.zoom < max_zoom_factor {
        let c_w = complex_width / req.zoom;
        let (_, duration, cores) = calc_multi_threaded(
            &req.center,
            c_w,
            req.zoom,
            width,
            height,
            req.max_iterations,
            req.colors,
            req.name.to_string(),
        );

        println!(
            "name:  {} duration {duration},   cores {cores},     zoom {}",
            req.name, req.zoom
        );

        let ratio = width as f64 / height as f64;
        let complex_height = complex_width / ratio;

        let re_min = req.center.a - req.complex_width / 2.0;
        let re_max = req.center.a + req.complex_width / 2.0;

        let img_min = req.center.b - complex_height / 2.0;
        let img_max = req.center.b + complex_height / 2.0;

        let tl = ComplexNumber {
            a: re_min,
            b: img_max,
        };
        let br = ComplexNumber {
            a: re_max,
            b: img_min,
        };

        let fractal_data = FractalData {
            req: req.clone(),
            zoom_factor,
            max_zoom_factor,
            tl,
            br,
        };

        let json = serde_json::to_string_pretty(&json!(&fractal_data)).unwrap();

        let path = get_filename(&req.name);

        let mut f = File::create(path).expect("Unable to create file");
        f.write_all(json.as_bytes()).expect("Unable to write data");
        req.zoom *= zoom_factor;
    }

    println!("rendering took {} seconds", start.elapsed().as_secs_f64());
}

fn get_filename(name: &str) -> String {
    let now = Utc::now();

    let path = env!("CARGO_MANIFEST_DIR");
    // println!("CARGO_MANIFEST_DIR   {path}");
    let path = format!("{}/../../images/{}/json", path, name);
    create_dir_all(&path).expect("create dir should work");

    format!("{}/{}_{}.json", path, now.timestamp_millis(), name)
}
