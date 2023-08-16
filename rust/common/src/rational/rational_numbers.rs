use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct RationalNumber {
    pub num: i128,
    pub denom: i128,
}

impl PartialEq<Self> for RationalNumber {
    fn eq(&self, other: &Self) -> bool {
        let divisor = gcd(self.num, self.denom);
        let a = self.num / divisor;
        let b = self.denom / divisor;

        let divisor = gcd(other.num, other.denom);
        let x = other.num / divisor;
        let y = other.denom / divisor;

        // let res = (a == x) && (b == y);
        // println!(
        //        "a = {}, b = {}    x = {}, y = {}      ( a == x) && (b == y)  {}",
        //        a, b, x, y, res
        //    );
        (a == x) && (b == y)
    }
}

impl PartialOrd<Self> for RationalNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let divisor = gcd(self.num, self.denom);
        let a = self.num / divisor;
        let b = self.denom / divisor;

        let divisor = gcd(other.num, other.denom);
        let x = other.num / divisor;
        let y = other.denom / divisor;

        let ordering = (x.pow(2) + y.pow(2)).cmp(&(a.pow(2) + b.pow(2)));

        // println!(
        //        "a = {}, b = {}    x = {}, y = {}      ordering  {:?}",
        //        a, b, x, y, ordering
        //    );

        Some(ordering)
    }
}

impl Add for RationalNumber {
    type Output = RationalNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let num = self.num * rhs.denom + self.denom * rhs.num;
        let denom = self.denom * rhs.denom;
        let divisor = gcd(num, denom);
        // println!("num  {}, denom  {}, divisor  {}", num, denom, divisor);

        RationalNumber {
            num: num / divisor,
            denom: denom / divisor,
        }
    }
}

impl Mul for RationalNumber {
    type Output = RationalNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        let num = self.num * rhs.num;
        let denom = self.denom * rhs.denom;
        let divisor = gcd(num, denom);

        RationalNumber {
            num: num / divisor,
            denom: denom / divisor,
        }
    }
}

pub fn gcd(mut a: i128, mut b: i128) -> i128 {
    // println!("a {}, b {}", a, b);
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
        // // println!("loop  a {}, b {}, remainder {}", a, b, remainder);
    }
    a
}

impl Add<&RationalNumber> for RationalNumber {
    type Output = RationalNumber;

    fn add(self, rhs: &Self) -> Self::Output {
        let num = self.num * rhs.denom + self.denom * rhs.num;
        let denom = self.denom * rhs.denom;
        let divisor = gcd(num, denom);
        // println!("num  {}, denom  {}, divisor  {}", num, denom, divisor);

        RationalNumber {
            num: num / divisor,
            denom: denom / divisor,
        }
    }
}

impl Sub<&RationalNumber> for RationalNumber {
    type Output = RationalNumber;

    fn sub(self, rhs: &Self) -> Self::Output {
        let num = self.num * rhs.denom - self.denom * rhs.num;
        let denom = self.denom * rhs.denom;
        let divisor = gcd(num, denom);
        // println!("num  {}, denom  {}, divisor  {}", num, denom, divisor);

        RationalNumber {
            num: num / divisor,
            denom: denom / divisor,
        }
    }
}

impl<'a, 'b> Add<&'b RationalNumber> for &'a RationalNumber {
    type Output = RationalNumber;

    fn add(self, rhs: &'b RationalNumber) -> Self::Output {
        // println!("self  {}, rhs  {}", self, rhs);

        let num = self.num * rhs.denom + self.denom * rhs.num;
        let denom = self.denom * rhs.denom;
        // println!("num  {}, denom  {}", num, denom);
        let divisor = gcd(num, denom);
        // println!("num  {}, denom  {}, divisor  {}", num, denom, divisor);

        RationalNumber {
            num: num / divisor,
            denom: denom / divisor,
        }
    }
}

impl<'a, 'b> Sub<&'b RationalNumber> for &'a RationalNumber {
    type Output = RationalNumber;

    fn sub(self, rhs: &'b RationalNumber) -> Self::Output {
        // println!("self  {}, rhs  {}", self, rhs);
        let num = self.num * rhs.denom - self.denom * rhs.num;
        let denom = self.denom * rhs.denom;
        // println!("num  {}, denom  {}", num, denom);
        let divisor = gcd(num, denom);
        // println!("num  {}, denom  {}, divisor  {}", num, denom, divisor);
        RationalNumber {
            num: num / divisor,
            denom: denom / divisor,
        }
    }
}

impl<'a, 'b> Mul<&'b RationalNumber> for &'a RationalNumber {
    type Output = RationalNumber;

    fn mul(self, rhs: &'b RationalNumber) -> Self::Output {
        let num = self.num * rhs.num;
        let denom = self.denom * rhs.denom;
        let divisor = gcd(num, denom);

        RationalNumber {
            num: num / divisor,
            denom: denom / divisor,
        }
    }
}
//
// impl Mul<&RationalNumber> for f64 {
//     type Output = RationalNumber;
//
//     fn mul(self, rhs: &RationalNumber) -> Self::Output {
//         let num = self * rhs.num;
//         let denom = self * rhs.denom;
//         let divisor = gcd(num, denom);
//
//         RationalNumber {
//             num: num / divisor,
//             denom: denom / divisor,
//         }
//     }
// }

// impl Mul<f64> for RationalNumber {
//     type Output = RationalNumber;
//
//     fn mul(self, rhs: f64) -> Self::Output {
//         let num = self.num * rhs;
//         let denom = self.denom * rhs;
//         let divisor = gcd(num, denom);
//
//         RationalNumber {
//             num: num / divisor,
//             denom: denom / divisor,
//         }
//     }
// }

impl<'a, 'b> Div<&'b RationalNumber> for &'a RationalNumber {
    type Output = RationalNumber;

    fn div(self, rhs: &'b RationalNumber) -> Self::Output {
        let num = self.num * rhs.denom;
        let denom = self.denom * rhs.num;
        let divisor = gcd(num, denom);

        RationalNumber {
            num: num / divisor,
            denom: denom / divisor,
        }
    }
}

// impl Div<f64> for RationalNumber {
//     type Output = RationalNumber;
//
//     fn div(self, rhs: f64) -> Self::Output {
//         let num = self.num / rhs;
//         let denom = self.denom / rhs;
//         let divisor = gcd(num, denom);
//
//         RationalNumber {
//             num: num / divisor,
//             denom: denom / divisor,
//         }
//     }
// }

impl RationalNumber {
    pub fn powi(&self, n: u32) -> RationalNumber {
        // z = z^n
        let num_powi = self.num.pow(n);
        let denom_powi = self.denom.pow(n);
        let divisor = gcd(num_powi, denom_powi);
        RationalNumber {
            num: num_powi / divisor,
            denom: denom_powi / divisor,
        }
    }
}

impl From<f64> for RationalNumber {
    fn from(value: f64) -> Self {
        let s = format!("{}", value);
        // println!("value {}.    value as string  '{}'", value, s);
        let x = s.find(".");
        if x.is_some() {
            let idx = x.unwrap();
            let (_, b) = s.split_at(idx);
            let cnt = b.len();
            let denom: f64 = 10_i128.pow(cnt as u32) as f64;
            let num = value * denom;
            // println!(                  "value {}.    value as string  '{}'    mul = {}   num = {}  ",                 value, s, denom, num             );

            let denom = denom as i128;
            let num = num as i128;
            let divisor = gcd(denom, num);
            RationalNumber {
                denom: denom / divisor,
                num: num / divisor,
            }
        } else {
            let denom = value as i128;
            let num: i128 = 1;
            RationalNumber {
                denom: denom,
                num: num,
            }
        }
    }
}

impl From<RationalNumber> for f64 {
    fn from(value: RationalNumber) -> Self {
        value.num as f64 / value.denom as f64
    }
}

impl Default for RationalNumber {
    fn default() -> Self {
        RationalNumber { num: 0, denom: 1 }
    }
}

impl Debug for RationalNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("x = ")
            .field("num", &self.num)
            .field("denom", &self.denom)
            .finish()
    }
}

impl Display for RationalNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / {} ", self.num, self.denom)
    }
}

#[cfg(test)]
mod tests {
    use crate::rational::rational_numbers::{gcd, RationalNumber};

    #[test]
    fn test_ggt_01() {
        let denom = 12;
        let num = 2;
        let expected = 2;

        let actual = gcd(denom, num);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_add_01() {
        let a = RationalNumber { num: 2, denom: 3 };
        let b = RationalNumber { num: 4, denom: 5 };

        let expected = RationalNumber { num: 22, denom: 15 };
        let actual = &a + &b;

        // println!("a = {},   b = {}", a, b);
        // println!("actual    = {}", actual);
        // println!("expected  = {}", expected);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_add_02() {
        let a = RationalNumber { num: -2, denom: 3 };
        let b = RationalNumber { num: 4, denom: 5 };

        let expected = RationalNumber { num: 2, denom: 15 };
        let actual = &a + &b;

        // println!("a = {},   b = {}", a, b);
        // println!("actual    = {}", actual);
        // println!("expected  = {}", expected);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_add_03() {
        let a = RationalNumber { num: -2, denom: 3 };
        let b = RationalNumber { num: -4, denom: 3 };

        let expected = RationalNumber { num: -2, denom: 1 };
        let actual = &a + &b;

        // println!("a = {},   b = {}", a, b);
        // println!("actual    = {}", actual);
        // println!("expected  = {}", expected);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mul_01() {
        let a = RationalNumber { num: 2, denom: 3 };
        let b = RationalNumber { num: 4, denom: 5 };

        let expected = RationalNumber { num: 8, denom: 15 };
        let actual = &a * &b;

        // println!("a = {},   b = {}", a, b);
        // println!("actual    = {}", actual);
        // println!("expected  = {}", expected);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mul_02() {
        let a = RationalNumber { num: 2, denom: -3 };
        let b = RationalNumber { num: 4, denom: 5 };

        let expected = RationalNumber { num: 8, denom: -15 };
        let actual = &a * &b;

        // println!("a = {},   b = {}", a, b);
        // println!("actual    = {}", actual);
        // println!("expected  = {}", expected);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_div_01() {
        let a = RationalNumber { num: 1, denom: 2 };
        let b = RationalNumber { num: 1, denom: 2 };

        let expected = RationalNumber { num: 1, denom: 4 };
        let actual = &a * &b;

        // println!("a = {},   b = {}", a, b);
        // println!("actual    = {}", actual);
        // println!("expected  = {}", expected);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_powi_01() {
        let a = RationalNumber { num: 2, denom: -3 };

        let expected = RationalNumber { num: 4, denom: 9 };
        let actual = a.powi(2);

        // println!("a         = {}", a);
        // println!("actual    = {}", actual);
        // println!("expected  = {}", expected);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_powi_02() {
        let a = RationalNumber { num: 2, denom: 4 };

        let expected = RationalNumber { num: 1, denom: 4 };
        let actual = a.powi(2);

        // println!("a         = {}", a);
        // println!("actual    = {}", actual);
        // println!("expected  = {}", expected);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_eq_01() {
        let a = RationalNumber { num: 2, denom: 4 };
        let b = RationalNumber { num: 3, denom: 6 };

        let expected = true;
        let actual = a == b;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_eq_02() {
        let a = RationalNumber { num: 3, denom: 4 };
        let b = RationalNumber { num: 3, denom: 6 };

        let expected = false;
        let actual = a == b;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ord_01() {
        let a = RationalNumber { num: 1, denom: 2 };
        let b = RationalNumber { num: 1, denom: 3 };

        // println!("a = {}", a);
        // println!("b = {}", b);
        let expected = true;
        let actual = a > b;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ord_02() {
        let a = RationalNumber { num: 1, denom: 3 };
        let b = RationalNumber { num: 2, denom: 6 };

        let expected = true;
        let actual = a == b;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ord_03() {
        let a = RationalNumber { num: -1, denom: 3 };
        let b = RationalNumber { num: 2, denom: 6 };

        let expected = true;
        let actual = a >= b;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ord_04() {
        let a = RationalNumber { num: -1, denom: 3 };
        let b = RationalNumber { num: 1, denom: 6 };

        let expected = false;
        let actual = a == b;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_ord_05() {
        let a = RationalNumber { num: 1, denom: 1 };
        let b = RationalNumber {
            num: 1_000_000,
            denom: 1_000_001,
        };

        let expected = false;
        let actual = a == b;

        assert_eq!(actual, expected);

        let expected = false;
        let actual = a < b;

        assert_eq!(actual, expected);

        let expected = false;
        let actual = a <= b;

        assert_eq!(actual, expected);
    }
}
