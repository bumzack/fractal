use std::fs::File;
use std::io::Write;

use crate::fractal::FractalImage;

pub fn write_to_ppm(fractal_image: &FractalImage, filename: &str) {
    let full_path = format!("{}/{}.ppm", env!("CARGO_MANIFEST_DIR"), filename);
    let mut file = File::create(full_path).expect("cant create file");

    let data = "P3\n";
    let _ = file.write(data.as_ref()).expect("Unable to write file");
    let data = format!("{} {}\n", fractal_image.width, fractal_image.height);
    let _ = file.write(data.as_ref()).expect("Unable to write file");
    let data = format!("255\n");
    let _ = file.write(data.as_ref()).expect("Unable to write file");

    for y in 0..fractal_image.height {
        for x in 0..fractal_image.width {
            let idx = y * fractal_image.width + x;
            let _ = file
                .write(
                    format!(
                        "{} {} {} ",
                        fractal_image.pixels[idx].r,
                        fractal_image.pixels[idx].g,
                        fractal_image.pixels[idx].b
                    )
                    .as_ref(),
                )
                .expect("Unable to write pixel to file");

            // line should not be longer than 70 characters - "255 255 255 " = 12 characters
            if (idx + 1) % 5 == 0 {
                let _ = file
                    .write("\n".as_ref())
                    .expect("Unable to write pixel to file");
            }
        }
    }
}
