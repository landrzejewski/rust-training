use std::ops::Add;

#[derive(Default, Eq, PartialEq, Debug)]
pub struct Integer {
    value: i32,
}

impl Integer {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}


// https://doc.rust-lang.org/std/ops/index.html

impl Add<Integer> for Integer {
    type Output = Integer;

    fn add(self, rhs: Integer) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

pub fn run() {
    println!("Sum: {:?}", Integer::new(1) + Integer::new(2));
}

/*#[cfg(test)]
mod tests {
    use crate::wrappers::Integer;

    #[test]
    fn add_integers_internal() {
        let lhs = Integer::new(1);
        let rhs = Integer::new(2);
        assert_eq!(lhs + rhs, Integer::new(3));
    }

}*/
