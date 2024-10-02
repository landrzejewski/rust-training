extern "C" {
    fn abs(n: i32) -> i32;
    fn fabs(n: f64) -> f64;
}

#[no_mangle]
pub extern "C"
fn you_can_call_me_from_c() {
    println!("Greetings from my Rust function.");
}

pub fn run() {
    println!("\nIn demo_language_integration::do_it()");

    unsafe {
        let res1 = abs(-42);
        println!("res1 {}", res1);

        let res2 = fabs(-3.14);
        println!("res2 {}", res2);

        let res3 = my_unsafe_function();
        println!("res3 {}", res3);
    }

    // You can call published-to-other-language functions in normal code (i.e. no need for unsafe).
    you_can_call_me_from_c();
}

unsafe fn my_unsafe_function() -> i32 {
    // Could do something potentially dangerous in here...
    42
}

// https://pyo3.rs/main