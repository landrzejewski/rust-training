pub fn do_it() {

    println!("\nIn demo_unsafe_code::do_it()");

    let mut x = 100;
    x += 1;

    let mut y = 200;
    y += 1;

    // Declare some raw pointers (not guaranteed to point to valid memory, are not managed in any way).
    let p1 : *const i32 = &x;       // *const - designates a raw pointer that treats the data as constant.    
    let p2 : *mut i32   = &mut y;   // *mut   - designates a raw pointer that treats the data as mutable.
    
    // Dereference raw pointers in an "unsafe" code block (if you try this in normal code, it won't compile). 
    unsafe {
        println!("*p1 {}", *p1);

        // This won't compile because p1 treats the data as constant.
        // *p1 = 111;
        // println!("*p1 {}", *p1);

        *p2 = 222;
        println!("*p2 {}", *p2);
    }
}