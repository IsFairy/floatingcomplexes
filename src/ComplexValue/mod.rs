extern crate num_irrational;
extern crate num_rational;
extern crate num_traits;
extern crate serde;

use std::ops::{Add, AddAssign, Mul, MulAssign};

use num::ToPrimitive;
use num_irrational::{QuadraticSurd, FromSqrt};

#[derive(Debug, Clone, Copy, Hash, Eq)]
pub struct ComplexValue {
    r: QuadraticSurd<i64>,
    i: QuadraticSurd<i64>,
}

pub static Zero: ComplexValue = ComplexValue {
    r: QuadraticSurd::from(0),
    i: QuadraticSurd::from(0),
};
pub static One: ComplexValue = ComplexValue {
    r: QuadraticSurd::from(1),
    i: QuadraticSurd::from(0),
};
pub static Sqrt2_2: ComplexValue = ComplexValue {
    r: QuadraticSurd::from_sqrt(2i64).unwrap(),
    i: QuadraticSurd::from(0),
};

impl ComplexValue {
    fn new(r: QuadraticSurd<i64>, i: QuadraticSurd<i64>) -> ComplexValue {
        ComplexValue { r: r, i: i }
    }

    fn approximately_equals(self, other: ComplexValue) -> bool {
        self.r == other.r && self.i == other.i
    }

    fn approximately_zero(self) -> bool {
        self.r == QuadraticSurd::from(0i64) && self.i == QuadraticSurd::from(0i64)
    }
    fn approximately_one(self) -> bool {
        self.r == QuadraticSurd::from(1i64) && self.i == QuadraticSurd::from(0i64)
    }

    fn get_lowest_fraction(self) -> (i64, i64) {
        (
            self.r.to_rational().value().numer().clone(),
            self.r.to_rational().value().denom().to_owned().clone(),
        )
    }

    fn print_formatted(self) {
        println!("{} + {}i", self.r, self.i);
    }
}

impl PartialEq for ComplexValue {
    fn eq(&self, other: &ComplexValue) -> bool {
        self.r == other.r && self.i == other.i
    }

    fn ne(&self, other: &ComplexValue) -> bool {
        self.r != other.r || self.i != other.i
    }
}

impl PartialOrd for ComplexValue {
    fn partial_cmp(&self, other: &ComplexValue) -> Option<std::cmp::Ordering> {
        if self.r == other.r {
            self.i
                .to_rational()
                .value()
                .partial_cmp(&other.i.to_rational().value())
        } else {
            self.r
                .to_rational()
                .value()
                .partial_cmp(&other.r.to_rational().value())
        }
    }
}

impl Ord for ComplexValue {
    fn cmp(&self, other: &ComplexValue) -> std::cmp::Ordering {
        if self.r == other.r {
            self.i
                .to_rational()
                .value()
                .cmp(&other.i.to_rational().value())
        } else {
            self.r
                .to_rational()
                .value()
                .cmp(&other.r.to_rational().value())
        }
    }
}

impl num_traits::MulAdd for ComplexValue {
    type Output = ComplexValue;

    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        ComplexValue::new(
            self.r * a.r + self.i * a.i + b.r,
            self.r * a.i + self.i * a.r + b.i,
        )
    }
}

impl Add for ComplexValue {
    type Output = ComplexValue;

    fn add(self, other: Self) -> Self::Output {
        ComplexValue::new(self.r + other.r, self.i + other.i)
    }
}

impl AddAssign for ComplexValue {
    fn add_assign(&mut self, other: Self) {
        self.r = self.r + other.r;
        self.i = self.i + other.i;
    }
}

impl Mul for ComplexValue {
    type Output = ComplexValue;

    fn mul(self, other: Self) -> Self::Output {
        ComplexValue::new(self.r * other.r, self.i * other.i)
    }
}

impl MulAssign for ComplexValue {
    fn mul_assign(&mut self, other: Self) {
        self.r = self.r * other.r;
        self.i = self.i * other.i;
    }
}

impl num_traits::Zero for ComplexValue {
    fn zero() -> Self {
        ComplexValue::new(QuadraticSurd::from(0i64), QuadraticSurd::from(0i64))
    }

    fn is_zero(&self) -> bool {
        self.r == QuadraticSurd::from(0i64) && self.i == QuadraticSurd::from(0i64)
    }
}

impl num_traits::One for ComplexValue {
    fn one() -> Self {
        ComplexValue::new(QuadraticSurd::from(1i64), QuadraticSurd::from(0i64))
    }
}

impl ToString for ComplexValue {
    fn to_string(&self) -> String {
        format!("{} + {}i", self.r, self.i)
    }
}

impl From<String> for ComplexValue {
    fn from(s: String) -> Self {
        let mut split = s.split("+");
        let r = split.next().unwrap();
        let i = split.next().unwrap();
        let r = QuadraticSurd::from(r.parse::<i64>().unwrap());
        let i = QuadraticSurd::from(i.parse::<i64>().unwrap());
        ComplexValue::new(r, i)
    }
}

impl From<i64> for ComplexValue {
    fn from(i: i64) -> Self {
        ComplexValue::new(QuadraticSurd::from(i), QuadraticSurd::from(0i64))
    }
}

impl From<f64> for ComplexValue {
    fn from(f: f64) -> Self {
        ComplexValue::new(QuadraticSurd::from(f as i64), QuadraticSurd::from(0i64))
    }
}

impl From<f32> for ComplexValue {
    fn from(f: f32) -> Self {
        ComplexValue::new(QuadraticSurd::from(f as i64), QuadraticSurd::from(0i64))
    }
}

impl From<(f32, f32)> for ComplexValue {
    fn from(f: (f32, f32)) -> Self {
        ComplexValue::new(
            QuadraticSurd::from(f.0 as i64),
            QuadraticSurd::from(f.1 as i64),
        )
    }
}

impl From<(i64, i64)> for ComplexValue {
    fn from(i: (i64, i64)) -> Self {
        ComplexValue::new(QuadraticSurd::from(i.0), QuadraticSurd::from(i.1))
    }
}

impl Into<String> for ComplexValue {
    fn into(self) -> String {
        format!("{} + {}i", self.r, self.i)
    }
}

impl Into<(f32, f32)> for ComplexValue {
    fn into(self) -> (f32, f32) {
        (self.r.to_f32().unwrap(), self.i.to_f32().unwrap())
    }
}
