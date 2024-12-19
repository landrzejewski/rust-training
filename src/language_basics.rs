use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use std::mem::size_of_val;
use std::sync::atomic::{AtomicI32, Ordering};

pub fn run() {
    /*
    - simple variables can be printed using the println! macro and {} - Display trait is required
    - complex variables like structures can be printed using the println! macro and {:?} or {:#?} (pretty printing) - Debug trait is required
    - it is also possible to format the output {variable name:padding_symbol alignment(<^>) minimum.maximum precision}
     */
    // variable_declaration();
    // constant_declaration();
    // static_values();
    // data_types();
    // control_flow();
    // functions();
    // structs();
    // _ = enums();
}

#[allow(unused_assignments)]
#[allow(unused_mut)]
fn variable_declaration() {
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

static GLOBAL_TIMESTAMP: Lazy<DateTime<Utc>> = Lazy::new(|| {
    let now = Utc::now();
    println!("global GLOBAL_TIMESTAMP: {} ***** initialization *****", now.format("%T"));
    return now;
});

fn static_values() {
    println!("\nIn f1, GLOBAL_MESSAGE:   {}", APP_NAME);
    println!("In f1, GLOBAL_TIMESTAMP: {}", (*GLOBAL_TIMESTAMP).format("%T"));

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
    println!("The smallest i16: {} The biggest i16: {}", i16::MIN, i16::MAX);
    println!("The smallest u16: {} and the biggest u16: {}", u16::MIN, u16::MAX);
    println!("The smallest i32: {} The biggest i32: {}", i32::MIN, i32::MAX);
    println!("The smallest u32: {} The biggest u32: {}", u32::MIN, u32::MAX);
    println!("The smallest i64: {} The biggest i64: {}", i64::MIN, i64::MAX);
    println!("The smallest u64: {} The biggest u64: {}", u64::MIN, u64::MAX);
    println!("The smallest i128: {} The biggest i128: {}", i128::MIN, i128::MAX);
    println!("The smallest u128: {} The biggest u128: {}", u128::MIN, u128::MAX);

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
    println!("Positive value: {}, negative value: {}", positive_value, negative_value);

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
    println!("str1 is {} bytes and {} chars", str.len(), str.chars().count());
    println!("str1 {:?} bytes.", "L".as_bytes());
    let other_str = "Łukasz";
    println!("str1 is {} bytes and {} chars", other_str.len(), other_str.chars().count());
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

    println!("s3 ptr: {:p}, len: {}, text: {}", slice1.as_ptr(), slice1.len(), slice1);
    println!("s4 ptr: {:p}, len: {}, text: {}", slice2.as_ptr(), slice2.len(), slice2);
    println!("s5 ptr: {:p}, len: {}, text: {}", slice3.as_ptr(), slice3.len(), slice3);
    println!("s6 ptr: {:p}, len: {}, text: {}", slice4.as_ptr(), slice4.len(), slice4);

    /*
    Rust has two kind of strings:
    - str - text literal, represents immutable slice of text, allocated statically
    - Instances od String type - potentially mutable text, allocated on the heap, deallocated at the end of the String object's lifetime
    */

    let text_slice: &'static str = "Hello"; // string literal

    println!("{} {:p}, {}", text_slice, text_slice.as_ptr(), text_slice.len()); // string slice knows the address of the first byte, and the number of bytes

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

    let tuple = (11, 11.0, 11);
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

#[allow(unused_assignments)]
fn control_flow() {
    /*
    If expression
    - allow code sections to be executed when a certain condition is met
    - the result of a conditional expression must be of type bool (no implicit conversion as in some languages)
    - return a result, which, for example, can be assigned to a variable or used in other ways (returned results must be of the same type, all possible scenarios must be taken care of)
    */

    let number = 3;
    if number < 5 {
        println!("Number is lower than 5");
    } else if number > 5 {
        println!("Number is greater than 5");
    } else {
        println!("Number is equal 5");
    }

    let some_condition = true;
    let _option: char = if some_condition { 'a' } else { 'b' };

    /*
    Match expression
    - allow code sections to be executed when a certain condition is met
    - return a result, which, for example, can be assigned to a variable or used in other ways (returned results must be of the same type, all possible scenarios must be taken care of)
    */

    let dice_roll = 3;
    let _roll_result = match dice_roll {
        6 => "Win",
        val @ 1 => {
            println!("You lost! {val}");
            "Loose"
        }
        value => {
            // if you are not interested in the value you can use _
            println!("You hit {value}, try again");
            "Try again"
        }
    };

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let sky = "cloudy";
    let temperature = "warm";
    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's dark and unpleasant today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", _) => println!("It's dark but not bad"),
        _ => println!("Not sure what the weather is."),
    }

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x), // match guard
        Some(x) => println!("The number {} is odd", x),
        _ => (),
    }

    // Loops

    // loop

    let mut counter = 0;
    let result = 'myLoop: loop {
        counter += 1;
        if counter == 20 {
            break 'myLoop counter * 2;
        }
    };
    println!("The loop result is: {result}");

    // while

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
        break;
    }

    // for in

    let elements = [1, 2, 3, 4, 5];

    for index in 0..4 {
        println!("Current element: {}", elements[index]);
    }

    for element in elements {
        println!("Current element: {}", element);
    }

    for _ in (0..elements.len()).step_by(2) {
        println!("Next loop");
    }
}

fn functions() {
    println!("Addition result: {}", add(1, 2));
    println!("Validation result: {}", validate(10, is_even));
    println!("Validation result: {}", validate(10, |value| value % 2 == 0));

    // block expression
    let _score = {
        let x = 3;
        x * 3
    };
}

fn add(value: i32, other_value: i32) -> i32 {
    // return value + other_value; // explicit return of the function result
    value + other_value // implicit return of the function result (no semicolon at the end)
}

fn validate<T>(value: T, predicate: fn(T) -> bool) -> bool {
    predicate(value)
}

fn is_even(value: i32) -> bool {
    value % 2 == 0
}

/*
Structs
- group elements of any type, but unlike tuples, allow them to be named
- allow you to create multiple instances having the same properties (like object instances from other languages)
- structure elements are accessed using the dot operator
- if the structure instance is mutable it is possible to modify its fields
 */
#[allow(unused_variables)]
fn structs() {
    let origin = Point(0, 0);
    let x = origin.0;

    let point = Point3d { x: 2, y: 2, z: 2 };
    let y = point.y;

    let Point3d { x, y, z } = point;
    println!("x, y, z: {x}, {y}. {z}");
    let Point3d { x: a, y: b, z: c } = point;
    let Point3d { x: aa, .. } = point;
    println!("x, y, z: {a}, {b}. {c}");

    let active = true;
    let mut account = Account {
        email: "john@training.pl".to_string(),
        password: String::from("123"),
        active, // shortcut for active: active
    };

    println!("{:#?}", account);
    account.active = false;
    println!("{:#?}", account);

    let other_account = Account {
        email: String::from("marek@training.pl"),
        ..account.clone()
    };

    println!("Other account: {:#?}", other_account);
    println!("{:?}", account.email);
    // println!("{:?}", account.password); // error - after copying elements to other_account, we partially lost ownership (reference types)
    // println!("{:?}", account); // error - after copying elements to other_account, we partially lost ownership (reference types)

    match point {
        Point3d { x: 0, y, z } => println!("On the x axis at {}", x),
        Point3d { x, y: 0, z } => println!("On the y axis at {}", y),
        Point3d { x, y, z } => println!("On neither axis: ({}, {})", x, y),
    }

    match point {
        Point3d { x, .. } => println!("x is {}", x), // ignoring other elements of the structure
    }

    let rectangle = Rectangle { width: 100, height: 50 };
    println!("Rectangle area: {}", rectangle.area()); // == Rectangle::area(&rectangle);

    let square = Rectangle::square(10);
    println!("The area of the rectangle is {} square pixels.", square.area()); // == Rectangle::area(&rectangle);
}

// unit struct
struct Directory;

// tuple struct/named tuple
struct Point(i32, i32);

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone)]
struct Account {
    active: bool,
    email: String,
    password: String,
}

#[derive(Debug, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    const DEFAULT_SIZE: u32 = 100;

    fn area(&self) -> u32 {
        //  &self is an abbreviation of self: &Self or self: &Rectangle
        self.width * self.height
    }

    fn is_bigger(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        // associated function (like static functions in other languages), often used to create new instances, such as String::new, String::from
        Self {
            // in this case Self is an alias for Rectangle
            width: size,
            height: size,
        }
    }

    fn square_with_default_size() -> Self {
        Self {
            width: Self::DEFAULT_SIZE,
            height: Self::DEFAULT_SIZE,
        }
    }
}

impl Rectangle {
    fn width(&self) -> u32 {
        // methods can be split into multiple impl blocks
        self.width
    }
}

/*
Enums
- represent the enumeration of fixed/possible variants
- allow to define methods/behaviors (the same as for structures)
*/
#[allow(unused_variables)]
fn enums() -> Result<(), String> {
    let qr_code: Barcode = Barcode::Qr(String::from("345345345345"));
    let product_code = Barcode::Product {
        id: 5,
        value: String::from("123"),
    };

    println!("{}", size_of_val(&qr_code));
    println!("{}", size_of_val(&product_code));
    println!("{}", size_of_val(&Barcode::Other));
    // let a  = [qr_code, product_code, Barcode::Other];

    // println!("Value: {}", Values::from_i32(17));

    // destructuring of enumeration elements

    match qr_code {
        Barcode::Other => println!("Other barcode"),
        Barcode::Product { value, id } => println!("Product {id}:{value} "),
        Barcode::Qr(value) => println!("Qr {value} "),
        _ => (),
    }

    match product_code {
        Barcode::Product { id: id_value @ 4..=10, value: _ } => println!("Id in big range {id_value}"), // bind values in range
        Barcode::Product { id: 1..=3, value: _ } => println!("Id in small range"),
        _ => {}
    }

    /* one of the built-in enumeration types is Option representing a value or the absence of a value (alternative to null)

    enum Option<T> {
        None,
        Some(T),
    }
    */

    let _result = safe_div(3.0, 3.0).expect("Division by 0"); //.unwrap();

    let val = safe_div(3.0, 3.0).unwrap_or(0.0);

    match safe_div(3.0, 3.0) {
        Some(value) => println!("3.0 / 3.0 = {}", value),
        _ => (),
    }

    if let Some(value) = safe_div(3.0, 3.0) {
        println!("3.0 / 3.0 = {}", value);
    }

    let Some(value) = safe_div(3.0, 3.0) else {
        return Err("error".to_string());
    };

    println!("3.0 / 3.0 = {}", value);

    let mut stack = vec![1, 2, 3];
    while let Some(value) = stack.pop() {
        println!("Value: {}", value);
    }

    /*
     enum Result<T, E> {
        Ok(T),
        Err(E),
     }
    */

    let _result = safe_div_with_result(3.0, 3.0)?; // in case of Err return/exit function

    safe_div_with_result(3.0, 3.0).ok();

    match safe_div_with_result(3.0, 3.0) {
        Ok(value) => println!("3.0 / 3.0 = {}", value),
        Err(message) => return Err(message),
    }

    // you can use unwrap_or() to extract Ok value from a Result, or use a fallback value if Err.
    let parsed_value = i32::from_str_radix("FF", 16);
    println!("Result: {}", parsed_value.unwrap_or(-1));

    // Option -> Result

    let some_option: Option<i32> = Some(42);
    let none_option: Option<i32> = None;

    let result_some: Result<i32, &str> = some_option.ok_or("Value was None");
    let result_none: Result<i32, &str> = none_option.ok_or("Value was None");

    let result_some: Result<i32, String> = some_option.ok_or_else(|| "Value was None".to_string());
    let result_none: Result<i32, String> = none_option.ok_or_else(|| "Value was None".to_string());

    println!("{:?}", result_some); // Output: Ok(42)
    println!("{:?}", result_none); // Output: Err("Value was None")

    // Result -> Option

    let success: Result<i32, &str> = Ok(42);
    let error: Result<i32, &str> = Err("Something went wrong");

    let success_option: Option<i32> = success.ok();
    let error_option: Option<i32> = error.ok();

    println!("{:?}", success_option); // Output: Some(42)
    println!("{:?}", error_option); // Output: None

    let success_error: Option<&str> = success.err();
    let error_error: Option<&str> = error.err();

    println!("{:?}", success_error); // Output: None
    println!("{:?}", error_error); // Output: Some("Something went wrong")

    Ok(())
}

fn get_first_char(input: &str) -> Option<char> {
    let string = input.chars().nth(0)?; // If `input` is `None`, this returns `None` immediately.
    Some(string)
}

enum Currency {
    #[allow(dead_code)]
    Eur,
    Pln,
    Gbp,
}

enum Values {
    A = 17,
    B = 42,
    C,
}

struct Money {
    value: f64,
    currency: Currency,
}

#[derive(Debug)]
enum Barcode {
    Other,
    Qr(String),
    Upc(i32, i32, i32, i32),
    Product { value: String, id: i64 },
}

impl Barcode {
    fn get_info(&self) -> String {
        format!("Barcode {:?}", self)
    }
}

fn safe_div(value: f64, dividend: f64) -> Option<f64> {
    if dividend == 0.0 {
        None
    } else {
        Some(value / dividend)
    }
}

fn safe_div_with_result(value: f64, dividend: f64) -> Result<f64, String> {
    if dividend == 0.0 {
        Err("Division by 0".to_string())
    } else {
        Ok(value / dividend)
    }
}
