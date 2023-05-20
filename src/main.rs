use std::fmt::Debug;

struct Fraction {
    num: usize,
    den: usize,
}

impl Fraction {
    const fn new(num: usize, den: usize) -> Self {
        Self { num, den }
    }
}

impl Debug for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

macro_rules! f {
    ($n:literal/$d:literal) => {
        Fraction::new($n, $d)
    };
}

fn step(start: usize, program: &[Fraction]) -> usize {
    for Fraction { num, den } in program {
        if start % den == 0 {
            return start / den * num;
        }
    }
    return start;
}

fn main() {
    let mut start = 2;
    for i in 0..20 {
        println!("{}", start);
        start = step(start, &PRIMEGAME);
    }
}

const PRIMEGAME: [Fraction; 14] = [
    f!(17 / 91),
    f!(78 / 85),
    f!(19 / 51),
    f!(23 / 38),
    f!(29 / 33),
    f!(77 / 29),
    f!(95 / 23),
    f!(77 / 19),
    f!(1 / 17),
    f!(11 / 13),
    f!(13 / 11),
    f!(15 / 2),
    f!(1 / 7),
    f!(55 / 1),
];
