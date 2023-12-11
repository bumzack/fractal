use std::fmt::{Display, Formatter};

use crate::complex::ComplexNumber;
use crate::fractal::ColorEnum::{BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, WHITE, YELLOW};

pub const ASCII_BLACK_BACKGROUND: &str = "\x1b[1;39;40m";
pub const ASCII_RED_BACKGROUND: &str = "\x1b[1;31;41m";
pub const ASCII_GREEN_BACKGROUND: &str = "\x1b[1;32;42m";
pub const ASCII_YELLOW_BACKGROUND: &str = "\x1b[1;33;43m";
pub const ASCII_BLUE_BACKGROUND: &str = "\x1b[1;34;44m";
pub const ASCII_MAGENTA_BACKGROUND: &str = "\x1b[1;35;45m";
pub const ASCII_CYAN_BACKGROUND: &str = "\x1b[1;36;46m";
pub const ASCII_WHITE_BACKGROUND: &str = "\x1b[1;37;47m";
pub const ASCII_DEFAULT_BACKGROUND: &str = "\x1b[1;39;49m";
pub const ASCII_RESET_BACKGROUND: &str = "\x1b[1;0;0m";

#[derive(Debug, Clone, Copy)]
pub enum ColorEnum {
    BLACK,
    RED,
    GREEN,
    YELLOW,
    BLUE,
    MAGENTA,
    CYAN,
    WHITE,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub fn calc_color(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    max_iterations: usize,
) -> ColorEnum {
    let complex_width = 3.1;
    let zoom = 0.7;
    let complex_width = complex_width / zoom;
    let ratio = width as f64 / height as f64;
    let complex_height = complex_width / ratio;

    let center = ComplexNumber { a: -0.8, b: 0.0 };

    let re_min = center.a - complex_width / 2.0;
    let re_max = center.a + complex_width / 2.0;

    let img_min = center.b - complex_height / 2.0;
    let img_max = center.b + complex_height / 2.0;

    let x_delta = (re_max - re_min) / width as f64;
    let y_delta = (img_max - img_min) / height as f64;

    // println!("re_min {re_min}, re_max {re_max},  img_min {img_min}   img_max {img_max}  x_delta {x_delta}  y_delta  {y_delta} ");
    // println!("x_delta {},   y_delta {}   width {}  height {},  max_iterations {},  re_min {}, re_max {}, img_min {}, img_max {}" ,
    //     x_delta, y_delta, width,  height, max_iterations, re_min, re_max, img_min, img_max);

    let colors: Vec<ColorEnum> = vec![RED, GREEN, YELLOW, BLUE, MAGENTA, CYAN, WHITE];

    calc_fractal_color(
        x,
        y,
        re_min,
        img_min,
        x_delta,
        y_delta,
        max_iterations,
        &colors,
    )
}

pub fn calc_fractal_color(
    x: usize,
    y: usize,
    re_min: f64,
    img_min: f64,
    x_delta: f64,
    y_delta: f64,
    max_iterations: usize,
    colors: &Vec<ColorEnum>,
) -> ColorEnum {
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
        // println!(
        //     "BLACK       z = {}, c = {} ,  cnt_iterations {}, max_iterations {}",
        //     &z, &c, cnt_iterations, max_iterations
        // );
        BLACK
    } else {
        let idx = cnt_iterations % colors.len();
        let c: &ColorEnum = colors.get(idx).unwrap();
        // println!(
        //     "color    idx {}   z = {}, c = {:?} ,  cnt_iterations {}, max_iterations {}",
        //     idx, &z, c, cnt_iterations, max_iterations
        // );
        c.clone()
    }
}

impl Display for ColorEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            WHITE => write!(f, "{}", format!("{ASCII_WHITE_BACKGROUND}")),
            RED => write!(f, "{}", format!("{ASCII_RED_BACKGROUND}")),
            BLACK => write!(f, "{}", format!("{ASCII_BLACK_BACKGROUND}")),
            BLUE => write!(f, "{}", format!("{ASCII_BLUE_BACKGROUND}")),
            CYAN => write!(f, "{}", format!("{ASCII_CYAN_BACKGROUND}")),
            MAGENTA => write!(f, "{}", format!("{ASCII_MAGENTA_BACKGROUND}")),
            GREEN => write!(f, "{}", format!("{ASCII_GREEN_BACKGROUND}")),
            YELLOW => write!(f, "{}", format!("{ASCII_YELLOW_BACKGROUND}")),
        }
    }
}
