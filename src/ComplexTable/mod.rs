use super::ComplexValue::*;
use num_irrational::{FromSqrt, QuadraticSurd};

pub struct Entry {
    pub value: ComplexValue,
    pub next: Option<Box<Entry>>,
    pub ref_count: usize,
}

pub struct ComplexTable {
    pub Zero: Box<Entry>,
    pub One: Box<Entry>,
    pub sqrt2_2: Box<Entry>,
}

impl ComplexTable {
    pub fn new() -> ComplexTable {
        let zero = Box::new(Entry {
            value: ComplexValue::from(0i64),
            next: None,
            ref_count: 0,
        });
        let one = Box::new(Entry {
            value: ComplexValue::from(1i64),
            next: None,
            ref_count: 0,
        });
        let sqrt2_2 = Box::new(Entry {
            value: ComplexValue::new(
                QuadraticSurd::from_sqrt(2i64).unwrap(),
                QuadraticSurd::from(0i64),
            ),
            next: None,
            ref_count: 0,
        });
        ComplexTable {
            Zero: zero,
            One: one,
            sqrt2_2: sqrt2_2,
        }
    }

}
