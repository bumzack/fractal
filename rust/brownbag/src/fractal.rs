use std::fmt::{Display, Formatter};

use crate::complex::ComplexNumber;

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

pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
pub const RED: Color = Color { r: 255, g: 0, b: 0 };
pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
pub const YELLOW: Color = Color {
    r: 255,
    g: 255,
    b: 0,
};
pub const MAGENTA: Color = Color {
    r: 255,
    g: 0,
    b: 255,
};
pub const CYAN: Color = Color {
    r: 0,
    g: 255,
    b: 255,
};
pub const WHITE: Color = Color { r: 0, g: 0, b: 255 };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn calc_color(x: usize, y: usize, width: usize, height: usize, max_iterations: usize) -> Color {
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
    let colors: Vec<Color> = vec![WHITE, CYAN, MAGENTA, BLUE, YELLOW, GREEN, RED];

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

    if cnt_iterations >= max_iterations {
        BLACK
    } else {
        let idx = cnt_iterations % colors.len();
        colors.get(idx).unwrap().clone()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            WHITE => write!(f, "{}", format!("{ASCII_WHITE_BACKGROUND}")),
            RED => write!(f, "{}", format!("{ASCII_RED_BACKGROUND}")),
            BLACK => write!(f, "{}", format!("{ASCII_BLACK_BACKGROUND}")),
            CYAN => write!(f, "{}", format!("{ASCII_CYAN_BACKGROUND}")),
            MAGENTA => write!(f, "{}", format!("{ASCII_MAGENTA_BACKGROUND}")),
            GREEN => write!(f, "{}", format!("{ASCII_GREEN_BACKGROUND}")),
            YELLOW => write!(f, "{}", format!("{ASCII_YELLOW_BACKGROUND}")),
            BLUE => write!(f, "{}", format!("{ASCII_BLUE_BACKGROUND}")),
            _ => write!(f, "{}", format!("{ASCII_WHITE_BACKGROUND}")),
        }
    }
}
