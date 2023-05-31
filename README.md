# ➗ `FRACTRAN_rs` ➗
[![crates.io version](https://img.shields.io/crates/v/fractran_rs)](https://crates.io/crates/fractran_rs)
[![docs](https://img.shields.io/docsrs/fractran_rs)](https://docs.rs/fractran_rs/latest/fractran_rs/)
![MIT Licence](https://img.shields.io/github/license/rspencer01/fractran_rs)
[![Check and Lint](https://github.com/rspencer01/fractran_rs/actions/workflows/check-and-lint.yaml/badge.svg)](https://github.com/rspencer01/fractran_rs/actions/workflows/check-and-lint.yaml)
[![Test](https://github.com/rspencer01/fractran_rs/actions/workflows/test.yaml/badge.svg)](https://github.com/rspencer01/fractran_rs/actions/workflows/test.yaml)
[![Buy me a coffee](https://img.shields.io/badge/-buy_me_a%C2%A0coffee-gray?logo=buy-me-a-coffee)](https://www.buymeacoffee.com/rspencer)

> Because why not?

## 📜 The Language

Each FRACTRAN program consists of a finite ordered tuple of positive rational numbers (fractions).
It takes as input a natural number and either outputs another natural number or does not halt.
To do this, take the input number `n` and compare it to each fraction `f` in turn. If `nf` is an integer, then restart the procedure with that number as input. If we reach the end of the list of fractions without that ever being an integer, halt.

For example, here is a program that takes in a number and outputs its largest odd divisor.
```text
1/2
```

On the other hand, this program takes an input of the form 2<sup>a</sup>3<sup>b</sup> and outputs 5<sup>a+b</sup>.
```text
5/2  5/3
```
We could think of this as the program to add two numbers together.  Similarly the single instruction `5 / 6` takes 2<sup>a</sup>3<sup>b</sup> to 5<sup>max(a,b)</sup>.

The most famous example of a FRACTRAN program is [PRIMEGAME](https://github.com/rspencer01/fractran_rs/tree/main/examples/primegame.rs):
```text
17/91 78/85 19/51 23/38 29/33 77/29 95/23 77/19 1/17 11/13 13/11 15/2 1/7 55/1
```
Given the input of 2, this program never halts, but every time it hits a power of two, it does so in the form 2<sup>p</sup> for successive primes p.

## 🦄 Using `FRACTRAN_rs`

For examples of usage, see the [`examples`](https://github.com/rspencer01/fractran_rs/tree/main/examples) folder. Simple programs can be constructed as using the `fractran!` macro:
```rust
#[macro_use]
use fractran_rs::*;

let program : FractranProgram<usize, usize> = fractran!(5/2 5/3);
let result = program.run(24);
```

On the other hand, you may wish to provide encoding and decoding to the natural numbers for arbitrary datatypes. For example here we consider a hypothetical "find in array" program:
```no_run
#[macro_use]
use fractran_rs::*;

struct Input<'a>(usize, &'a [usize]);

impl<'a> Into<usize> for Input<'a> {
  fn into(self) -> usize { todo!() }
}

struct Output(Option<usize>);

impl From<usize> for Output {
  fn from(_: usize) -> Self { todo!() }
}

fn main() {
    let program : FractranProgram<Input, Output> = fractran!(/* ... */);
    let input = Input(3, &[2, 3, 5, 7, 11]);
    let result = program.run(input);
}
```

You can also "step through" your programs. To do this, call `start` on the program, followed by `next`:
```no_run
#[macro_use]
use fractran_rs::*;

let primegame : FractranProgram<usize, usize> = fractran!(/* ... */);
let mut program_run = primegame.start(2);
program_run.next(); // Some(15)
program_run.next(); // Some(825)
// etc.
```
This is implemented as an iterator interface, so you can run through the entire program with
```no_run
#[macro_use]
use fractran_rs::*;

let program : FractranProgram = fractran!(/* ... */);

for intermediate_result in program.start(3) {
  // ...
}
```

## 🎨 Prior art and alternatives
You may be intersted in [`fractran`](https://crates.io/crates/fractran) and [`fractran_macros`](https://crates.io/crates/fractran_macros) if you'd like to write fractran but have it compile to machine code.
Both use prime factorised representations of numbers internally.

I would like to recommend reading Michael Malis' [excellent article](https://malisper.me/building-fizzbuzz-fractran-bottom/) on writing a compiler that _targets_ FRACTRAN.

## 🚨 Limitations
Currently `FRACTRAN_rs` is limited to working with `usize`s as numerators and denominators of fractions. This is to keep the codebase to under 100 LOC. Besides, 64 bits (at time of writing) should be enough for anyone.

Due to the current economic climate, all `FRACTRAN_rs` programs must be finite.

## 👩🏼‍💻 Contributing
FRACTRAN_rs is considered feature-complete by the author. However, pull requests to fix bugs or suggest new features are welcome.
