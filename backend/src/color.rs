use std::fmt::{Debug, Display, Formatter};
use std::fs;

use serde_derive::{Deserialize, Serialize};

pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };

pub const MAROON: Color = Color { r: 128, g: 0, b: 0 };

pub const GREEN: Color = Color { r: 0, g: 128, b: 0 };

pub const OLIVE: Color = Color {
    r: 128,
    g: 128,
    b: 0,
};

pub const NAVY: Color = Color { r: 0, g: 0, b: 128 };
pub const PURPLE: Color = Color {
    r: 128,
    g: 0,
    b: 128,
};

pub const TEAL: Color = Color {
    r: 0,
    g: 128,
    b: 128,
};

pub const SILVER: Color = Color {
    r: 128,
    g: 128,
    b: 0,
};

pub const GRAY: Color = Color {
    r: 128,
    g: 128,
    b: 128,
};

pub const RED: Color = Color { r: 255, g: 0, b: 0 };

pub const LIME: Color = Color { r: 0, g: 255, b: 0 };

pub const YELLOW: Color = Color {
    r: 255,
    g: 255,
    b: 0,
};

pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };

pub const FUCHSIA: Color = Color {
    r: 255,
    g: 0,
    b: 255,
};

pub const AQUA: Color = Color {
    r: 0,
    g: 255,
    b: 255,
};

pub const WHITE: Color = Color {
    r: 0,
    g: 255,
    b: 255,
};

#[derive(Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FileColor {
    pub(crate) color_id: u8,
    pub(crate) hex_string: String,
    pub(crate) rgb: Color,
    pub(crate) hsl: Hsl,
    pub(crate) name: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Hsl {
    pub(crate) h: f32,
    pub(crate) s: f32,
    pub(crate) l: f32,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

pub fn color256() -> Vec<Color> {
    let data = fs::read_to_string("./256-colors.json").expect("Unable to read file");
    let colors: Vec<FileColor> = serde_json::from_str(&data).expect("JSON was not well-formatted");
    let cs: Vec<Color> = colors
        .iter()
        .map(|c| Color {
            r: c.rgb.r,
            g: c.rgb.g,
            b: c.rgb.b,
        })
        .collect();

    // cs.iter().for_each(|c| info!("color  {}", &c));
    cs
}

pub fn color16() -> Vec<Color> {
    let mut v = vec![];
    v.push(BLACK);
    v.push(MAROON);
    v.push(GREEN);
    v.push(OLIVE);
    v.push(NAVY);
    v.push(PURPLE);
    v.push(TEAL);
    v.push(SILVER);
    v.push(GRAY);
    v.push(RED);
    v.push(LIME);
    v.push(YELLOW);
    v.push(BLUE);
    v.push(FUCHSIA);
    v.push(AQUA);
    v.push(WHITE);
    v
}

impl Default for Color {
    fn default() -> Self {
        Color { r: 0, b: 0, g: 0 }
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("color = ")
            .field("r", &self.r)
            .field("g", &self.g)
            .field("b", &self.b)
            .finish()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "color = ( {} / {} / {} )", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
