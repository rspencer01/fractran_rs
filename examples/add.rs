use fractran_rs::*;

struct AddInput(u32, u32);

impl Into<usize> for AddInput {
    fn into(self) -> usize {
        2usize.pow(self.0) * 3usize.pow(self.1)
    }
}

#[derive(Debug)]
struct AddOutput(u32);

impl From<usize> for AddOutput {
    fn from(value: usize) -> Self {
        AddOutput(value.ilog(2))
    }
}

const ADD: FractranProgram<AddInput, AddOutput> = fractran!(2 / 3);

fn main() {
    println!("{:?}", ADD.run(AddInput(23, 17)));
}
