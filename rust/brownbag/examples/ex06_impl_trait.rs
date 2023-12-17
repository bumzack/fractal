use std::fmt::{Debug, Display, Formatter};

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let red = Pixel { r: 255, g: 0, b: 0 };

    println!("red =  {:?}  ", red);
    println!("red =  {red}  ");
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
