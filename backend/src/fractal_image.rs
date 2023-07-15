use std::fmt::{Debug, Display, Formatter};

use crate::color::Color;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct FractalImage {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) pixels: Vec<Color>,
}

impl Debug for FractalImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FractalImage = ")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

impl Display for FractalImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FractalImage: w =  {},  h  =  {}",
            self.width, self.height
        )
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
