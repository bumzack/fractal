use crate::complex::ComplexNumber;
use crate::models::FractalRequest;

pub fn flower() -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -1.999985882,
        b: 0.0,
    };

    let zoom = 1.0;
    let max_iterations: u32 = 50000;
    let zoom_factor = 1.01;
    let max_zoom_factor = 50_000_000_000.0;

    let width: u32 = 4096;
    let height: u32 = 2160;

    let complex_width = 4.1;

    let colors = 256;

    let req = FractalRequest {
        center,
        width,
        height,
        complex_width,
        max_iterations,
        colors,
        x_tiles: 10,
        y_tiles: 10,
        zoom,
        name: "starfish".to_string(),
    };

    (req, zoom_factor, max_zoom_factor)
}

pub fn tendrils() -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -1.999985882,
        b: 0.0,
    };

    let zoom = 1.0;
    let max_iterations: u32 = 50000;
    let zoom_factor = 1.01;
    let max_zoom_factor = 50_000_000_000.0;

    let width: u32 = 4096;
    let height: u32 = 2160;

    let complex_width = 4.1;

    let colors = 256;

    let req = FractalRequest {
        center,
        width,
        height,
        complex_width,
        max_iterations,
        colors,
        x_tiles: 10,
        y_tiles: 10,
        zoom,

        name: "starfish".to_string(),
    };
    (req, zoom_factor, max_zoom_factor)
}

pub fn julia_island() -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -1.768778833,
        b: -0.001738996,
    };

    let zoom = 1.0;
    let max_iterations: u32 = 50_000;
    let zoom_factor = 1.01;
    let max_zoom_factor = 50_000_000_000.0;

    let width: u32 = 4096;
    let height: u32 = 2160;

    let complex_width = 4.1;
    let colors = 256;

    let req = FractalRequest {
        center,
        width,
        height,
        complex_width,
        max_iterations,
        colors,
        x_tiles: 10,
        y_tiles: 10,
        zoom,
        name: "starfish".to_string(),
    };
    (req, zoom_factor, max_zoom_factor)
}

pub fn seahorse_valley() -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -0.743517833,
        b: -0.127094578,
    };

    let zoom = 1.0;
    let max_iterations: u32 = 50_000;
    let zoom_factor = 1.01;
    let max_zoom_factor = 50_000_000_000.0;

    let width: u32 = 4096;
    let height: u32 = 2160;

    let complex_width = 4.1;
    let colors = 256;

    let req = FractalRequest {
        center,
        width,
        height,
        complex_width,
        max_iterations,
        colors,
        x_tiles: 10,
        y_tiles: 10,
        zoom,

        name: "starfish".to_string(),
    };
    (req, zoom_factor, max_zoom_factor)
}

pub fn starfish() -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -0.3740041393,
        b: 0.659792175,
    };

    let zoom = 1.0;
    let max_iterations: u32 = 50_000;
    let zoom_factor = 1.01;
    let max_zoom_factor = 50_000_000_000.0;

    let width: u32 = 4096;
    let height: u32 = 2160;

    let complex_width = 4.1;
    let colors = 256;

    let req = FractalRequest {
        center,
        width,
        height,
        complex_width,
        max_iterations,
        colors,
        x_tiles: 10,
        y_tiles: 10,
        zoom,

        name: "starfish".to_string(),
    };

    (req, zoom_factor, max_zoom_factor)
}

pub fn sun() -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -0.3740041393,
        b: 0.659792175,
    };

    let zoom = 1.0;
    let max_iterations: u32 = 50_000;
    let zoom_factor = 1.01;
    let max_zoom_factor = 50_000_000_000.0;

    let width: u32 = 4096;
    let height: u32 = 2160;

    let complex_width = 4.1;
    let colors = 256;

    let req = FractalRequest {
        center,
        width,
        height,
        complex_width,
        max_iterations,
        colors,
        x_tiles: 10,
        y_tiles: 10,
        zoom,
        name: "starfish".to_string(),
    };

    (req, zoom_factor, max_zoom_factor)
}

pub fn tree() -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -1.940157343,
        b: -1. / 1250000.0,
    };
    let width: u32 = 4096;
    let height: u32 = 2160;
    let complex_width = 4.1;

    let zoom = 1.0;
    let max_iterations: u32 = 50_000;
    let zoom_factor = 1.01;
    let max_zoom_factor = 50_000_000_000.0;

    let colors = 256;

    let req = FractalRequest {
        center,
        width,
        height,
        complex_width,
        max_iterations,
        colors,
        x_tiles: 10,
        y_tiles: 10,
        zoom,

        name: "starfish".to_string(),
    };

    (req, zoom_factor, max_zoom_factor)
}
