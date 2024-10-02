use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Wrapper {
    value: i32
}

impl Wrapper {

    pub fn new(value: i32) -> Wrapper {
        Wrapper {
            value
        }
    }

}

// https://doc.rust-lang.org/std/ops/index.html
impl Add<Wrapper> for Wrapper {

    type Output = Wrapper;

    fn add(self, rhs: Wrapper) -> Self::Output {
        Wrapper {
            value: self.value + rhs.value
        }
    }

}

pub fn run() {
    println!("{:?}", Wrapper { value: 1 } + Wrapper { value: 2 });
}