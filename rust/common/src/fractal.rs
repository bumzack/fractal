use crate::color::{BLACK, Color};
use crate::complex::ComplexNumber;

pub fn calc_fractal_color(
    x: u32,
    y: u32,
    re_min: f64,
    img_min: f64,
    x_delta: f64,
    y_delta: f64,
    max_iterations: u32,
    colors: &Vec<Color>,
) -> Color {
    let mut cnt_iterations = 0;
    let c = ComplexNumber {
        a: re_min + x as f64 * x_delta,
        b: img_min + (y) as f64 * y_delta,
    };

    let mut z = ComplexNumber::default();
    while z.length_squared() < 4.0 && cnt_iterations < max_iterations {
        z = z.pow2() + &c;
        cnt_iterations += 1;
    }
    //info!("z = {}, c = {} ,  cnt_iterations {}, max_iterations {}", &z, &c, cnt_iterations, max_iterations);

    if cnt_iterations >= max_iterations {
        //  info!("BLACK       z = {}, c = {} ,  cnt_iterations {}, max_iterations {}", &z, &c, cnt_iterations, max_iterations);
        BLACK
    } else {
        let idx = cnt_iterations as usize % colors.len();
        let c: &Color = colors.get(idx).unwrap();
        //  info!("color    idx {}   z = {}, c = {} ,  cnt_iterations {}, max_iterations {}",idx, &z, &c, cnt_iterations, max_iterations);
        c.clone()
    }
}

pub fn calc_fractal_color2(
    x: u32,
    y: u32,
    re_min: f64,
    img_min: f64,
    x_delta: f64,
    y_delta: f64,
    max_iterations: u32,
    colors: &Vec<Color>,
    pixel: &mut Color,
) {
    let mut cnt_iterations = 0;
    let c = ComplexNumber {
        a: re_min + x as f64 * x_delta,
        b: img_min + (y) as f64 * y_delta,
    };

    let mut z = ComplexNumber::default();
    while z.length_squared() < 4.0 && cnt_iterations < max_iterations {
        z = z.pow2() + &c;
        cnt_iterations += 1;
    }
    //info!("z = {}, c = {} ,  cnt_iterations {}, max_iterations {}", &z, &c, cnt_iterations, max_iterations);

    if cnt_iterations >= max_iterations {
        //  info!("BLACK       z = {}, c = {} ,  cnt_iterations {}, max_iterations {}", &z, &c, cnt_iterations, max_iterations);
        pixel.r = BLACK.r;
        pixel.g = BLACK.g;
        pixel.b = BLACK.b;
    } else {
        let idx = cnt_iterations as usize % colors.len();
        let c: &Color = colors.get(idx).unwrap();
        //  info!("color    idx {}   z = {}, c = {} ,  cnt_iterations {}, max_iterations {}",idx, &z, &c, cnt_iterations, max_iterations);
        pixel.r = c.r;
        pixel.g = c.g;
        pixel.b = c.b;
    }
}
