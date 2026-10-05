fn fibonacci_recursive(n: usize) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci(n: usize) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut sum = 0;
            let mut last = 0;
            let mut current = 1;
            for _ in 1..n {
                sum = last + current;
                last = current;
                current = sum;
            }
            sum
        }
    }
}

pub fn run() {
    println!("{}", fibonacci_recursive(10));
    println!("{}", fibonacci(10));
}