extern "C" {
    fn abs(n: i32) -> i32;
    fn fabs(n: f64) -> f64;
}

#[no_mangle]
pub extern "C"
fn you_can_call_me_from_c() {
    println!("Greetings from my Rust function.");
}

//------------ Using custom Rust struct from C ------------------------------------------

//------------ Rust code

/*
#[repr(C)] // ensures the Rust struct is laid out in memory the same way as the C struct
pub struct MyStruct {
    pub x: i32,
    pub y: f64,
}

#[no_mangle]
pub extern "C" fn create_my_struct(x: i32, y: f64) -> *mut MyStruct {
    let my_struct = MyStruct { x, y };
    // Allocate the structure on the heap and return a pointer to it
    Box::into_raw(Box::new(my_struct))
}

#[no_mangle]
pub extern "C" fn free_my_struct(ptr: *mut MyStruct) {
    if !ptr.is_null() {
        // Reconvert the raw pointer into a Box to safely free it
        unsafe { Box::from_raw(ptr); }
    }
}
*/

//------------ C header file

/*
typedef struct {
    int x;
    double y;
} MyStruct;

MyStruct* create_my_struct(int x, double y);
void free_my_struct(MyStruct* ptr);
*/

//------------- C file

/*
#include <stdio.h>
#include "my_struct.h"

int main() {
    MyStruct* s = create_my_struct(10, 20.5);
    printf("MyStruct: x = %d, y = %f\n", s->x, s->y);
    free_my_struct(s);
    return 0;
}
*/

//------------- Cargo.toml configuration

/*
[lib]
crate-type = ["cdylib"]  # or "staticlib" for static linking
*/

//------------- Building
/*
cargo build --release

Compile the C code and link it to the Rust library:
gcc main.c -o main -L./target/release -lmy_rust_lib
*/

//------------ Using custom C struct from Rust ------------------------------------------

//-------------- C header file
/*
#ifndef MY_STRUCT_H
#define MY_STRUCT_H

struct MyStruct {
    int x;
    double y;
};

void print_my_struct(const struct MyStruct* s);

struct MyStruct* create_my_struct(int x, double y);

void free_my_struct(struct MyStruct* s);

#endif
*/

//------------ C file

/*
#include "my_struct.h"
#include <stdio.h>
#include <stdlib.h>

void print_my_struct(const struct MyStruct* s) {
    printf("MyStruct: x = %d, y = %f\n", s->x, s->y);
}

struct MyStruct* create_my_struct(int x, double y) {
    struct MyStruct* s = (struct MyStruct*)malloc(sizeof(struct MyStruct));
    s->x = x;
    s->y = y;
    return s;
}

void free_my_struct(struct MyStruct* s) {
    free(s);
}
*/

//------------ Rust code

/*
#[repr(C)]
pub struct MyStruct {
    pub x: i32,
    pub y: f64,
}

extern "C" {
    pub fn create_my_struct(x: i32, y: f64) -> *mut MyStruct;
    pub fn print_my_struct(s: *const MyStruct);
    pub fn free_my_struct(s: *mut MyStruct);
}

fn main() {
    let my_struct: *mut MyStruct = unsafe { create_my_struct(42, 3.14) };
    unsafe {
        print_my_struct(my_struct as *const MyStruct);
    }
    unsafe {
        free_my_struct(my_struct);
    }
}
*/

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