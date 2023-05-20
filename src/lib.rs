#![macro_use]
use std::fmt::Debug;

pub struct Fraction {
    pub num: usize,
    pub den: usize,
}

impl Debug for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

#[macro_export]
macro_rules! fractran {
    ($( $n:literal/$d:literal) *) => {
        [
          $(
              Fraction{ num:$n, den:$d},
          )*
        ]
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
