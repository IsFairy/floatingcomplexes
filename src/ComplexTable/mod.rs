use super::ComplexValue::*;


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
            value: Zero,
            next: None,
            ref_count: 0,
        });
        let one = Box::new(Entry {
            value: One,
            next: None,
            ref_count: 0,
        });
        let sqrt2_2 = Box::new(Entry {
            value: Sqrt2_2,
            next: None,
            ref_count: 0,
        });
        ComplexTable {
            Zero: zero,
            One: one,
            sqrt2_2: sqrt2_2,
        }
    }

    pub fn get(&mut self, value: ComplexValue) -> Box<Entry> {
        match value {
            ComplexValue::Zero => self.Zero,
            ComplexValue::One => self.One,
            ComplexValue::Sqrt2_2 => self.sqrt2_2,
        }
    }
}