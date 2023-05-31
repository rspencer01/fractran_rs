#![no_std]
#![macro_use]
#![doc = include_str!("../README.md")]

use core::{iter::FusedIterator, marker::PhantomData};

/// A positive ratio of two `usize`s
///
/// ### Warning
///
/// In a true FRACTRAN program, the fractions must be in lowest terms. At no point are `Fraction`s
/// checked for this condition. It is therefore possible to construct programs using `Fraction`s
/// that are not entirely valid.
#[derive(Copy, Clone)]
pub struct Fraction {
    pub num: usize,
    pub den: usize,
}

/// A program in the FRACTRAN Language
///
/// FRACTRAN programs are simply lists of fractions and this thinly wraps a slice to such a list.
/// However, we also encode the input and output types, `I` and `O` respectively which must expose
/// an interface to convert to and from natural numbers (i.e. `usize`).
pub struct FractranProgram<'a, I = usize, O = usize>
where
    I: Into<usize>,
    O: From<usize>,
{
    instructions: &'a [Fraction],
    _phantom: PhantomData<(I, O)>,
}

/// A in-progress run of a FRACTRAN program
///
/// This function exposes an iterator of `usize`. Since intermediate calculations might not have
/// meaningful interpretation you should not convert them to `O` using `From` until the program is
/// complete.
pub struct FractranRun<'a, I: Into<usize>, O: From<usize>> {
    current: usize,
    program: &'a FractranProgram<'a, I, O>,
}

impl<'a, I: Into<usize>, O: From<usize>> FractranProgram<'a, I, O> {
    /// Construct a new program with the given source code.
    pub const fn new(instructions: &'a [Fraction]) -> Self {
        Self {
            instructions,
            _phantom: PhantomData,
        }
    }

    /// Create a [`FractranRun`] with a given input for this program.
    ///
    /// The `FractranRun` struct exposes an iterator interface that permits stepping through the
    /// program.
    pub fn start(&'a self, start: I) -> FractranRun<'a, I, O> {
        FractranRun {
            current: start.into(),
            program: self,
        }
    }

    /// Runs the FRACTRAN program to completion on the given input.
    pub fn run(&self, start: I) -> O {
        self.start(start).last().unwrap().into()
    }
}

/// Macro for making [`FractranProgram`]s
///
/// Programs should be specified as a space-separated list of fractions.
///
/// ### Example
/// ```rust
/// use fractran_rs::*;
///
/// let program : FractranProgram<usize, usize> = fractran!(2/3 5/6);
/// ```
#[macro_export]
macro_rules! fractran {
    ($( $n:literal/$d:literal) *) => {
        FractranProgram::new(&[$(Fraction{ num:$n, den:$d},)*])
    };
}

impl<'a, I: Into<usize>, O: From<usize>> Iterator for FractranRun<'a, I, O> {
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
