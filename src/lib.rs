#![macro_use]
use std::fmt::Debug;

pub struct Fraction {
    num: usize,
    den: usize,
}

impl Fraction {
    pub const fn new(num: usize, den: usize) -> Self {
        Self { num, den }
    }
}

impl Debug for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

#[macro_export]
macro_rules! f {
    ($n:literal/$d:literal) => {
        Fraction::new($n, $d)
    };
}

pub fn step(start: usize, program: &[Fraction]) -> usize {
    for Fraction { num, den } in program {
        if start % den == 0 {
            return start / den * num;
        }
    }
    start
}
