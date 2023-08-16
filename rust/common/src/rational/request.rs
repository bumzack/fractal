use serde_derive::{Deserialize, Serialize};

use crate::rational::complex_rational_numbers::ComplexRationalNumber;
use crate::rational::rational_numbers::RationalNumber;

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct FractalRequestRational {
    pub center: ComplexRationalNumber,
    pub width: u32,
    pub height: u32,
    pub complex_width: RationalNumber,
    pub max_iterations: u32,
    pub colors: u32,
    pub x_tiles: u32,
    pub y_tiles: u32,
    pub zoom: RationalNumber,
    pub name: String,
}
