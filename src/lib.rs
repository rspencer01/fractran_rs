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

pub struct FractranProgram<'a> {
    start: usize,
    instructions: &'a [Fraction],
}

impl<'a> FractranProgram<'a> {
    pub const fn new(start: usize, instructions: &'a [Fraction]) -> Self {
        Self {
            start,
            instructions,
        }
    }
}

#[macro_export]
macro_rules! fractran {
    ($s:literal | $( $n:literal/$d:literal) *) => {
        FractranProgram::new($s,&[
          $(
              Fraction{ num:$n, den:$d},
          )*
        ])
    };
}

impl<'a> Iterator for FractranProgram<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = 'nxt: {
            for Fraction { num, den } in self.instructions {
                if self.start % den == 0 {
                    break 'nxt Some(self.start / den * num);
                }
            }
            None
        };
        self.start = next.unwrap_or(self.start);
        next
    }
}
