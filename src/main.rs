/*
https://github.com/landrzejewski/rust-training

Installation/environment setup
- rustup tool from https://rustup.rs
- Visual Code + Rust extension, alternatively RustRover
- git

Important commands:
rustup --version                       # check rustup and rustc version
rustc main.rs                          # compile a file
rustfmt main.rs                        # format source a file
cargo new training_project             # create new project with cargo tool
cargo build                            # build an application in debug mode
cargo run                              # build and run an application in debug mode
cargo build --release                  # build an application in release mode
cargo check                            # check/build code without generating executables
cargo fmt                              # format source files in the project
cargo clean                            # clean project
*/
use std::sync::atomic::{AtomicI32, Ordering};

fn main(){
}

#[allow(unused_assignments)]
#[allow(unused_mut)]
fn variable_declaration() {
    /*
   - simple variables can be printed using the println! macro and {} - Display trait is required
   - complex variables like structures can be printed using the println! macro and {:?} or {:#?} (pretty printing) - Debug trait is required
   - it is also possible to format the output {variable name:padding_symbol alignment(<^>) minimum.maximum precision}
    */
    let a = 10; // declaration of a non-mutable variable, the type is inferred automatically, but can be optionally defined, e.g. let a:i32 = 10;
    println!("The value of a is: {a}");
    // a = 6; // compilation error - non-mutable variable cannot change its value

    let b; // initialization is not necessary at the time of declaration, but it is necessary before the first use of a variable
    b = 10;
    println!("The value of b is: {b}");

    let mut c = 10; // declaration of a mutable variable
    c = 30; // it is allowed to change the value, but not the type of the variable
    println!("The value of c is: {c}");

    let _d = 10; // the use of an underscore at the beginning of a variable name means that it will not cause compiler warnings, even if the variable is never used

    // variable shadowing
    let e = 20;
    {
        let mut e: f32 = e as f32 * 3.0;
        println!("The value of e in the inner scope {e}")
    }
    println!("The value of e in the outer scope {e}");

    // shadowing comes in handy when multiple steps are needed to get the final result, and the value and type of the partial result can change (no need to declare intermediate variables)
    let some_result = 5;
    let mut some_result = add(some_result, 3);
    let _some_result = some_result + 5;
}

fn add(value: i32, other_value: i32) -> i32 {
    // return value + other_value; // explicit return of the function result
    value + other_value // implicit return of the function result (no semicolon at the end)
}

/*
Compile-time constants
- not allocated in memory, are substituted directly in code on usage
- require an explicit type declaration - it is not inferred automatically
- their value must be known at compile time
- cannot change their value (use of mut keyword is not allowed)
- can have local and global scope
*/

const MONTH_OF_THE_YEAR: i8 = 4;

fn constant_declaration() {
    const TIMEOUT: i64 = 3600 * 10;
}

/*
Static values
- allocated in memory, with a static lifetime, have an address
- can be mutable
- require an explicit type declaration - it is not inferred automatically
- you can't initialize a static directly with a run-time value (it's not thread-safe)
- can have local and global scope
*/

static APP_NAME: &str = "Training app";

// static GLOBAL_TIMESTAMP: Lazy<DateTime<Utc>> = Lazy::new(|| {
//     let now = Utc::now();
//     println!(
//         "global GLOBAL_TIMESTAMP: {} ***** initialization *****",
//         now.format("%T")
//     );
//     return now;
// });


fn static_values() {
    println!("{}", MONTH_OF_THE_YEAR);
    println!("\nIn f1, GLOBAL_MESSAGE:   {}", APP_NAME);
    // println!(
    //     "In f1, GLOBAL_TIMESTAMP: {}",
    //     (*GLOBAL_TIMESTAMP).format("%T")
    // );

    // static mut LOCAL_COUNT: i32 = 0;
    // LOCAL_COUNT += 1; // mutable static variable is not thread safe, require unsafe block or some kind of synchronization

    static LOCAL_COUNT: AtomicI32 = AtomicI32::new(0);
    LOCAL_COUNT.fetch_add(1, Ordering::Relaxed);
}

/*
Data types
- must be known/specified at compile time - Rust is statically typed
- in most cases, the type can be inferred automatically by the compiler
- scalar/simple types - integers, floating-point numbers, booleans and characters
- compound types - tuples, arrays
*/
#[allow(unused_variables)]
fn data_types() {
    /*
    Integers

         Length	   Signed type       Unsigned type
         8-bit	   i8	             u8
         16-bit	   i16	             u16
         32-bit	   i32 // default    u32
         64-bit	   i64	             u64
         128-bit   i128	             u128

         32/64bit  isize	         usize  // size dependent on architecture, used among others as indexes in arrays

         Number literals	         Example
         Decimal	                 98_222_000
         Hex	                     0xff
         Octal	                     0o77
         Binary	                     0b1111_0000
         Byte (u8 only)	             b'A'

         let small_number = 10u8;
         let big_number = 100_000_000_i32;
    */

    println!("The smallest i8: {} The biggest i8: {}", i8::MIN, i8::MAX);
    println!("The smallest u8: {} The biggest u8: {}", u8::MIN, u8::MAX);
    println!(
        "The smallest i16: {} The biggest i16: {}",
        i16::MIN,
        i16::MAX
    );
    println!(
        "The smallest u16: {} and the biggest u16: {}",
        u16::MIN,
        u16::MAX
    );
    println!(
        "The smallest i32: {} The biggest i32: {}",
        i32::MIN,
        i32::MAX
    );
    println!(
        "The smallest u32: {} The biggest u32: {}",
        u32::MIN,
        u32::MAX
    );
    println!(
        "The smallest i64: {} The biggest i64: {}",
        i64::MIN,
        i64::MAX
    );
    println!(
        "The smallest u64: {} The biggest u64: {}",
        u64::MIN,
        u64::MAX
    );
    println!(
        "The smallest i128: {} The biggest i128: {}",
        i128::MIN,
        i128::MAX
    );
    println!(
        "The smallest u128: {} The biggest u128: {}",
        u128::MIN,
        u128::MAX
    );

    // let a: u8 = 300; // in debug mode, the compiler adds integer overflow verification (assertion) and interrupts program execution in case of its occurrence

    /*
    Floating-point numbers
    - compliant with IEEE-754 standard

         Length	                     Type
         32-bit	                     f32
         64-bit	                     f64 // default
    */

    let a = 14.3;
    let b = 14.;
    let c: f32 = 0.1;

    // standard mathematical operators are supported https://rust-book.cs.brown.edu/appendix-02-operators.html
    // explicit type conversion is required during calculation

    let sum = 5 + 10;
    let subtraction = 11.5 - 4f32; // 4 as f32
    let product = 4.0 * 12.5;
    let division = 56.7 / 32.2;
    let truncated_value = 2 / 3; // result truncated to 0
    let remainder = 54 % 5;

    let float: f32 = 5.0;
    let other_float = 8.5; // the compiler inferred the type as f32 from the context (operation below)
    let result = float + other_float;

    // one can use scientific notation with floats
    println!("Electron charge: {0}, {0:e}, {0:.4e}", -1.60217663e-16);
    println!("Speed of light: {0}, {0:e}, {0:.4e}", 2.99792458e8);

    /*
    Boolean
    - acceptable values are true and false
    - there is no implicit conversion to boolean
    */

    let positive_result = true;
    let negative_result: bool = false;

    let positive_value = positive_result as i32;
    let negative_value = negative_result as i32;
    println!(
        "Positive value: {}, negative value: {}",
        positive_value, negative_value
    );

    let result = positive_result && negative_result; // logical expressions are shortened if the result is known after expanding part of the expression

    /*
    Character
    - represents Unicode Scalar Value (can store complex characters)
    - is always 4 bytes in size
    */

    let letter = 'a';
    let other_letter: char = 'ℤ';
    let cat = '😻';

    // Unlike chars, strings can occupy a varying number of bytes in memory (from 1 to 4 bytes)

    let str = "Lukasz";
    println!(
        "str1 is {} bytes and {} chars",
        str.len(),
        str.chars().count()
    );
    println!("str1 {:?} bytes.", "L".as_bytes());
    let other_str = "Łukasz";
    println!(
        "str1 is {} bytes and {} chars",
        other_str.len(),
        other_str.chars().count()
    );
    println!("str2 {:?} bytes.", "Ł".as_bytes());

    // iterating over character, despite how many bytes it occupies
    for character in other_str.chars() {
        println!("  {}", character);
    }

    let first_letter = other_str.chars().nth(0); // safe access

    let message = "Hello😎";

    // Create slices as a portion of string.
    let slice1 = &message[0..3];
    let slice2 = &message[..3];
    let slice3 = &message[2..5];
    let slice4 = &message[2..];

    println!(
        "s3 ptr: {:p}, len: {}, text: {}",
        slice1.as_ptr(),
        slice1.len(),
        slice1
    );
    println!(
        "s4 ptr: {:p}, len: {}, text: {}",
        slice2.as_ptr(),
        slice2.len(),
        slice2
    );
    println!(
        "s5 ptr: {:p}, len: {}, text: {}",
        slice3.as_ptr(),
        slice3.len(),
        slice3
    );
    println!(
        "s6 ptr: {:p}, len: {}, text: {}",
        slice4.as_ptr(),
        slice4.len(),
        slice4
    );

    /*
    Rust has two kind of strings:
    - str - text literal, represents immutable slice of text, allocated statically
    - Instances od String type - potentially mutable text, allocated on the heap, deallocated at the end of the String object's lifetime
    */

    let text_slice: &'static str = "Hello"; // string literal

    println!(
        "{} {:p}, {}",
        text_slice,
        text_slice.as_ptr(),
        text_slice.len()
    ); // string slice knows the address of the first byte, and the number of bytes

    let text = String::from("Hello"); // instance oth the String type (vector of bytes)

    let text_slice: &str = &text; // implicit type coercion

    let mut message = String::from("Example ");
    message.push_str(" of mutable text");

    /*
    Tuple
    - groups elements of any type
    - has an invariant length
    - individual elements can be accessed through indexes or destructuring
    - memory allocation occurs on the stack
    */

    let tuple = (11, 11.0, 11); // max 12 elements in order to implement standard traits like PartialEq, Eq...
    println!("tuple first value {}", tuple.0);
    let (a, b, c) = tuple; // tuple destructuring, number of elements must match
    let (_, b, c) = tuple; // selected elements can be ignored
    let unit = (); // empty tuple, unit
    let other_tuple: (bool, i32);

    /*
    Array
    - groups elements of the same type
    - has an invariant length
    - access to elements is done through indexes, exceeding the correct index range ends the program (panic)
    - memory allocation occurs on the stack
    */

    let mut numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
    numbers[0] = -1;
    let matrix = [[1, 2], [3, 4]];

    let m = matrix;

    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("First number: {}", numbers[0]);
    let zeros = [0; 7]; // create an array of the specified size, filled with 0
    println!("Array of zeros: {:?}", zeros);

    // we can create/use a array slice (you have to use & because the size of the slice is determined dynamically and unknown at compile time)
    let array_of_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let two_to_five = &array_of_ten[2..5];
    let two_to_five_included = &array_of_ten[2..=5];
    let start_at_one = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];
    let test: [i32; 0] = [];
    println!("Two to five: {two_to_five:?}, Start at one: {start_at_one:?}, End at five: {end_at_five:?}, Everything: {everything:?}");
}