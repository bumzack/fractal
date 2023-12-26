use crate::complex::ComplexNumber;
use crate::models::FractalRequest;

pub fn basic(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber { a: -0.8, b: 0.0 };

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 500_000;
    let mut zoom_factor = 1.01;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096 * 2;
    let mut height: u32 = 3072 * 2;

    if debug {
        zoom = 0.7;
        max_iterations = 10_000;
        zoom_factor = 1.2;
        max_zoom_factor = 50_000_000.0;

        width = 4096;
        height = 2160;
    }

    let complex_width = 3.1;

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
        name: "basic".to_string(),
    };

    (req, zoom_factor, max_zoom_factor)
}

pub fn flower(debug: bool) -> (FractalRequest, f64, f64) {
    let center = ComplexNumber {
        a: -1.999985881222,
        b: 0.0,
    };

    let mut zoom = 16969081.0;
    let mut max_iterations: u32 = 50_000_000;
    let mut zoom_factor = 1.01;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096 * 2;
    let mut height: u32 = 3072 * 2;

    if debug {
        zoom = 1.0;
        max_iterations = 10_000;
        zoom_factor = 1.5;
        max_zoom_factor = 50_000_000.0;

        width = 4096;
        height = 2160;
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
        a: -0.2262667110075,
        b: 1.11617444253,
    };

    let center = ComplexNumber {
        a: -0.22626671100753,
        b: 1.116174442537,
    };

    let center = ComplexNumber {
        a: -0.226266711007581,
        b: 1.1161744425361,
    };

    let center = ComplexNumber {
        a: -0.2262667110075811,
        b: 1.11617444253611,
    };

    let center = ComplexNumber {
        a: -0.2262667110075813,
        b: 1.11617444253613,
    };


    let center = ComplexNumber {
        a: -0.2262667110075814,
        b: 1.116174442536132,
    };


    // 6407226.562
    let center = ComplexNumber {
        a: -0.22626671100758141,
        b: 1.1161744425361321,
    };


    // 6407226.562

    let center = ComplexNumber {
        a: -0.22626671100758142,
        b: 1.1161744425361322,
    };

    // 6407226.562
    let center = ComplexNumber {
        a: -0.22626671100758146,
        b: 1.1161744425361325,
    };

    // 6407226.562
    let center = ComplexNumber {
        a: -0.22626671100758149,
        b: 1.1161744425361328,
    };


    // 6407226.562
    let center = ComplexNumber {
        a: -0.22626671100758155,
        b: 1.116174442536133,
    };


    let mut zoom = 1.0;
    let mut max_iterations: u32 = 500_000;
    let mut zoom_factor = 1.01;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096 * 2;
    let mut height: u32 = 3072 * 2;

    let complex_width = 4.1;

    let colors = 256;

    if debug {
        zoom = 250_000.0;
        max_iterations = 50_000;
        zoom_factor = 1.5;
        max_zoom_factor = 50_000_000.0;

        width = 4096 / 4;
        height = 2160 / 4;
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
    let mut max_iterations: u32 = 500_000;
    let mut zoom_factor = 1.01;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096 * 2;
    let mut height: u32 = 3072 * 2;

    let complex_width = 4.1;
    let colors = 256;

    if debug {
        zoom = 100000000.0;
        max_iterations = 10_000;
        zoom_factor = 1000000.2;
        max_zoom_factor = 50_000.0;

        width = 4096;
        height = 2160;
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
        a: -0.74351784,
        b: -0.127094578,
    };

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 500_000;
    let mut zoom_factor = 1.01;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096 * 2;
    let mut height: u32 = 3072 * 2;

    let complex_width = 4.1;
    let colors = 256;

    if debug {
        zoom = 1.0;
        max_iterations = 10_000;
        zoom_factor = 1.5;
        max_zoom_factor = 50_000_000.0;

        width = 4096;
        height = 2160;
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
    let mut max_iterations: u32 = 500_000;
    let mut zoom_factor = 1.01;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096 * 2;
    let mut height: u32 = 3072 * 2;

    let complex_width = 4.1;
    let colors = 256;

    if debug {
        zoom = 1.0;
        max_iterations = 10_000;
        zoom_factor = 1.2;
        max_zoom_factor = 50_000_000.0;

        width = 4096;
        height = 2160;
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
    let mut max_iterations: u32 = 500_000;
    let mut zoom_factor = 1.01;
    let mut max_zoom_factor = 50_000_000_000.0;

    let mut width: u32 = 4096 * 2;
    let mut height: u32 = 3072 * 2;

    let complex_width = 4.1;
    let colors = 256;

    if debug {
        zoom = 1.0;
        max_iterations = 10_000;
        zoom_factor = 1.2;
        max_zoom_factor = 50_000_000.0;

        width = 4096;
        height = 2160;
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
    let mut width: u32 = 4096 * 2;
    let mut height: u32 = 3072 * 2;
    let complex_width = 4.1;

    let mut zoom = 1.0;
    let mut max_iterations: u32 = 500_000;
    let mut zoom_factor = 1.01;
    let mut max_zoom_factor = 50_000_000_000.0;

    if debug {
        zoom = 1.0;
        max_iterations = 10_000;
        zoom_factor = 1.2;
        max_zoom_factor = 50_000_000.0;

        width = 4096;
        height = 2160;
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
