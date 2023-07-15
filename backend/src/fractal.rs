use crate::color::{BLACK, Color};
use crate::complex::ComplexNumber;

pub fn calc_fractal_color(
    x: u16,
    y: u16,
    upper_left: &ComplexNumber,
    x_delta: f32,
    y_delta: f32,
    max_iterations: u16,
    colors: &Vec<Color>,
) -> Color {
    let mut cnt_iterations = 0;
    let c = ComplexNumber {
        a: upper_left.a + x as f32 * x_delta,
        b: upper_left.b - (y) as f32 * y_delta,
    };

    // info!("c = {}", &c);

    let mut z = ComplexNumber::default();
    while z.length_squared() < 4.0 && cnt_iterations < max_iterations {
        z = z.pow2() + &c;
        cnt_iterations += 1;
    }
    //info!("z = {}, c = {} ,  cnt_iterations {}, max_iterations {}", &z, &c, cnt_iterations, max_iterations);

    if cnt_iterations >= max_iterations {
        //  info!("BLACK       z = {}, c = {} ,  cnt_iterations {}, max_iterations {}", &z, &c, cnt_iterations, max_iterations);
        return BLACK;
    } else {
        let idx = cnt_iterations as usize % colors.len();
        let c: &Color = colors.get(idx).unwrap();
        //  info!("color    idx {}   z = {}, c = {} ,  cnt_iterations {}, max_iterations {}",idx, &z, &c, cnt_iterations, max_iterations);
        return c.clone();
    }
}

