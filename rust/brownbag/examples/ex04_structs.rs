struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let red = Pixel { r: 255, g: 0, b: 0 };

    println!("red =  ({} / {} / {})  ", red.r, red.g, red.b);
}
