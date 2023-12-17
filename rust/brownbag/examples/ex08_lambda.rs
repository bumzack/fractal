use std::fmt::{Debug, Display, Formatter};

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let mut pixels = vec![];
    let cnt_pixels = 20;

    for idx in 0..cnt_pixels {
        let pixel = Pixel {
            r: idx,
            g: idx,
            b: idx,
        };
        pixels.push(pixel);
    }

    pixels.iter().for_each(|p| println!("pixel {}", p));
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "color r: {}, g: {}, b: {}", self.r, self.g, self.b)
    }
}
