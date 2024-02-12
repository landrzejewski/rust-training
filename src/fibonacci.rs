fn main() {
    println!("Fibonacci: {}", fibonacci(10));
}

fn fibonacci_recursive(n: i32) -> u64 {
    if n <= 0 {
        panic!("{} Illegal argument!", n);
    }
    match n {
        1 | 2 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci(n: i32) -> u64 {
    if n < 0 || n == 0 {
        panic!("Illegal argument");
    } else if n == 1 {
        return 1;
    }

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
