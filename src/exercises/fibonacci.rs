fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci(n: u32) -> u64 {
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
    println!("Fibonacci: {}", fibonacci(10));
    println!("Fibonacci: {}", fibonacci_recursive(10));
}
