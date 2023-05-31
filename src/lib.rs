#![no_std]
#![macro_use]

use core::{iter::FusedIterator, marker::PhantomData};

#[derive(Copy, Clone)]
pub struct Fraction {
    pub num: usize,
    pub den: usize,
}

pub struct FractranProgram<'a, I = usize, O = usize>
where
    I: Into<usize>,
    O: From<usize>,
{
    instructions: &'a [Fraction],
    _phantom: PhantomData<(I, O)>,
}

pub struct FractranRun<'a, I, O>
where
    I: Into<usize>,
    O: From<usize>,
{
    current: usize,
    program: &'a FractranProgram<'a, I, O>,
}

impl<'a, I: Into<usize>, O: From<usize>> FractranProgram<'a, I, O> {
    pub const fn new(instructions: &'a [Fraction]) -> Self {
        Self {
            instructions,
            _phantom: PhantomData,
        }
    }

    pub fn start(&'a self, start: I) -> FractranRun<'a, I, O> {
        FractranRun {
            current: start.into(),
            program: self,
        }
    }

    pub fn run(&self, start: I) -> O {
        self.start(start).last().unwrap().into()
    }
}

#[macro_export]
macro_rules! fractran {
    ($( $n:literal/$d:literal) *) => {
        FractranProgram::new(&[$(Fraction{ num:$n, den:$d},)*])
    };
}

impl<'a, I, O> Iterator for FractranRun<'a, I, O>
where
    I: Into<usize>,
    O: From<usize>,
{
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

impl<'a, I: Into<usize>, O: From<usize>> FusedIterator for FractranRun<'a, I, O> {}
