use common::fractal_calculation::calc_multi_threaded;
use std::time::Instant;

use common::models::FractalRequest;

fn main() {
    flower();
    tendrils();
    julia_island();
    seahorse_valley();
    sun();
    tree();
    starfish();
}

fn flower() {
    let (req, zoom_factor, max_zoom_factor) = common::fractal_templates::flower();
    render(req, zoom_factor, max_zoom_factor);
}

fn tendrils() {
    let (req, zoom_factor, max_zoom_factor) = common::fractal_templates::tendrils();
    render(req, zoom_factor, max_zoom_factor);
}

fn julia_island() {
    let (req, zoom_factor, max_zoom_factor) = common::fractal_templates::julia_island();
    render(req, zoom_factor, max_zoom_factor);
}

fn seahorse_valley() {
    let (req, zoom_factor, max_zoom_factor) = common::fractal_templates::seahorse_valley();
    render(req, zoom_factor, max_zoom_factor);
}

fn starfish() {
    let (req, zoom_factor, max_zoom_factor) = common::fractal_templates::starfish();
    render(req, zoom_factor, max_zoom_factor);
}

fn sun() {
    let (req, zoom_factor, max_zoom_factor) = common::fractal_templates::sun();
    render(req, zoom_factor, max_zoom_factor);
}

fn tree() {
    let (req, zoom_factor, max_zoom_factor) = common::fractal_templates::tree();
    render(req, zoom_factor, max_zoom_factor);
}

fn render(mut req: FractalRequest, zoom_factor: f64, max_zoom_factor: f64) {
    let width: u32 = 4096;
    let height: u32 = 2160;

    // -4.0 ... 1.3
    let complex_width = 5.3;

    let start = Instant::now();

    while req.zoom < max_zoom_factor {
        // 6591292.0
        let c_w = complex_width / req.zoom;
        let (_, duration, cores) = calc_multi_threaded(
            &req.center,
            c_w,
            req.zoom,
            width,
            height,
            req.max_iterations,
            256,
            req.name.to_string(),
        );

        println!(
            "name:  {} duration {duration},   cores {cores},     zoom {}",
            req.name, req.zoom
        );
        req.zoom = req.zoom * zoom_factor;
    }

    println!("rendering took {} seconds", start.elapsed().as_secs_f64());
}
