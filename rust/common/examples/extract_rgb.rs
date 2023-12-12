use std::fs::File;
use std::io::Write;

use common::palette::read_palette;

fn main() {
    let palette = read_palette();
    let colors = palette.get("basic.map").unwrap().clone();

    let full_path = format!("{}/{}.rs", env!("CARGO_MANIFEST_DIR"), "colors");
    let mut file = File::create(full_path).expect("cant create file");

    file.write(format!("pub const PALETTE : Vec<Color> = vec! [   \n", ).as_ref())
        .expect("Unable to write pixel to file");

    colors.iter().for_each(|c| {
        file.write(
            format!(
                "Color {}  r:   {}, g: {},  b: {} {},   \n ",
                "{", c.r, c.g, c.b, "}"
            )
                .as_ref(),
        )
            .expect("Unable to write pixel to file");
    });

    file.write(format!("];   \n", ).as_ref())
        .expect("Unable to write pixel to file");
}
