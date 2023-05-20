use fractran_rs::*;

fn main() {
    let mut start = 2;
    for _ in 0..20 {
        println!("{start}");
        start = step(start, &PRIMEGAME);
    }
}

const PRIMEGAME: [Fraction; 14] = fractran!(
    17 / 91
    78 / 85
    19 / 51
    23 / 38
    29 / 33
    77 / 29
    95 / 23
    77 / 19
    1 / 17
    11 / 13
    13 / 11
    15 / 2
    1 / 7
    55 / 1
);
