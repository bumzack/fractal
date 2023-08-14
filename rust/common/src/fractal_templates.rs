use crate::complex::ComplexNumber;
use crate::models::FractalRequest;

pub fn flower(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -1.999985882,
        b: 0.0,
    };

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 50_000;
    let mut zoom_factor = 1.1;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096;
    let mut height: u32 = 2160;

    if debug {
        zoom = 1.0;
        max_iterations = 1000;
        zoom_factor = 1.2;
        max_zoom_factor = 50_000_000.0;

        width = 800;
        height = 600;
    }

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
        name: "flower".to_string(),
    };

    (req, zoom_factor, max_zoom_factor)
}

pub fn tendrils(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -1.999985882,
        b: 0.0,
    };

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 50_000;
    let mut zoom_factor = 1.1;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096;
    let mut height: u32 = 2160;

    let complex_width = 4.1;

    let colors = 256;

    if debug {
        zoom = 1.0;
        max_iterations = 1000;
        zoom_factor = 1.2;
        max_zoom_factor = 50_000_000.0;

        width = 800;
        height = 600;
    }

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

        name: "tendrils".to_string(),
    };

    (req, zoom_factor, max_zoom_factor)
}

pub fn julia_island(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -1.768778833,
        b: -0.001738996,
    };

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 50_000;
    let mut zoom_factor = 1.1;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096;
    let mut height: u32 = 2160;

    let complex_width = 4.1;
    let colors = 256;

    if debug {
        zoom = 100000000.0;
        max_iterations = 10_000;
        zoom_factor = 1000000.2;
        max_zoom_factor = 50_000.0;

        width = 800;
        height = 600;
    }

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
        name: "julia_island".to_string(),
    };
    (req, zoom_factor, max_zoom_factor)
}

pub fn seahorse_valley(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -0.743517833,
        b: -0.127094578,
    };

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 50_000;
    let mut zoom_factor = 1.1;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096;
    let mut height: u32 = 2160;

    let complex_width = 4.1;
    let colors = 256;

    if debug {
        zoom = 1.0;
        max_iterations = 1000;
        zoom_factor = 1.2;
        max_zoom_factor = 50_000_000.0;

        width = 800;
        height = 600;
    }

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

        name: "seahorse_valley".to_string(),
    };
    (req, zoom_factor, max_zoom_factor)
}

pub fn starfish(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -0.3740041393,
        b: 0.659792175,
    };

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 50_000;
    let mut zoom_factor = 1.1;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096;
    let mut height: u32 = 2160;

    let complex_width = 4.1;
    let colors = 256;

    if debug {
        zoom = 1.0;
        max_iterations = 1000;
        zoom_factor = 1.2;
        max_zoom_factor = 50_000_000.0;

        width = 800;
        height = 600;
    }

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

pub fn sun(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -0.3740041393,
        b: 0.659792175,
    };

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 50_000;
    let mut zoom_factor = 1.1;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096;
    let mut height: u32 = 2160;

    let complex_width = 4.1;
    let colors = 256;

    if debug {
        zoom = 1.0;
        max_iterations = 1000;
        zoom_factor = 1.2;
        max_zoom_factor = 50_000_000.0;

        width = 800;
        height = 600;
    }

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
        name: "sun".to_string(),
    };

    (req, zoom_factor, max_zoom_factor)
}

pub fn tree(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -1.940157343,
        b: -1. / 1250000.0,
    };
    let mut width: u32 = 4096;
    let mut height: u32 = 2160;
    let complex_width = 4.1;

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 50_000;
    let mut zoom_factor = 1.1;
    let mut max_zoom_factor = 50_000_000_000.0;

    if debug {
        zoom = 1.0;
        max_iterations = 1000;
        zoom_factor = 1.2;
        max_zoom_factor = 50_000_000.0;

        width = 800;
        height = 600;
    }

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

        name: "tree".to_string(),
    };

    (req, zoom_factor, max_zoom_factor)
}
