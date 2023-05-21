use fractran_rs::*;

fn main() {
    println!("PRIMEGAME:");
    for step in PRIMEGAME.start(2).take(20) {
        println!("{step}");
    }
}

const PRIMEGAME: FractranProgram =
    fractran!(17/91 78/85 19/51 23/38 29/33 77/29 95/23 77/19 1/17 11/13 13/11 15/2 1/7 55/1);
