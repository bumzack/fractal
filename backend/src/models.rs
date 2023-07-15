use serde_derive::{Deserialize, Serialize};

use crate::complex::ComplexNumber;
use crate::fractal_image::FractalImage;

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct Request {
    pub z1: ComplexNumber,
    pub z2: ComplexNumber,
    pub width: u16,
    pub max_iterations: u16,
    pub colors: u16,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct Response {
    pub duration: String,
    pub fractal: FractalImage,
}
