#![macro_use]

pub struct Fraction {
    pub num: usize,
    pub den: usize,
}

pub struct FractranProgram<'a> {
    instructions: &'a [Fraction],
}

pub struct FractranRun<'a> {
    current: usize,
    program: &'a FractranProgram<'a>,
}

impl<'a> FractranProgram<'a> {
    pub const fn new(instructions: &'a [Fraction]) -> Self {
        Self { instructions }
    }

    pub fn start(&'a self, start: usize) -> FractranRun<'a> {
        FractranRun {
            current: start,
            program: self,
        }
    }

    pub fn run(&self, start: usize) -> usize {
        self.start(start).last().unwrap()
    }
}

#[macro_export]
macro_rules! fractran {
    ($( $n:literal/$d:literal) *) => {
        FractranProgram::new(&[$(Fraction{ num:$n, den:$d},)*])
    };
}

impl<'a> Iterator for FractranRun<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = 'nxt: {
            for Fraction { num, den } in self.program.instructions {
                if self.current % den == 0 {
                    break 'nxt Some(self.current / den * num);
                }
            }
            None
        };
        self.current = next.unwrap_or(self.current);
        next
    }
}
