use fractran_rs::*;

struct MultiplyInput(u32, u32);

impl Into<usize> for MultiplyInput {
    fn into(self) -> usize {
        2usize.pow(self.0) * 3usize.pow(self.1)
    }
}

#[derive(Debug)]
struct MultiplyOutput(u32);

impl From<usize> for MultiplyOutput {
    fn from(value: usize) -> Self {
        MultiplyOutput(value.ilog(5))
    }
}

const MULTIPLY: FractranProgram<MultiplyInput, MultiplyOutput> =
    fractran!(455/33 11/13 1/11 3/7 11/2 1/3);

fn main() {
    println!("{:?}", MULTIPLY.run(MultiplyInput(4, 3)));
}
