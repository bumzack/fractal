use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};
use std::str::FromStr;

use crate::color::Color;

pub fn read_palette() -> HashMap<String, Vec<Color>> {
    let mut res = HashMap::new();

    let path = format!("{}/../../palette", env!("CARGO_MANIFEST_DIR"));
    let paths = read_dir(path).unwrap();

    for path in paths {
        let entry = path.unwrap();
        let buf = entry.path();
        let filename = buf.file_name().unwrap().to_str().unwrap();
        let p = buf.display().to_string();
//        println!("Name: {}", &p);
        if p.contains(".MAP") {
            let mut colors = vec![];
            for line in read_to_string(p).unwrap().lines() {
                let mut color_iter = line.split_whitespace();

                let r = color_iter.next();
                if r.is_some() {
                    let r = r.unwrap();
                    let number = r.parse::<u8>();

                    if number.is_ok() {
                        let r: String = r.chars().take(3).collect();
                        let r = u16::from_str(&r).unwrap();

                        let g = color_iter.next().unwrap();
                        let g: String = g.chars().take(3).collect();
  //                      println!("filename {filename} g {g}");
                        let g = u16::from_str(&g).unwrap();

                        let b = color_iter.next().unwrap();
                        let b: String = b.chars().take(3).collect();
    //                    println!("ilename {filename}  b {b}");
                        let b = u16::from_str(&b).unwrap();

                        let r = if r > 255 { 255 } else { r };
                        let g = if g > 255 { 255 } else { g };
                        let b = if b > 255 { 255 } else { b };

                        let c = Color {
                            r: r as u8,
                            g: g as u8,
                            b: b as u8,
                        };

                        colors.push(c);
                    }
      //              println!("filename {filename}  r {r}");
                }
            }
            res.insert(filename.to_string().to_lowercase(), colors);
        }
    }
    res
}
