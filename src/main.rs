use fractran_rs::*;

fn main() {
    let mut start = 2;
    for _ in 0..20 {
        println!("{start}");
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
