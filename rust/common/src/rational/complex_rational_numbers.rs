use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;

use serde_derive::{Deserialize, Serialize};

use crate::rational::rational_numbers::RationalNumber;

#[derive(Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct ComplexRationalNumber {
    pub a: RationalNumber,
    pub b: RationalNumber,
}

impl Add for ComplexRationalNumber {
    type Output = ComplexRationalNumber;

    fn add(self, rhs: Self) -> Self::Output {
        ComplexRationalNumber {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl Add<&ComplexRationalNumber> for ComplexRationalNumber {
    type Output = ComplexRationalNumber;

    fn add(self, rhs: &Self) -> Self::Output {
        ComplexRationalNumber {
            a: self.a + rhs.clone().a,
            b: self.b + rhs.clone().b,
        }
    }
}

impl ComplexRationalNumber {
    pub fn pow2(&self) -> ComplexRationalNumber {
        // z = a^2 + 2 i a b - b^2
        let two = RationalNumber { num: 2, denom: 1 };
        ComplexRationalNumber {
            a: self.a.powi(2) - &self.b.powi(2),
            b: self.a.clone() * self.b.clone() * two,
        }
    }

    pub fn length_squared(&self) -> RationalNumber {
        self.a.powi(2) + self.b.powi(2)
    }
}

impl Default for ComplexRationalNumber {
    fn default() -> Self {
        ComplexRationalNumber {
            a: RationalNumber::default(),
            b: RationalNumber::default(),
        }
    }
}

impl Debug for ComplexRationalNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("z = ")
            .field("a", &self.a)
            .field("b", &self.b)
            .finish()
    }
}

impl Display for ComplexRationalNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "z = {} + {} * i", self.a, self.b)
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
