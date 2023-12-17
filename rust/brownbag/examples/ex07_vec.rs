use std::fmt::{Debug, Display, Formatter};

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let mut pixels = vec![];
    // let black = Pixel {
    //     r: 0,
    //     g: 0,
    //     b: 0,
    // };
    // let pixels2 = vec![black; 23];

    let cnt_pixels = 20;

    for idx in 0..cnt_pixels {
        let pixel = Pixel {
            r: idx,
            g: idx,
            b: idx,
        };
        pixels.push(pixel);
    }

    println!("pixels {:?}", pixels);
}

impl Debug for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "r: {}, g: {}, b: {}", self.r, self.g, self.b)
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "color r: {}, g: {}, b: {}", self.r, self.g, self.b)
    }
}
