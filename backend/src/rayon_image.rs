use std::fmt::{Debug, Display, Formatter};

use serde_derive::{Deserialize, Serialize};

use crate::color::Color;

#[derive(Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Pixel {
    pub(crate) color: Color,
    pub(crate) x: u32,
    pub(crate) y: u32,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct RayonImage {
    pub(crate) pixels: Vec<Pixel>,
}

impl Debug for RayonImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RayonImage = ")
            .field("count pixels ", &self.pixels.len())
            .finish()
    }
}

impl Display for RayonImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RayonImage: cnt_pixels=  {} ", self.pixels.len())
    }
}
