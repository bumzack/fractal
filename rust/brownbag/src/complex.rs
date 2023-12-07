use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;

#[derive(Clone, PartialEq)]
pub struct ComplexNumber {
    pub a: f64,
    pub b: f64,
}

impl Add for ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, rhs: Self) -> Self::Output {
        ComplexNumber {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl Add<&ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, rhs: &Self) -> Self::Output {
        ComplexNumber {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl ComplexNumber {
    pub fn pow2(&self) -> ComplexNumber {
        // z = a^2 + 2 i a b - b^2
        ComplexNumber {
            a: self.a.powi(2) - self.b.powi(2),
            b: 2.0 * self.a * self.b,
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.a.powi(2) + self.b.powi(2)
    }
}

impl Default for ComplexNumber {
    fn default() -> Self {
        ComplexNumber { a: 0.0, b: 0.0 }
    }
}

impl Debug for ComplexNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("z = ")
            .field("a", &self.a)
            .field("b", &self.b)
            .finish()
    }
}

impl Display for ComplexNumber {
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
