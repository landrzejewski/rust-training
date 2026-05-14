use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use clap::builder::Resettable::Value;
// =================================================================================================
// Section 1: Struct Types
// =================================================================================================

/*
## Struct Types

- Structs group related data under named fields — Rust's primary way to define custom compound
  types.
- Three kinds of structs:
  - **Named-field**: `struct Point { x: i32, y: i32 }` — fields have names and are accessed with dot
    notation.
  - **Tuple struct**: `struct Color(u8, u8, u8)` — fields are unnamed and accessed by position
    (`.0`, `.1`, `.2`). More meaningful than bare tuples because each tuple struct is a distinct
    type. Function signatures must use the tuple struct name — you cannot pass a bare tuple where a
    tuple struct is expected, and vice versa (type-checked at compile time).
  - **Unit struct**: `struct Marker;` — has no fields. Used as markers or for trait implementations.
- All fields must be initialized when creating an instance.
- Struct instances are **immutable by default**. Use `let mut` to allow field mutation — mutability
  applies to the entire instance, not individual fields.
*/

// Named-field struct
struct Point {
    x: i32,
    y: i32,
}

// Tuple struct — distinct from other (u8, u8, u8) tuples
struct Color(u8, u8, u8);

// Unit struct — no fields
struct Marker;

fn struct_types() {
    // Named-field struct — access fields with dot notation
    let p = Point { y: 20, x: 10, };
    println!("point: ({}, {})", p.x, p.y);

    // Tuple struct — access fields by position
    let red = Color(255, 0, 0);
    println!("red: ({}, {}, {})", red.0, red.1, red.2);

    // Tuple struct in function signatures — Color is a distinct type
    fn print_color(c: &Color) {
        println!("color: ({}, {}, {})", c.0, c.1, c.2);
    }
    print_color(&red);
    // print_color(&(255, 0, 0)); // ERROR: expected &Color, found &(i32, i32, i32)

    // --- Newtype pattern ---
    // A single-field tuple struct creates a distinct type from its
    // inner type. Unlike type aliases (module 002 section 9), newtypes
    // prevent accidentally mixing types at compile time.
    struct Meters(f64);
    #[allow(dead_code)]
    struct Seconds(f64);

    let distance = Meters(100.0);
    let _duration = Seconds(9.58);
    // Meters(100.0) + Seconds(9.58); // ERROR: no + impl, different types
    println!("distance: {} m", distance.0);
    // To make arithmetic work, implement traits like Add (module 008).

    // Unit struct — instantiated without braces or parentheses
    let _mark = Marker;

    // Mutability applies to the entire instance
    let mut p = Point { x: 0, y: 0 };
    p.x = 5;
    p.y = 10;
    println!("mutated point: ({}, {})", p.x, p.y);

    // Nested struct — a struct containing another struct
    struct Address {
        city: String,
        zip: String,
    }
    struct Person {
        name: String,
        address: Address,
    }
    let person = Person {
        name: String::from("Alice"),
        address: Address {
            city: String::from("Portland"),
            zip: String::from("97201"),
        },
    };
    println!(
        "{} lives in {} ({})",
        person.name, person.address.city, person.address.zip
    );

    println!("struct_types section executed");
}

// =================================================================================================
// Section 2: Struct Features
// =================================================================================================

/*
## Struct Features

- **Field init shorthand**: when a variable has the same name as a field, write `Point { x, y }`
  instead of `Point { x: x, y: y }`.
- **Struct update syntax** (`..`): create a new instance reusing fields from an existing one:
  `Account { email: new, ..other }`. Fields with heap data (e.g., `String`) are **moved**, not
  copied — this can cause partial moves (covered in module 006).
- **Destructuring**: extract fields into variables with `let Point { x, y } = point;`. Use `..` to
  ignore remaining fields. Use `let Point { x: a, y: b }` to rename bindings.
- **`#[derive(...)]`**: automatically generates trait implementations. Common derives:
  - `Debug` — enables `{:?}` and `{:#?}` formatting.
  - `Clone` — enables `.clone()` for deep copies.
  - `PartialEq` — enables `==` and `!=` comparison.
  - `Eq` — marker trait (no extra methods) asserting that equality is **reflexive** (`a == a` is
    always true). Required for `HashMap`/`HashSet` keys. Floats implement `PartialEq` but not `Eq`
    because `NaN != NaN`.
  - `Copy` — enables implicit copying (only for types where all fields are `Copy`).
  - `Default` — provides a default value via `Type::default()`. For numbers the default is `0`, for
    `bool` it is `false`, for `String`/`Vec` it is empty. Commonly used with `..Default::default()`
    in struct update syntax.
  Multiple derives can be combined: `#[derive(Debug, Clone, PartialEq, Eq, Default)]`.
*/

#[derive(Debug, Clone, PartialEq)]
struct Account {
    email: String,
    active: bool,
}

fn struct_features() {
    // Field init shorthand — variable names match field names
    let email = String::from("alice@example.com");
    let active = true;
    let acc = Account { email, active };
    println!("account: {acc:?}");

    // Struct update syntax — reuse fields from another instance
    let acc2 = Account {
        active: false,
        ..acc.clone() // clone to avoid moving `acc`'s String field
    };
    println!("acc2: {acc2:?}");

    // After struct update without clone, heap fields are moved:
    // let acc3 = Account { active: false, ..acc };
    // println!("{}", acc.email); // ERROR: email was moved into acc3

    // Destructuring — extract fields into variables
    let Account { email, active } = acc2;
    println!("email: {email}, active: {active}");

    // Destructuring with rename and ignoring fields
    let p = Point { x: 3, y: 7 };
    let Point { x: horizontal, .. } = p;
    println!("horizontal: {horizontal}");

    // Derive PartialEq — enables == comparison
    let a = Account {
        email: String::from("same"),
        active: true,
    };
    let b = Account {
        email: String::from("same"),
        active: true,
    };
    println!("a == b: {}", a == b);

    // Derive Default — provides sensible default values
    #[derive(Debug, Default)]
    #[allow(dead_code)]
    struct Config {
        debug: bool,  // default: false
        timeout: u64, // default: 0
        name: String, // default: ""
    }
    let default_config = Config::default();
    println!("default config: {default_config:?}");

    // Struct update with Default — override only specific fields
    let custom = Config {
        timeout: 30,
        ..Config::default()
    };
    println!("custom config: {custom:?}");

    println!("struct_features section executed");
}

// =================================================================================================
// Section 3: Methods and Associated Functions
// =================================================================================================

/*
## Methods and Associated Functions

- Structs store data, but data alone is not powerful. Adding functionality via **`impl` blocks**
  lets structs act on their own data — making them more useful building blocks.
- Functions defined outside `impl` blocks are **free functions**. Functions inside `impl` that take
  `self` as the first parameter are **methods**, called via dot notation. Those without `self` are
  **associated functions**.
- Three forms of `self` in methods:
  - `&self` — immutable borrow (read-only, most common).
  - `&mut self` — mutable borrow (can modify fields).
  - `self` — takes ownership (consumes the instance).
- **`Self`** (capital S) is an alias for the implementing type inside an `impl` block.
- **A method may share a name with a field.** Rust disambiguates by the parentheses: `rect.width` is
  the field access, while `rect.width()` is a method call. This is commonly used to expose a private
  field as a read-only **getter** (see module 011 for field privacy) — the field stays private, and
  the method serves as the public API.
- **Associated functions** (no `self`) are called with `Type::function()`. A common pattern is
  `new()` as a constructor.
- **Associated constants** can be defined inside `impl`: `const MAX: u32 = 100;`.
- A type can have **multiple `impl` blocks** — the compiler merges them.
- **Method chaining**: methods that return `Self`, `&Self`, or `&mut Self` can be chained:
  - Returning owned `Self` allows chaining with any form of `self` (`self`, `&self`, `&mut self`).
  - Return `&mut Self` to chain with `&mut self` or `&self` methods.
  - Return `&Self` to chain with `&self` methods only.
  - Methods returning nothing end the chain.
- **Automatic referencing and dereferencing.** When you call a method with `instance.method()`, Rust
  automatically inserts `&`, `&mut`, or `*` to match the receiver type declared in the method
  signature. That is why `rect.area()` compiles even though `area` takes `&self` — Rust rewrites the
  call to `(&rect).area()` for you. This is why Rust has no `->` operator like C/C++: the receiver
  type is always unambiguous (`&self`/`&mut self`/`self`), so the compiler can always figure out
  whether a call is reading, mutating, or consuming. It is one of the features that makes owned
  values, references, and smart pointers feel uniform at the call site.
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated constant
    const MAX_SIZE: u32 = 10_000;

    // Method: &self — read-only access
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method: &mut self — can modify fields
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // Method: self — consumes the instance, returns a new one
    fn into_square(self) -> Self {
        let side = self.width.max(self.height);
        Self {
            width: side,
            height: side,
        }
    }

    // Associated function — no self, called with Rectangle::new()
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // Associated function — constructor for a square
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Multiple impl blocks are allowed — the compiler merges them
impl Rectangle {
    fn is_larger_than(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }

    // A method can share its name with a field. `width()` is a
    // validity check method; `width` (no parens) is the field.
    fn width(&self) -> bool {
        self.width > 0
    }
}

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }

    // Returns &mut Self for chaining
    fn increment(&mut self) -> &mut Self {
        self.count += 1;
        self
    }

    // Returns &Self for chaining with &self methods
    fn display(&self) -> &Self {
        println!("count: {}", self.count);
        self
    }
}

#[allow(unused_variables)]
fn methods_and_associated_functions() {
    // Associated function — no instance needed
    let rect = Rectangle::new(30, 50);
    println!("area: {}", rect.area());  // Rectangle::area(&rect);

    println!("max allowed size: {}", Rectangle::MAX_SIZE);

    // Method with &mut self — modifies the instance
    let mut rect = Rectangle::new(10, 20);
    rect.scale(3);
    println!("scaled: {rect:?}");

    // Method with self — consumes the instance
    let sq = rect.into_square();
    println!("square: {sq:?}");
    // println!("{rect:?}"); // ERROR: rect was consumed by into_square()

    // Associated function — square constructor
    let sq = Rectangle::square(5);
    println!("5x5 area: {}", sq.area());

    // Method from second impl block
    let a = Rectangle::new(10, 10);
    let b = Rectangle::new(5, 5);
    println!("a larger than b: {}", a.is_larger_than(&b));

    // A method can share its name with a field — parentheses
    // disambiguate. `r.width()` calls the method (defined in the
    // second impl block above); `r.width` accesses the field.
    let r = Rectangle::new(30, 50);
    if r.width() {
        // `r.width()` is the method; `r.width` is the field
        println!("non-zero width: {}", r.width);
    }

    // Method chaining — increment returns &mut Self
    let mut counter = Counter::new();
    counter.increment().increment().increment().display();

    println!("methods_and_associated_functions section executed");
}

// =================================================================================================
// Section 4: Builder Pattern
// =================================================================================================

/*
## Builder Pattern

- When a struct has many fields, constructors become unwieldy: `new(name, username, membership,
  gender, country, age)` is hard to read and easy to misorder. Adding a new field forces every call
  site to change.
- The **telescoping constructors** workaround — `new()`, `new_with_username()`,
  `new_with_membership()`, etc. — leads to a combinatorial explosion of functions.
- The **builder pattern** solves this with a separate struct (`FooBuilder`) that accumulates
  optional parameters, then produces the final struct via `build()`.
- Builder fields are `Option<T>` so each one is independently settable. Setter methods take `&mut
  self`, set the field to `Some(value)`, and return `&mut Self` for chaining.
- The `build()` method consumes the accumulated options and fills in defaults for anything not set
  (typically via `unwrap_or_default()`).
- This pattern is common in Rust libraries (e.g., `std::process::Command`,
  `reqwest::ClientBuilder`).
*/

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
struct Customer {
    name: String,
    username: String,
    membership: MembershipType,
    gender: char,
    country: String,
    age: u8,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum MembershipType {
    New,
    Casual,
    Loyal,
}

impl Default for MembershipType {
    fn default() -> Self {
        MembershipType::New
    }
}

// The builder: each optional field is Option<T>
#[derive(Default)]
struct CustomerBuilder {
    name: String,
    username: Option<String>,
    membership: Option<MembershipType>,
    gender: Option<char>,
    country: Option<String>,
    age: Option<u8>,
}

impl CustomerBuilder {
    fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    fn membership(&mut self, membership: MembershipType) -> &mut Self {
        self.membership = Some(membership);
        self
    }

    fn gender(&mut self, gender: char) -> &mut Self {
        self.gender = Some(gender);
        self
    }

    fn country(&mut self, country: String) -> &mut Self {
        self.country = Some(country);
        self
    }

    fn age(&mut self, age: u8) -> &mut Self {
        self.age = Some(age);
        self
    }

    fn build(&mut self) -> Customer {
        Customer {
            name: self.name.clone(),
            username: self.username.clone().unwrap_or_default(),
            membership: self.membership.clone().unwrap_or_default(),
            gender: self.gender.unwrap_or_default(),
            country: self.country.clone().unwrap_or_default(),
            age: self.age.unwrap_or_default(),
        }
    }
}

impl Customer {
    // Returns a builder instead of the final struct
    fn builder(name: String) -> CustomerBuilder {
        CustomerBuilder {
            name,
            ..Default::default()
        }
    }
}

fn builder_pattern() {
    // Minimal — only the required field (name)
    let basic = Customer::builder("Alice".to_string()).build();
    println!("basic: {basic:?}");

    // Add a few fields — order doesn't matter
    let with_login = Customer::builder("Bob".to_string())
        .username("bob42".to_string())
        .age(30)
        .build();
    println!("with login: {with_login:?}");

    // Full customization — every field set
    let full = Customer::builder("Carol".to_string())
        .username("carol99".to_string())
        .membership(MembershipType::Loyal)
        .gender('F')
        .country("Canada".to_string())
        .age(28)
        .build();
    println!("full: {full:?}");

    // Without the builder, you'd need telescoping constructors:
    // fn new(name: String) -> Self { ... }
    // fn new_with_username(name: String, username: String) -> Self { ... }
    // fn new_with_membership(name: String, username: String, membership: MembershipType) -> Self { ... }
    // Each new optional field doubles the number of constructors.

    println!("builder_pattern section executed");
}

// =================================================================================================
// Section 5: Fallible Constructors
// =================================================================================================

/*
## Fallible Constructors

- A constructor that can **fail** returns `Result<Self, E>` instead of `Self`. This forces callers
  to handle the error case, preventing invalid instances from being created.
- Common use case: validating input in `new()`. If validation fails, return `Err(...)` instead of
  panicking.
- Combines well with `#[derive(Default)]` — callers can use `unwrap_or_default()` to fall back to a
  safe default when construction fails.
- The `..Default::default()` syntax can be used to override only specific fields of a default
  instance.
- **Naming convention: `new` should be infallible**. Rust idiom reserves the name `new` for
  constructors that **cannot fail** — callers rightly expect `Foo::new(...)` to return a `Foo`, not
  a `Result<Foo, E>`. When construction can fail, the idiomatic names are **`try_new`** (Rust
  community convention, parallel to `TryFrom`/`TryInto`) or **`build`** (used by TRPL ch. 12 in its
  `Config::build` example). The signature `fn try_new(...) -> Result<Self, E>` tells the caller at a
  glance that the construction is fallible. Module 012's `Rectangle::try_new`
  (src/basic/mod_012_testing.rs) follows this convention; the `Student::new` example below uses
  `new` purely for brevity, and a production version would rename it to `try_new`.
*/

#[derive(Debug, Default)]
#[allow(dead_code)]
struct Student {
    id: u8,
    age: u8,
    name: String,
}

impl Student {
    // Fallible constructor — validates that the name contains only lowercase letters
    fn new(name: String) -> Result<Self, String> {
        if name.chars().all(|c| matches!(c, 'a'..='z')) {
            Ok(Self {
                id: 0,
                age: 20,
                name,
            })
        } else {
            Err("name must contain only lowercase letters".to_string())
        }
    }
}

fn fallible_constructors() {
    // Valid name — returns Ok
    let s1 = Student::new("alice".to_string());
    println!("valid: {s1:?}");

    // Invalid name — returns Err
    let s2 = Student::new("Alice123".to_string());
    println!("invalid: {s2:?}");

    // Fallback to default on failure — unwrap_or_default
    let s3 = Student::new("Bob!".to_string()).unwrap_or_default();
    println!("fallback: {s3:?}");

    // Override specific fields of a default instance
    let s4 = Student {
        age: 25,
        ..Default::default()
    };
    println!("partial override: {s4:?}");

    // Custom Default implementation (commented out — derive is used above):
    // impl Default for Student {
    //     fn default() -> Self {
    //         Self { id: 0, name: "unknown".to_string(), age: 20 }
    //     }
    // }

    println!("fallible_constructors section executed");
}

// =================================================================================================
// Section 7: Enums
// =================================================================================================

/*
## Enums

- Enums reduce the risk of **typos and invalid values** compared to using strings or magic numbers,
  and are more descriptive than plain integer indexes.
- An **enum** defines a type with a fixed set of **variants**. Each variant can optionally carry
  associated data.
- Variant data forms:
  - No data: `Quit`.
  - Tuple-like: `Move(i32, i32)`.
  - Struct-like: `Resize { width: u32, height: u32 }`.
- All variants share the same type — they can be stored together in a `Vec` or matched in a single
  `match`.
- **Enums vs structs**: a struct's fields each have their own type; an enum defines multiple
  variants under one type. Both can have methods via `impl` blocks.
- Embedding data in enum variants removes the need to pass extra parameters separately — the data
  travels with the variant.
- **Discriminants**: variants without data can have explicit integer values (`A = 1, B = 2`). Cast
  with `as`.
- The size of an enum equals the size of its **largest variant** plus a discriminant tag.
- Use `match` (covered in module 004) to handle variants — the compiler enforces exhaustiveness.
*/

// Basic enum — no associated data
#[derive(Debug)]
#[allow(dead_code)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Enum with different data forms per variant
#[derive(Debug)]
enum Command {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
}

impl Command {
    fn execute(&self) {
        match self {
            Command::Quit => println!("quitting"),
            Command::Echo(msg) => println!("echo: {msg}"),
            Command::Move { x, y } => println!("moving to ({x}, {y})"),
        }
    }
}

// Enum with explicit discriminant values
#[allow(dead_code)]
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalError = 500,
}

fn enums() {
    // Basic enum — creating and matching variants
    let dir = Direction::East;
    let label = match dir {
        Direction::North => "north",
        Direction::South => "south",
        Direction::East => "east",
        Direction::West => "west",
    };
    println!("direction: {label}");

    // Enum variants in a collection — all share the same type
    let commands = vec![
        Command::Echo(String::from("hello")),
        Command::Move { x: 10, y: 20 },
        Command::Quit,
    ];
    for cmd in &commands {
        cmd.execute();
    }

    // Discriminant values — cast with `as`
    println!("HTTP 200 = {}", HttpStatus::Ok as i32);
    println!("HTTP 404 = {}", HttpStatus::NotFound as i32);

    // Size of an enum equals its largest variant + tag
    println!("size of Command: {} bytes", std::mem::size_of::<Command>());

    // --- Enum tuple-variant constructors as function pointers ---
    // Tuple variants like Some, Ok, Err are actually functions:
    // Some has type fn(T) -> Option<T>.
    // This lets you pass them where a closure would go.
    let wrapped: Vec<Option<i32>> = vec![1, 2, 3].into_iter().map(Some).collect();
    println!("map(Some): {wrapped:?}"); // [Some(1), Some(2), Some(3)]

    let results: Vec<Result<i32, &str>> = vec![1, 2].into_iter().map(Ok).collect();
    println!("map(Ok): {results:?}");

    println!("enums section executed");
}

// =================================================================================================
// Section 8: Option<T>
// =================================================================================================

/*
## Option<T>

- `Option<T>` is Rust's replacement for null — an enum with two variants: `Some(T)` (value present)
  and `None` (no value).
- **Why it prevents bugs.** `Option<T>` and `T` are **different types** — the compiler will not let
  you use an `Option<T>` where a `T` is expected. For example, `5i8 + Some(5i8)` is a type error:
  there is no `Add<Option<i8>>` for `i8`. To use the inner value you **must** explicitly convert
  `Option<T>` to `T` via `match`, `if let`, `?`, or an `unwrap*` method — the compiler forces you to
  acknowledge the `None` case. This eliminates the "forgot to null-check" class of bugs that plagues
  languages with implicit nulls.
- Extracting the inner value:
  - `match` (covered in module 004) or `if let` (section 7 below).
  - `.unwrap()` — returns the value or **panics** if `None`.
  - `.expect("msg")` — like unwrap with a custom panic message.
  - `.unwrap_or(default)` — returns the value or a fallback.
  - `.unwrap_or_else(|| closure)` — lazy fallback via closure.
- Transforming: `.map(|v| ...)` applies a function to `Some`, passes `None` through unchanged.
- Chaining: `.and_then(|v| ...)` applies a function that itself returns `Option` — useful for
  chaining fallible operations without nested `Some(Some(...))`.
- Defaults: `.unwrap_or_default()` returns the type's default value (e.g., `0` for integers, `""`
  for strings) when `None`.
- Flattening: `.flatten()` converts `Option<Option<T>>` to `Option<T>`.
- Testing: `.is_some()`, `.is_none()`.
- The **`?` operator** in a function returning `Option<T>`: if `None`, returns `None` early; if
  `Some`, unwraps the value.
- Converting to `Result`: `.ok_or(err)`, `.ok_or_else(|| err)`.
*/

// Returns None when dividing by zero
fn safe_div(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

// Demonstrates the ? operator with Option
// (Idiomatic version: just `s.chars().next()` — this form shows ? explicitly)
fn first_char(s: &str) -> Option<char> {
    let ch = s.chars().next()?; // returns None if empty
    Some(ch)
}

fn option_type() {
    // Creating Some and None
    let some_val: Option<i32> = Some(42);
    let no_val: Option<i32> = None;
    println!("some: {some_val:?}, none: {no_val:?}");

    // Extracting with match
    match safe_div(10.0, 3.0) {
        Some(result) => println!("10 / 3 = {result:.4}"),
        None => println!("cannot divide by zero"),
    }

    // unwrap, expect, unwrap_or
    let val = safe_div(10.0, 2.0).unwrap();
    println!("unwrap: {val}");

    let val = safe_div(10.0, 0.0).unwrap_or(0.0);
    println!("unwrap_or(0.0): {val}");

    // map — transform the inner value
    let doubled = safe_div(10.0, 2.0).map(|v| v * 2.0);
    println!("mapped: {doubled:?}");

    // is_some / is_none
    println!("Some(5).is_some(): {}", Some(5).is_some());
    println!("None::<i32>.is_none(): {}", None::<i32>.is_none());

    // ? operator — first_char returns None for empty strings
    println!("first_char(\"hello\"): {:?}", first_char("hello"));
    println!("first_char(\"\"): {:?}", first_char(""));

    // and_then — chain operations that each return Option
    let result = safe_div(10.0, 2.0).and_then(|v| safe_div(v, 3.0));
    println!("and_then chain: {result:?}");

    let result = safe_div(10.0, 0.0).and_then(|v| safe_div(v, 3.0));
    println!("and_then short-circuit: {result:?}"); // None

    // unwrap_or_default — uses Default trait value when None
    let val: Option<i32> = None;
    println!("unwrap_or_default: {}", val.unwrap_or_default()); // 0

    // unwrap_or_else — lazy fallback computed only when needed
    let val: Option<i32> = None;
    let result = val.unwrap_or_else(|| {
        println!("computing fallback...");
        42
    });
    println!("unwrap_or_else: {result}");

    // expect — like unwrap() but with a custom panic message
    let val = safe_div(10.0, 2.0).expect("division should succeed");
    println!("expect: {val}");
    // safe_div(10.0, 0.0).expect("oops"); // PANIC: "oops"

    // --- Unwrap family summary ---
    // unwrap()            — panics on None. Use in tests/prototypes only.
    // expect("msg")       — panics with message. Better for diagnosing bugs.
    // unwrap_or(default)  — returns default (always evaluated).
    // unwrap_or_else(f)   — returns f() (lazy — only called on None).
    // unwrap_or_default() — returns T::default() (requires T: Default).
    //
    // In production code, prefer `?`, `if let`, or `match` over any
    // unwrap variant. Use `expect()` when a panic message aids debugging.
    // Use `unwrap_or*` when a fallback value is appropriate.

    // flatten — collapse Option<Option<T>> into Option<T>
    let nested: Option<Option<i32>> = Some(Some(42));
    println!("flatten: {:?}", nested.flatten()); // Some(42)
    let nested: Option<Option<i32>> = Some(None);
    println!("flatten None: {:?}", nested.flatten()); // None

    // Converting Option to Result
    let opt: Option<i32> = Some(42);
    let result: Result<i32, &str> = opt.ok_or("value was None");
    println!("Option -> Result: {result:?}");

    let none: Option<i32> = None;
    let result: Result<i32, &str> = none.ok_or("value was None");
    println!("None -> Result: {result:?}");

    // filter — keeps Some only if the predicate passes
    let even = Some(4).filter(|x| x % 2 == 0);
    let odd = Some(3).filter(|x| x % 2 == 0);
    println!("filter even: {even:?}, odd: {odd:?}"); // Some(4), None

    // zip — combines two Options into a tuple Option
    let name: Option<&str> = Some("Alice");
    let age: Option<i32> = Some(30);
    println!("zip: {:?}", name.zip(age)); // Some(("Alice", 30))
    println!("zip None: {:?}", name.zip(None::<i32>)); // None

    println!("option_type section executed");
}

// =================================================================================================
// Section 9: Result<T, E>
// =================================================================================================

/*
## Result<T, E>

- `Result<T, E>` represents success (`Ok(T)`) or failure (`Err(E)`). Used for operations that can
  fail — file I/O, parsing, network calls. Rust has no exceptions — `Result` replaces try/catch
  patterns, encouraging explicit handling.
- `Option` represents **presence or absence** of a value; `Result` represents **success or failure**
  of an operation.
- Extracting: `.unwrap()`, `.expect("msg")`, `.unwrap_or(default)`, `match`.
- The **`?` operator** in a function returning `Result`: if `Err`, returns the error early; if `Ok`,
  unwraps the value.
- Transforming: `.map(|v| ...)` transforms `Ok`, `.map_err(|e| ...)` transforms `Err`.
- Converting to `Option`: `.ok()` discards the error, `.err()` discards the success value.
*/

fn parse_and_double(s: &str) -> Result<i32, String> {
    let n = s.parse::<i32>().map_err(|e| e.to_string())?;
    Ok(n * 2)
}

fn result_type() -> Result<i32, String> {
    // Creating Ok and Err
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("something failed");
    println!("success: {success:?}, failure: {failure:?}");

    // Matching on Result
    match parse_and_double("21") {
        Ok(val) => println!("parsed and doubled: {val}"),
        Err(e) => return Err(e),
    }

   //  let result = parse_and_double("21")?;

    // ? operator — error propagation
    match parse_and_double("abc") {
        Ok(val) => println!("parsed: {val}"),
        Err(e) => println!("parse error: {e}"),
    }

    // unwrap_or — fallback on error
    let val = parse_and_double("abc").unwrap_or(-1);
    println!("unwrap_or(-1): {val}");

    // map — transform the Ok value
    let result = parse_and_double("5").map(|v| v + 10);
    println!("map(+10): {result:?}");

    // map_err — transform the Err value
    let result = parse_and_double("abc").map_err(|e| format!("PARSE ERROR: {e}"));
    println!("map_err: {result:?}");

    // Converting Result to Option
    let ok_val: Result<i32, &str> = Ok(42);
    let as_option: Option<i32> = ok_val.ok();
    println!("Ok(42).ok() = {as_option:?}");

    let err_val: Result<i32, &str> = Err("fail");
    let as_option: Option<i32> = err_val.ok();
    println!("Err.ok() = {as_option:?}");

    println!("result_type section executed");

    Ok(0)
}

// =================================================================================================
// Section 10: if let, while let, and let else
// =================================================================================================

/*
## `if let`, `while let`, and `let else`

- A **pattern** describes the expected shape of data; **pattern matching** pairs a value against a
  pattern to destructure and bind variables. Every match involves two parts: the **value** and the
  **pattern**.
- **`if let`** is a shorthand for matching a single pattern — a concise alternative to `match`
  (covered in module 004). It runs a block when the pattern matches, and optionally an `else` block
  when it does not. Commonly used with `Option<T>` and `Result<T, E>`. Specifically, `if let PAT =
  expr { body }` replaces a `match` with one meaningful arm and a catch-all `_ => ()` ("do
  nothing"); writing `()` (the unit value) as an arm body means the arm does nothing and evaluates
  to the unit type. Use `match` when you want the compiler to force you to handle every case; use
  `if let` when you deliberately want to ignore the rest.
- If the pattern in `if let` is **irrefutable** (always matches, e.g., `if let x = 5`), the compiler
  warns — use a plain `let` binding instead.
- **`while let`** is the loop version of `if let` — it runs as long as the pattern matches. Commonly
  used to drain an iterator or consume values from a collection. If the pattern always matches, the
  loop runs infinitely.
- **`let else`** is the inverse of `if let`: it binds the matched value or **diverges** (must
  `return`, `break`, `continue`, or `panic!`). Useful for early-return guard clauses.
- `let` bindings are themselves pattern matching: `let (a, b) = (10, 20);` destructures the tuple
  (see also struct destructuring in section 2 above).
- **Function parameters are patterns** — e.g., `fn foo((a, b): (i32, i32))` destructures the
  argument directly (demonstrated in module 004).
- In patterns, a **variable name** always matches and binds the value — it does not compare. Use
  literal values or constants for comparison.
- **`ref` / `ref mut`** in patterns: creates a reference to the matched value instead of moving or
  copying it. In Rust 2024, match ergonomics make `ref` largely unnecessary — the compiler inserts
  references automatically when matching on borrowed values. Explicit `ref` is still valid but
  uncommon in idiomatic code.
- **Refutability**: patterns come in two flavors.
  - **Irrefutable** patterns *always* match — e.g., `let x = 5` or `let (a, b) = (1, 2)`. The name
    on the left side binds unconditionally, so there is no failure case to handle.
  - **Refutable** patterns *might not* match — e.g., `Some(x)` against an `Option<T>`, because the
    value could be `None`.
  - Contexts that have nowhere to go on a non-match require **irrefutable** patterns: plain `let`,
    function and closure parameters, and `for` loops.
  - Contexts whose whole purpose is to handle failure accept **refutable** patterns: `if let`,
    `while let`, and `let else`.
  - Using a refutable pattern where only irrefutable is allowed is an error — e.g., `let Some(x) =
    some_option;` fails with E0005. The fix is `let else` (diverge on failure) or `if let` (skip on
    failure).
  - Using an irrefutable pattern in a refutable-only context (e.g., `if let x = 5 { ... }` or `let x
    = 5 else { ... };`) compiles but warns — the extra machinery is pointless; use a plain `let`.
- **Nested destructuring**: patterns compose arbitrarily. An enum variant wrapping a struct wrapping
  another enum can be taken apart in a single pattern, binding the innermost fields directly — no
  intermediate `match` or field-access chain required.
- **Match guard + or-pattern precedence**: when a match guard is combined with an or-pattern, the
  `if` applies to the **whole** or-pattern. `4 | 5 | 6 if y` parses as `(4 | 5 | 6) if y`, not `4 |
  5 | (6 if y)` — the arm matches any of the three values but only when the guard is true. See
  module 004 Section 3 for a runnable demonstration.
*/

fn if_let_and_let_else() {
    // --- if let ---
    // Match a single pattern without a full match expression
    let config_value: Option<i32> = Some(42);

    // Before `if let`: a match with a do-nothing catch-all arm.
    // The `_ => ()` arm satisfies exhaustiveness but is pure
    // boilerplate — we don't care about the None case here.
    // match config_value {
    //     Some(val) => println!("config is set to: {val}"),
    //     _ => (), // do nothing
    // }

    // `if let` collapses that pattern into one line of intent:
    if let Some(val) = config_value {
        println!("config is set to: {val}");
    }

    // Equivalent full match (more verbose for a single pattern):
    // match config_value {
    //     Some(val) => println!("config is set to: {val}"),
    //     None => println!("config is not set"),
    // }

    // if let with Result
    let parsed: Result<i32, _> = "42".parse();

    // --- while let ---
    // Loop while the pattern continues to match
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("popped: {top}");
    }
    println!("stack is now empty: {stack:?}");

    // --- let else ---
    // Bind the value or diverge — useful for guard clauses
    let input: Option<&str> = Some("hello");

    let Some(value) = input else {
        println!("input was None — returning early");
        return;
    };
    // `value` is available here, unwrapped from the Option
    println!("let else unwrapped: {value}");

    // --- Pattern matching on struct fields ---
    // Concrete values are compared; variables are bound
    struct Status {
        code: u16,
        msg: String,
    }
    let resp = Status {
        code: 200,
        msg: String::from("OK"),
    };
    match resp {
        Status { code: 200, msg } => println!("success: {msg}"),
        Status { code: 404, msg } => println!("not found: {msg}"),
        Status { code, msg } => println!("other {code}: {msg}"),
    }

    // --- Variable bindings in match: the shadowing gotcha ---
    // Variables in match patterns ALWAYS bind; they never compare
    // against outer variables of the same name. This can silently
    // hide bugs when a pattern name collides with an outer binding.
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("got 50"),
        // This `y` is a NEW variable introduced by the pattern —
        // it shadows the outer `y = 10` inside this arm and binds
        // the inner 5 from Some(5). The arm ALWAYS matches.
        Some(y) => println!("matched, inner y = {y} (not 10!)"),
        _ => println!("default"),
    }
    println!("outer y is still {y}");

    // The fix: use a different binding name and a match guard
    // that references the outer `y`:
    match x {
        Some(50) => println!("got 50"),
        Some(n) if n == y => println!("equal to outer y = {y}"),
        _ => println!("default, x = {x:?}"),
    }

    // --- Refutability: which patterns can go where ---
    // `let` requires an IRREFUTABLE pattern (one that can't fail
    // to match). A refutable pattern like `Some(x)` is rejected:
    let some_val: Option<i32> = Some(5);
    // let Some(x) = some_val;
    // ERROR[E0005]: refutable pattern in local binding:
    //   pattern `None` not covered
    //
    // Fix 1: use `let else` to handle the None case explicitly.
    let Some(x) = some_val else {
        unreachable!("we know it's Some in this demo")
    };
    println!("refutability fix via let else: x = {x}");

    // Fix 2: use `if let` when the None case should just be skipped.
    if let Some(x) = some_val {
        println!("refutability fix via if let: x = {x}");
    }

    // Using an IRREFUTABLE pattern in a refutable-only context
    // (`if let`, `while let`, `let else`) is a warning:
    //   if let x = 5 { ... }  // warning: irrefutable `if let` pattern
    //   let x = 5 else { ... }; // warning: irrefutable `let...else` pattern

    // --- Chained if let with mixed predicates ---
    // `if let` can be combined with `else if` and `else if let`; the
    // conditions do not need to relate to each other or share a type.
    let favorite: Option<&str> = None;
    let is_monday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite {
        println!("using favorite {color}");
    } else if is_monday {
        println!("monday blue");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("mature palette");
        } else {
            println!("fresh palette");
        }
    } else {
        println!("fallback");
    }

    // --- Nested destructuring ---
    // Patterns nest arbitrarily. Here, an Instruction variant wraps
    // a Paint enum, and one pattern destructures all the way down
    // to the inner fields in a single arm — no intermediate match.
    #[allow(dead_code)]
    enum Paint {
        Rgb(u8, u8, u8),
        Named(String),
    }
    #[allow(dead_code)]
    enum Instruction {
        Stop,
        SetColor(Paint),
    }
    let cmd = Instruction::SetColor(Paint::Rgb(10, 20, 30));
    match cmd {
        Instruction::SetColor(Paint::Rgb(r, g, b)) => {
            println!("nested rgb: ({r}, {g}, {b})");
        }
        Instruction::SetColor(Paint::Named(name)) => {
            println!("nested named: {name}");
        }
        Instruction::Stop => println!("stop"),
    }

    // --- `..` in tuple patterns (not just structs) ---
    // `..` ignores a contiguous run of elements. In tuples it can
    // appear in the middle to pick out the first and last items.
    // The pattern form is the same in `let`, `match`, and fn params.
    let nums = (1, 2, 3, 4, 5);
    let (first, .., last) = nums;
    println!("tuple .. — first = {first}, last = {last}");
    // Note: `..` can appear only once per tuple pattern — otherwise
    // the compiler cannot tell which elements to ignore, e.g.
    //   let (.., mid, ..) = nums; // ERROR: `..` can only be used once

    // --- Closure parameters are patterns too ---
    // Like function parameters, closure parameters accept any
    // irrefutable pattern — including tuple and struct destructuring.
    // This is why `|(a, b)|` works in iterator chains: the tuple
    // pattern destructures each element directly in the parameter list.
    let pairs = [(1, 2), (3, 4), (5, 6)];
    let sums: Vec<i32> = pairs.iter().map(|(a, b)| a + b).collect();
    println!("closure destructuring sums: {sums:?}");

    // --- matches! macro ---
    // Returns true if a value matches a pattern — concise alternative
    // to `match value { pattern => true, _ => false }`.
    let status = 200;
    println!("is success: {}", matches!(status, 200 | 201));
    println!("is client error: {}", matches!(status, 400..=499));

    // With enums
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Color {
        Red,
        Green,
        Blue,
    }
    let c = Color::Green;
    println!("is warm: {}", matches!(c, Color::Red));

    // With guards
    let score: Option<i32> = Some(85);
    println!("passing grade: {}", matches!(score, Some(s) if s >= 60));

    // Useful in iterator chains for filtering
    let codes = vec![200, 301, 404, 500, 201];
    let successes: Vec<_> = codes.iter().filter(|c| matches!(c, 200..=299)).collect();
    println!("success codes: {successes:?}");

    // --- @ bindings in patterns (deeper coverage) ---
    // @ binds a name to a value that also matches a pattern.
    // Already seen briefly in module 004 (match) and module 002 (slices).
    let age = 25;
    let category = match age {
        n @ 0..=17 => format!("{n} is a minor"),
        n @ 18..=64 => format!("{n} is an adult"),
        n => format!("{n} is a senior"),
    };
    println!("{category}");

    // With enums — bind the inner value while constraining it
    let msg: Option<i32> = Some(42);
    if let Some(n @ 10..=100) = msg {
        println!("@ in if let: {n} is in range");
    }

    // --- ref and ref mut in patterns ---
    // `ref` creates a reference to a value instead of moving/copying it.
    // `ref mut` creates a mutable reference. These are the pattern-side
    // equivalent of `&` and `&mut` on the value side.

    let name = String::from("Alice");
    match name {
        ref n => println!("ref pattern: borrowed {n}"),
        // Without `ref`, `name` would be moved into the match arm.
        // `ref n` binds `n: &String` without moving `name`.
    }
    // `name` is still usable here because `ref` only borrowed it
    println!("name after ref match: {name}");

    let mut score = 100;
    match score {
        ref mut s => *s += 10,
        // `ref mut s` binds `s: &mut i32` — we can modify through it.
    }
    println!("score after ref mut match: {score}"); // 110

    // Note: match ergonomics (shown below) do NOT help the two examples
    // above, because the scrutinee in each case is an owned value
    // (`name`, `score`), not a reference. Default binding modes only kick
    // in when you are matching on a reference like `&String` or `&i32`.

    // In modern Rust (2024 edition), match ergonomics often make `ref`
    // unnecessary — the compiler automatically adds references when
    // matching on a reference. For example:
    let values = vec![String::from("a"), String::from("b")];
    for v in &values {
        // `v` is `&String` automatically — no `ref` needed
        match v {
            s if s.starts_with('a') => println!("match ergonomics: {s}"),
            _ => {}
        }
    }
    // Explicit `ref` is still useful in `let` bindings or when the
    // automatic behavior isn't sufficient, but it's uncommon in
    // idiomatic Rust 2024 code.

    println!("if_let_and_let_else section executed");
}

// =================================================================================================
// Section 11: Vec<T>
// =================================================================================================

/*
## Vec<T>

- `Vec<T>` is a **growable, heap-allocated** array — the most commonly used collection in Rust.
- Creation: `Vec::new()`, `vec![1, 2, 3]`, `Vec::with_capacity(n)` (pre-allocates without
  initializing).
- Modification: `.push(val)`, `.pop()` → `Option<T>`, `.insert(idx, val)`, `.remove(idx)`.
- Access: indexing `vec[i]` **panics** on out-of-bounds. `.get(i)` returns `Option<&T>` for safe
  access. `.first()` and `.last()` return `Option<&T>`.
- Iteration: `for x in &vec` (borrow), `for x in &mut vec` (mutable borrow), `for x in vec`
  (consumes the Vec).
- Slicing: `&vec[1..3]` creates a `&[T]` slice.
- Useful methods: `.len()`, `.is_empty()`, `.contains(&val)`, `.retain(|x| predicate)` (remove
  elements not matching a condition).
- **Capacity**: a Vec has both a `len()` (number of elements) and a `capacity()` (allocated space).
  When len exceeds capacity, the Vec reallocates (typically doubling). Use `Vec::with_capacity(n)`
  or `.reserve(n)` to pre-allocate when you know the size upfront.
- **Borrow checker + reallocation.** Holding any reference to a Vec element (via `&v[i]`,
  `.first()`, `.iter()`, or a `for x in &v` loop) prevents mutating operations like `.push`,
  `.insert`, `.remove`, or `.drain` on the same Vec in that scope. The reason is visceral: when a
  Vec exceeds its capacity, it allocates a new buffer and copies the existing elements there — any
  outstanding reference to an element would then point to deallocated memory. The borrow checker
  rejects such code at compile time. Concretely, `let first = &v[0]; v.push(6);
  println!("{first}");` won't compile — if `push` reallocates, `first` would dangle. The same rule
  forbids `.push`/`.remove` inside a `for` loop over the Vec.
- **Three iteration modes** (important distinction):
  - `.iter()` → `&T` — borrows each element (collection still usable).
  - `.iter_mut()` → `&mut T` — mutably borrows (can modify in place).
  - `.into_iter()` → `T` — consumes the collection (moves elements).
  `for x in &vec` desugars to `.iter()`, `for x in &mut vec` to `.iter_mut()`, and `for x in vec` to
  `.into_iter()`.
*/

fn vec_collection() {
    // Creation
    let mut nums = Vec::<i32>::new();
    nums.push(10);
    nums.push(20);
    nums.push(30);
    println!("after push: {nums:?}");

    // vec! macro shorthand
    let mut letters = vec!['a', 'b', 'c'];

    // pop — returns Option<T>
    let popped = nums.pop();
    println!("popped: {popped:?}, remaining: {nums:?}");

    // insert and remove by index
    letters.insert(1, 'x'); // insert 'x' at index 1
    println!("after insert: {letters:?}");
    letters.remove(1); // remove element at index 1
    println!("after remove: {letters:?}");

    // Safe access with .get() — returns Option<&T>
    match nums.get(10) {
        Some(val) => println!("found: {val}"),
        None => println!("index 10 is out of bounds"),
    }

    // .first() and .last()
    println!("first: {:?}, last: {:?}", nums.first(), nums.last());

    // Iteration by immutable reference
    let colors = vec!["red", "green", "blue"];
    for color in &colors {
        print!("{color} ");
    }
    println!();

    // Iteration by mutable reference — modify in place
    // `score` is `&mut i32`, so `*score` dereferences the reference
    // to access the actual value and modify it in place
    let mut scores = vec![80, 90, 70];
    for score in &mut scores {
        *score += 5;
    }
    println!("adjusted scores: {scores:?}");

    // Slicing — create a &[T] view
    let slice = &colors[0..2];
    println!("slice: {slice:?}");

    // Useful methods
    println!("len: {}, is_empty: {}", colors.len(), colors.is_empty());
    println!("contains \"red\": {}", colors.contains(&"red"));

    // --- Capacity management ---
    let mut preallocated = Vec::with_capacity(100);
    println!(
        "len: {}, capacity: {}",
        preallocated.len(),
        preallocated.capacity()
    );
    preallocated.push(1);
    println!(
        "after push — len: {}, capacity: {}",
        preallocated.len(),
        preallocated.capacity()
    );

    // --- iter() vs iter_mut() vs into_iter() ---
    let words = vec![String::from("hello"), String::from("world")];

    // .iter() → &String — borrows, collection still usable
    for w in words.iter() {
        print!("{w} ");
    }
    println!("(words still usable: {} items)", words.len());

    // .iter_mut() → &mut String — can modify in place
    let mut words = words; // rebind as mutable
    for w in words.iter_mut() {
        w.push('!');
    }
    println!("after iter_mut: {words:?}");

    // .into_iter() → String — consumes the vec, moves each element
    let collected: Vec<String> = words.into_iter().map(|w| w.to_uppercase()).collect();
    println!("into_iter consumed and transformed: {collected:?}");
    // println!("{words:?}"); // ERROR: words was consumed by into_iter

    // retain — keep only elements matching the predicate
    let mut vals = vec![1, 2, 3, 4, 5, 6];
    vals.retain(|x| x % 2 == 0);
    println!("retain evens: {vals:?}");

    // drain — remove a range of elements, returning them as an iterator
    // The removed elements are yielded; the remaining elements stay in the Vec
    let mut data = vec![10, 20, 30, 40, 50];
    let drained: Vec<i32> = data.drain(1..3).collect();
    println!("drained: {drained:?}, remaining: {data:?}"); // [20, 30], [10, 40, 50]

    // --- For-loop desugaring ---
    // `for x in &collection` is syntactic sugar for the Iterator protocol:
    let numbers = vec![1, 2, 3];
    // This for loop:
    //   for x in &numbers { println!("{x}"); }
    // Desugars to:
    let mut iter = numbers.iter();
    loop {
        match iter.next() {
            Some(x) => print!("{x} "),
            None => break,
        }
    }
    println!("← desugared for loop");

    // --- Sorting ---
    let mut nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    nums.sort(); // in-place, stable sort — requires Ord
    println!("sorted: {nums:?}");

    nums.sort_by(|a, b| b.cmp(a)); // descending
    println!("descending: {nums:?}");

    // sort_by_key — sort by a derived key
    let mut words = vec!["banana", "apple", "cherry"];
    words.sort_by_key(|w| w.len());
    println!("sorted by length: {words:?}");

    // Note: f64 does not implement Ord (NaN is not comparable).
    // Use sort_by with partial_cmp:
    // floats.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    // --- windows, chunks, dedup ---
    let data = vec![1, 2, 3, 4, 5];

    // windows(n) — sliding window of n elements
    let pairs: Vec<&[i32]> = data.windows(2).collect();
    println!("windows(2): {pairs:?}"); // [[1,2], [2,3], [3,4], [4,5]]

    // chunks(n) — non-overlapping groups of n (last may be shorter)
    let groups: Vec<&[i32]> = data.chunks(2).collect();
    println!("chunks(2): {groups:?}"); // [[1,2], [3,4], [5]]

    // dedup — removes consecutive duplicates (sort first for full dedup)
    let mut duped = vec![1, 1, 2, 3, 3, 3, 2];
    duped.dedup();
    println!("dedup: {duped:?}"); // [1, 2, 3, 2] — not fully unique!

    println!("vec_collection section executed");
}

// =================================================================================================
// Section 12: HashMap<K, V>
// =================================================================================================

/*
## HashMap<K, V>

- Using separate vectors or tuple-based collections for associated data is error-prone — indexes can
  fall out of sync and duplicates are easy to introduce. `HashMap` solves this with unique key-value
  pairs.
- `HashMap<K, V>` stores key-value pairs with **O(1) average** lookup time. Keys must implement `Eq`
  and `Hash`.
- Requires: `use std::collections::HashMap;`.
- Creation: `HashMap::new()` — specify types via annotation or let the compiler infer from usage.
- Insertion: `.insert(key, value)` — if the key already exists, the old value is **overwritten** and
  returned as `Option<V>`.
- **Ownership on insert.** `HashMap` takes ownership of both keys and values that do not implement
  `Copy`. Calling `map.insert(String::from("k"), String::from("v"))` **moves** both `String` values
  into the map — the original bindings are no longer usable afterward. `Copy` types (like `i32` or
  `&str`) are trivially copied in, so they stay usable. If you need the map to *borrow* instead of
  own, insert references (e.g., `HashMap<&str, &str>`) — but then the referenced data must outlive
  the map, which is a separate lifetime concern.
- Lookup: `.get(&key)` → `Option<&V>` for safe access. Indexing `map[&key]` panics if the key is
  missing.
- `.contains_key(&key)` → `bool`.
- **Entry API**: `.entry(key)` returns an `Entry` enum with `Occupied` (key exists) or `Vacant` (key
  missing) variants. `.or_insert(default)` inserts the default only if vacant and returns a mutable
  reference to the value. `.or_insert_with(|| closure)` for lazy initialization.
- Iteration: `for (key, value) in &map` — order is not guaranteed.
- `.len()`, `.is_empty()`, `.remove(&key)`.
*/

fn hashmap_collection() {
    // Creation and insertion
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 73);
    println!("scores: {scores:?}");

    // Shorthand construction from an array of tuples
    let quick = HashMap::from([("x", 1), ("y", 2), ("z", 3)]);
    println!("from array: {quick:?}");

    // insert overwrites existing values
    let old = scores.insert("Alice", 100);
    println!("Alice's old score: {old:?}");

    // Safe lookup with .get()
    match scores.get("Bob") {
        Some(score) => println!("Bob's score: {score}"),
        None => println!("Bob not found"),
    }

    // contains_key
    println!("has Charlie: {}", scores.contains_key("Charlie"));
    println!("has Dave: {}", scores.contains_key("Dave"));

    // Entry API — insert only if key is absent
    scores.entry("Dave").or_insert(60);
    println!("Dave (new): {:?}", scores.get("Dave"));

    // or_insert does nothing if key already exists
    scores.entry("Dave").or_insert(99);
    println!("Dave (unchanged): {:?}", scores.get("Dave"));

    // or_insert returns a &mut V — use it to modify the value
    let count = scores.entry("Alice").or_insert(0);
    *count += 10; // Alice already exists, modify her score via the ref
    println!("Alice after entry mutation: {:?}", scores.get("Alice"));

    // or_insert_with — lazy initialization (closure runs only when key is absent)
    // Contrast with or_insert(value) which always evaluates the value expression
    scores.entry("Eve").or_insert_with(|| {
        println!("computing Eve's score...");
        88
    });

    // and_modify + or_insert — modify if exists, insert if absent
    // Common for counting occurrences
    let mut word_counts: HashMap<&str, i32> = HashMap::new();
    for word in "the cat sat on the mat".split_whitespace() {
        word_counts.entry(word).and_modify(|c| *c += 1).or_insert(1);
    }
    println!("word counts: {word_counts:?}");

    // Iteration — order is not guaranteed
    for (name, score) in &scores {
        println!("  {name}: {score}");
    }

    // Mutable iteration — modify values in place
    for (_name, score) in scores.iter_mut() {
        *score += 5; // curve all scores
    }
    println!("after curving: {scores:?}");

    // keys() and values() — iterate over just keys or just values
    let keys: Vec<_> = scores.keys().collect();
    let values: Vec<_> = scores.values().collect();
    println!("keys: {keys:?}");
    println!("values: {values:?}");

    // remove
    let removed = scores.remove("Charlie");
    println!("removed Charlie: {removed:?}");
    println!("len after remove: {}", scores.len());

    // --- collect into HashMap from an iterator of tuples ---
    let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
    let map: HashMap<&str, i32> = pairs.into_iter().collect();
    println!("collected HashMap: {map:?}");

    println!("hashmap_collection section executed");
}

// =================================================================================================
// Section 13: Other Standard Collections
// =================================================================================================

/*
## HashSet<T> and Other Standard Collections

- **`HashSet<T>`** — an unordered set of unique values. Provides O(1) lookup, insertion, and
  removal. Values must implement `Eq` and `Hash`. Supports set operations: `.union()`,
  `.intersection()`, `.difference()`, `.symmetric_difference()`. `use std::collections::HashSet;`.
- Other standard collections:
  - **`VecDeque<T>`** — a double-ended queue backed by a growable ring buffer. Efficient push/pop at
    **both** ends. Useful for queues and sliding windows. `use std::collections::VecDeque;`.
  - **`BTreeMap<K, V>`** — a sorted map using a B-tree. Keys are always in sorted order. Lookups are
    O(log n) instead of O(1). `use std::collections::BTreeMap;`.
  - **`BTreeSet<T>`** — a sorted set using a B-tree. Elements are always in sorted order. `use
    std::collections::BTreeSet;`.
- Choose `Hash*` for speed, `BTree*` when you need sorted order.
*/

fn hashset_collection() {
    // Creation and insertion
    let mut colors: HashSet<&str> = HashSet::new();
    colors.insert("red");
    colors.insert("green");
    colors.insert("blue");
    colors.insert("red"); // duplicate — ignored, returns false
    println!("colors: {colors:?} (no duplicates)");
    println!("len: {}", colors.len()); // 3

    // Lookup
    println!("contains red: {}", colors.contains("red"));
    println!("contains yellow: {}", colors.contains("yellow"));

    // From an array
    let a: HashSet<i32> = HashSet::from([1, 2, 3, 4, 5]);
    let b: HashSet<i32> = HashSet::from([3, 4, 5, 6, 7]);

    // Set operations — return iterators, collect into sorted Vecs for display
    let mut union: Vec<_> = a.union(&b).copied().collect();
    union.sort();
    println!("union: {union:?}");

    let mut intersection: Vec<_> = a.intersection(&b).copied().collect();
    intersection.sort();
    println!("intersection: {intersection:?}");

    let mut difference: Vec<_> = a.difference(&b).copied().collect();
    difference.sort();
    println!("a - b: {difference:?}");

    let mut sym_diff: Vec<_> = a.symmetric_difference(&b).copied().collect();
    sym_diff.sort();
    println!("symmetric difference: {sym_diff:?}");

    // Remove
    let mut s = HashSet::from(["a", "b", "c"]);
    s.remove("b");
    println!("after remove: {s:?}");

    // --- VecDeque — efficient push/pop at both ends ---
    use std::collections::VecDeque;
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back("first");
    queue.push_back("second");
    queue.push_front("zeroth"); // efficient — O(1) amortized
    println!("VecDeque: {queue:?}");
    println!("pop_front: {:?}", queue.pop_front()); // Some("zeroth")
    println!("pop_back:  {:?}", queue.pop_back()); // Some("second")

    // --- BTreeMap — sorted by keys ---
    use std::collections::BTreeMap;
    let mut bt = BTreeMap::new();
    bt.insert("cherry", 3);
    bt.insert("apple", 1);
    bt.insert("banana", 2);
    // Iteration is always in sorted key order
    for (fruit, count) in &bt {
        println!("  {fruit}: {count}");
    }

    // --- BTreeSet — sorted unique values ---
    use std::collections::BTreeSet;
    let bts: BTreeSet<i32> = [3, 1, 4, 1, 5].into_iter().collect();
    println!("BTreeSet (sorted, unique): {bts:?}"); // {1, 3, 4, 5}

    println!("hashset_collection section executed");
}

// =================================================================================================
// Section 14: Closures and Iterators
// =================================================================================================

/*
## Closures and Iterators

- Closures are **anonymous functions** that can **capture** variables from their enclosing scope.
  Syntax: `|params| body`.
- Unlike named functions (covered in module 004, which cannot access outer variables), closures
  capture values from the environment by reference, by mutable reference, or by value (move).
- Parameter types and the return type are usually **inferred** by the compiler, but can be
  annotated: `|x: i32| -> i32 { x + 1 }`.
- **Type inference is one-shot and locking**. Closures look like they might be generic, but they are
  **not**: the compiler infers each parameter type and the return type from the closure's **first**
  call site, and from then on those types are frozen. Calling the same closure with a different type
  later produces a `mismatched types` error — even when the body (`|x| x`) would trivially work for
  both. Example:
  ```
  let id = |x| x;
  let s = id(String::from("hi")); // infers x: String, return: String
  let n = id(5); // ERROR: expected `String`, found integer
  ```
  If you need a reusable polymorphic helper, write a generic **fn** instead (`fn id<T>(x: T) -> T {
  x }`) — closures are not the right tool for type-parameterized code. When multiple call types are
  genuinely required, annotate the parameter explicitly (e.g. `|x: i32|`) so the intent is obvious
  at the definition site.
- If the body is a single expression, braces can be omitted: `|x| x + 1`.
- Closures are commonly used with **iterator adapters** to transform collections. Iterators are
  **lazy** — adapters like `.map()` and `.filter()` build a chain that only executes when consumed
  by a method like `.collect()`, `.sum()`, `.for_each()`, or `.fold()`.
- **Consuming vs iterator adapters**. Iterator methods fall into two categories. **Iterator
  adapters** (`.map`, `.filter`, `.take`, `.skip`, `.zip`, `.chain`, `.enumerate`, `.flat_map`, ...)
  are **lazy** — they return a new iterator and do nothing on their own. **Consuming adapters**
  (`.collect`, `.sum`, `.count`, `.fold`, `.for_each`, `.any`, `.all`, `.find`, ...) call `next`
  internally until the iterator is exhausted and produce a final value. A pipeline needs **at least
  one** consuming adapter at the end, otherwise nothing runs.
- **`#[must_use]` warning on dropped adapters**. Iterator adapters are marked `#[must_use]` in the
  standard library, so forgetting the terminal consumer produces a compiler warning:
  ```
  v.iter().map(|x| x + 1); // warning: unused `Map` that must be used
                           // note: iterators are lazy and do nothing
                           //       unless consumed
  ```
  If you intentionally want to trigger side effects for each element without producing a value, use
  `.for_each(|x| ...)` instead of `.map(|x| ...)` — `for_each` is consuming, `map` is not.
- Common adapters: `.map()`, `.filter()`, `.enumerate()`, `.zip()`, `.chain()`, `.flat_map()`,
  `.take()`, `.skip()`, `.fold()`.
- Query methods: `.any()`, `.all()`, `.find()`, `.count()`.
- Use `move` before the parameter list to force the closure to take ownership of captured variables:
  `move |x| x + captured`.
- **Closure trait families** (determines where a closure can be used):
  - `Fn` — can be called multiple times, captures by shared ref.
  - `FnMut` — can be called multiple times, captures by mutable ref.
  - `FnOnce` — can be called only once, may consume captured values.
  Every closure implements `FnOnce`. If it doesn't consume captures, it also implements `FnMut`. If
  it only reads captures, it also implements `Fn`. Functions accepting closures use these traits as
  bounds (covered fully in module 008).
- **The `Iterator` trait**: all iterators implement `trait Iterator { type Item; fn next(&mut self)
  -> Option<Self::Item>; }`. Adapters like `.map()` and `.filter()` are provided as default methods
  on this trait. You can implement `Iterator` for your own types to make them work with `for` loops
  and all adapter methods.
- **The `IntoIterator` trait**: defines how a type is converted into an iterator. `for x in value`
  desugars to `for x in value.into_iter()`. Standard collections implement this automatically. You
  can implement it for your own types to make them work with `for` loops.
- **`Option<T>` implements `IntoIterator`**: `Some(v)` yields one element, `None` yields zero. This
  allows extending collections with Options, chaining iterators with Options, and flattening
  `Vec<Option<T>>` to strip out `None` values.
- **Zero-cost abstractions**: iterator chains compile down to the same machine code as equivalent
  hand-written loops. The compiler inlines and fuses the adapter calls, eliminating the abstraction
  overhead. Benchmarks in the Rust Book show that an iterator chain like
  `.iter().filter().map().sum()` is as fast as a manual `for` loop with an accumulator. This means
  choosing iterators over loops is a style decision, not a performance trade-off.
*/

fn closures_and_iterators() {
    // Basic closure — types are inferred
    let add = |a, b| a + b;
    println!("add(3, 4) = {}", add(3, 4));

    // Closure with explicit type annotations
    let square = |x: i32| -> i32 { x * x };
    println!("square(5) = {}", square(5));

    // Capturing from the environment — closures can read outer variables
    let greeting = "hello";
    let greet = |name| println!("{greeting}, {name}!");
    greet("Rust");

    // Mutable capture — the closure borrows `count` mutably
    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    println!("increment: {}", increment());
    println!("increment: {}", increment());

    // Closures with iterators — map, filter, for_each
    let numbers = [1, 2, 3, 4, 5];
    let doubled: Vec<_> = numbers.iter()
        .map(|x| x * 2)
        .collect();
    println!("doubled: {doubled:?}");

    // .collect() requires the compiler to know the target type.
    // Without annotation, you get: "type annotations needed"
    // let unknown = numbers.iter().collect(); // ERROR
    // Three ways to fix it:
    let _v1: Vec<&i32> = numbers.iter().collect(); // annotate binding
    let _v2 = numbers.iter().collect::<Vec<&i32>>(); // turbofish
    let _v3 = numbers.iter().collect::<Vec<_>>(); // turbofish with _ for element

    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("evens: {evens:?}");

    // --- Iterator adapters ---

    // fold — accumulate a single value from all elements
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("fold sum: {sum}");

    // zip — pair elements from two iterators
    let names = ["Alice", "Bob", "Charlie"];
    let ages = [30, 25, 35];
    let pairs: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("zipped: {pairs:?}");

    // chain — concatenate two iterators
    let first = [1, 2, 3];
    let second = [4, 5, 6];
    let chained: Vec<_> = first.iter().chain(second.iter()).collect();
    println!("chained: {chained:?}");

    // flatten — collapses one level of nesting
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flat: Vec<_> = nested.into_iter().flatten().collect();
    println!("flatten: {flat:?}"); // [1, 2, 3, 4, 5]

    // flatten on Option iterators — filters out None values
    let maybe_values = vec![Some(1), None, Some(3), None, Some(5)];
    let values: Vec<_> = maybe_values.into_iter().flatten().collect();
    println!("flatten Options: {values:?}"); // [1, 3, 5]

    // flat_map — map each element to an iterator, then flatten
    // Equivalent to .map(f).flatten()
    let words = ["hello world", "foo bar"];
    let chars: Vec<&str> = words.iter().flat_map(|s| s.split_whitespace()).collect();
    println!("flat_map: {chars:?}");

    // take and skip — limit or offset the iterator
    let first_three: Vec<_> = (0..10).take(3).collect();
    let skip_five: Vec<_> = (0..10).skip(5).collect();
    println!("take(3): {first_three:?}");
    println!("skip(5): {skip_five:?}");

    // enumerate — (index, value) pairs on any iterator
    for (i, name) in names.iter().enumerate() {
        println!("  [{i}] {name}");
    }

    // --- Query methods ---

    // any — true if any element matches the predicate
    let has_even = numbers.iter().any(|x| x % 2 == 0);
    println!("any even: {has_even}");

    // all — true if every element matches
    let all_positive = numbers.iter().all(|x| *x > 0);
    println!("all positive: {all_positive}");

    // find — returns the first element matching the predicate
    let first_even = numbers.iter().find(|x| *x % 2 == 0);
    println!("first even: {first_even:?}");

    // move closure — takes ownership of captured variables
    let name = String::from("Rust");
    let print_name = move || println!("moved: {name}");
    print_name();
    // println!("{name}"); // ERROR: value used after move

    // --- Closure traits: Fn, FnMut, FnOnce ---
    // The compiler picks the least restrictive capture mode that works:
    //   1. &T (Fn) if the closure only reads
    //   2. &mut T (FnMut) if it mutates
    //   3. T (FnOnce) if it moves/consumes
    // Use `move` to force ownership transfer (e.g., for 'static lifetimes).

    // Fn — only reads captured values (can be called many times)
    let greeting = String::from("hi");
    let say_hi = || println!("{greeting}"); // captures &greeting
    say_hi();
    say_hi(); // can call multiple times

    // FnMut — mutates captured values (can be called many times)
    let mut total = 0;
    let mut accumulate = |x: i32| total += x; // captures &mut total
    accumulate(10);
    accumulate(20);
    println!("FnMut total: {total}"); // 30

    // FnOnce — consumes captured values (can only call once)
    let data = vec![1, 2, 3];
    let consume = move || {
        drop(data); // data is moved into the closure and consumed
        println!("data consumed");
    };
    consume();
    // consume(); // ERROR: cannot call FnOnce closure again

    // --- Custom Iterator ---
    // Implement Iterator for a custom struct
    struct Countdown {
        value: u32,
    }

    impl Iterator for Countdown {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            if self.value == 0 {
                None
            } else {
                self.value -= 1;
                Some(self.value + 1) // return the value before decrement
            }
        }
    }

    let countdown = Countdown { value: 5 };
    let values: Vec<u32> = countdown.collect();
    println!("custom iterator: {values:?}"); // [5, 4, 3, 2, 1]

    // Custom iterator works with all adapters
    let doubled: Vec<u32> = Countdown { value: 3 }.map(|x| x * 2).collect();
    println!("custom doubled: {doubled:?}"); // [6, 4, 2]

    // --- IntoIterator for Custom Types ---
    // IntoIterator is the trait that enables a type to work with `for` loops.
    // `for x in value` desugars to `for x in value.into_iter()`.
    // The trait requires three associated items:
    //   type Item        — the element type produced.
    //   type IntoIter    — the iterator type (must implement Iterator<Item = Self::Item>).
    //   fn into_iter(self) -> Self::IntoIter — creates the iterator, consuming self.
    // Simplest approach: collect fields into a Vec, return std::vec::IntoIter.
    // Distinction from Iterator: Iterator defines how to produce the next element;
    // IntoIterator defines how to create an iterator from a value.

    struct Book {
        title: String,
        author: String,
        genre: String,
    }

    impl IntoIterator for Book {
        type Item = String;
        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            vec![self.title, self.author, self.genre].into_iter()
        }
    }

    // Now Book works with for loops
    let book = Book {
        title: String::from("The Rust Book"),
        author: String::from("Klabnik & Nichols"),
        genre: String::from("Programming"),
    };
    for field in book {
        print!("{field}  ");
    }
    println!("← Book fields via IntoIterator");

    // Also works with iterator adapters after into_iter()
    let book2 = Book {
        title: String::from("Dune"),
        author: String::from("Frank Herbert"),
        genre: String::from("Science Fiction"),
    };
    let uppercased: Vec<String> = book2.into_iter().map(|s| s.to_uppercase()).collect();
    println!("uppercased: {uppercased:?}");

    // Alternative approach (commented out): a dedicated BookIterator struct
    // gives more control over iteration behavior:
    //
    // struct BookIterator { properties: Vec<String> }
    // impl Iterator for BookIterator {
    //     type Item = String;
    //     fn next(&mut self) -> Option<Self::Item> {
    //         if self.properties.is_empty() { None }
    //         else { Some(self.properties.remove(0)) }
    //     }
    // }
    // impl IntoIterator for Book {
    //     type Item = String;
    //     type IntoIter = BookIterator;
    //     fn into_iter(self) -> BookIterator {
    //         BookIterator { properties: vec![self.title, self.author, self.genre] }
    //     }
    // }

    // --- Option as Iterator ---
    // Option<T> implements IntoIterator — Some(v) yields one element,
    // None yields zero. This lets Option interoperate with iterator APIs.

    // Pattern 1: extend a Vec with an Option — pushes the value if Some
    let mut items = vec!["phone", "battery"];
    let extra: Option<&str> = Some("charger");
    let missing: Option<&str> = None;
    items.extend(extra); // pushes "charger"
    items.extend(missing); // does nothing
    println!("after extend: {items:?}"); // ["phone", "battery", "charger"]

    // Pattern 2: chain an iterator with an Option — appends 0 or 1 elements
    let base = vec!["apple", "banana"];
    let bonus: Option<&str> = Some("cherry");
    let all: Vec<&str> = base.iter().copied().chain(bonus.iter().copied()).collect();
    println!("chained with Option: {all:?}"); // ["apple", "banana", "cherry"]

    // Pattern 3: flatten a Vec<Option<T>> — strips out None values
    let maybe_items = vec![Some("pen"), None, Some("notebook"), None, Some("eraser")];
    let definite: Vec<&str> = maybe_items.into_iter().flatten().collect();
    println!("flattened options: {definite:?}"); // ["pen", "notebook", "eraser"]

    // --- Realistic chained pipeline ---
    let text = "  hello world  foo  bar  baz  hello  ";
    let unique_words: Vec<&str> = text
        .split_whitespace() // split into words
        .filter(|w| w.len() > 3) // keep words longer than 3 chars
        .collect::<HashSet<&str>>() // deduplicate
        .into_iter()
        .collect();
    println!("unique long words: {unique_words:?}");

    // --- collect into Result<Vec<T>, E> ---
    // When iterating over Results, collect can short-circuit:
    // if all Ok → Ok(Vec<T>), if any Err → first Err.
    let inputs = vec!["1", "2", "3"];
    let parsed: Result<Vec<i32>, _> = inputs.iter().map(|s| s.parse::<i32>()).collect();
    println!("all valid: {parsed:?}"); // Ok([1, 2, 3])

    let bad_inputs = vec!["1", "x", "3"];
    let parsed: Result<Vec<i32>, _> = bad_inputs.iter().map(|s| s.parse::<i32>()).collect();
    println!("has error: {parsed:?}"); // Err(ParseIntError)

    // Forward reference: error handling with custom error types
    // and the From trait for error conversion is covered in module 009.

    // --- Zero-cost abstractions ---
    // Iterator chains compile to the same code as hand-written loops.
    // These two are equivalent after optimization:

    // Iterator style:
    let sum_iter: i32 = (1..=1000).filter(|n| n % 2 == 0).sum();

    // Equivalent loop style:
    let mut sum_loop: i32 = 0;
    for n in 1..=1000 {
        if n % 2 == 0 {
            sum_loop += n;
        }
    }
    assert_eq!(sum_iter, sum_loop);
    println!("zero-cost: iterator sum = {sum_iter}, loop sum = {sum_loop}");

    // The compiler inlines and fuses .filter().sum() into a single loop
    // with no closure allocation or intermediate collection. Choosing
    // iterators over loops is a style decision, not a performance trade-off.

    println!("closures_and_iterators section executed");
}

// =================================================================================================
// Section 15: Advanced Iterator Patterns
// =================================================================================================

/*
## Advanced Iterator Patterns

- Section 14 introduced closures and iterator fundamentals with simple data (numbers, strings). This
  section demonstrates **realistic applications** on struct data — patterns used daily in production
  code.

### Finding Elements by Properties

- **`min_by_key(|item| key)` / `max_by_key(|item| key)`** — returns the element with the
  smallest/largest value of the key function. Returns `Option<&T>` (None if the iterator is empty).
  The key must implement `Ord`; for `f64` salaries, cast to `i64` or use `min_by` / `max_by` with a
  custom comparator instead.

### Partitioning Collections

- **`partition(|item| predicate)`** — splits an iterator into two collections: elements where the
  predicate is true, and where it is false. Returns a tuple `(TrueCollection, FalseCollection)`. The
  target type must be specified (often via type annotation on the let binding): `let (a, b):
  (Vec<_>, Vec<_>) = iter.partition(...)`.

### Custom Sorting

- **`sort_by(|a, b| comparator)`** — sorts a `Vec` in-place using a comparator that returns
  `std::cmp::Ordering` (Less, Equal, Greater).
- `f64` implements `PartialOrd` but not `Ord` (because `NaN != NaN`), so you must use
  `.partial_cmp(&b).unwrap_or(Ordering::Equal)` in the comparator. Swap `a` and `b` to sort
  descending.
- Sorting is **not** an iterator adapter — it operates on `Vec` directly. Typically: collect into a
  Vec, then sort.

### Advanced Fold Patterns

- **Function pointers as accumulators**: pass a function directly instead of a closure:
  `fold(f64::INFINITY, f64::min)` finds the minimum. Works because `f64::min` has signature `(f64,
  f64) -> f64`.
- **Multi-value accumulators**: use a tuple to track several values in one pass. Example:
  `fold((0.0, 0), |(sum, count), x| ...)` computes both sum and count for computing an average.
- **Building collections**: fold can construct a `HashMap` or `Vec`. Example: group items by a field
  with `fold(HashMap::new(), |mut map, item| { map.entry(...) ... })`.
- Many fold use-cases have dedicated methods (`sum`, `count`, `min`, `max`). Use fold when no
  specialized method fits.

### for_each vs for Loops

- **`.for_each(|item| ...)`** — consumes the iterator, running a closure on each element. Used for
  side effects (printing, logging). Cannot `break` early (use a `for` loop for that). Main
  advantage: chains cleanly at the end of a pipeline: `iter.filter(...).map(...).for_each(...)`.

### iter() vs into_iter() in Practice

- **`.iter()`** → yields `&T`. The collection is borrowed, still usable afterwards. Use when you
  only need to read.
- **`.into_iter()`** → yields `T`. The collection is consumed and no longer usable. Use when you
  need owned values (e.g., to move elements into a new `Vec` via `collect()`).
- Chaining example: `vec.into_iter().filter(...).collect()` moves matching elements into a new Vec;
  the original is gone.

### Turbofish Syntax

- **`::<Type>`** specifies generic type parameters when the compiler cannot infer them. Common with
  `collect()`: `.collect::<Vec<_>>()` tells collect to build a Vec (the `_` lets the compiler infer
  the element type).
- Named after the emoticon `::<>` which looks like a fish.
*/

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Employee {
    id: u32,
    name: String,
    department: String,
    salary: f64,
    age: u32,
    years_experience: u32,
    is_remote: bool,
}

impl Employee {
    fn new(
        id: u32,
        name: &str,
        department: &str,
        salary: f64,
        age: u32,
        years_experience: u32,
        is_remote: bool,
    ) -> Self {
        Employee {
            id,
            name: name.to_string(),
            department: department.to_string(),
            salary,
            age,
            years_experience,
            is_remote,
        }
    }
}

fn create_employees() -> Vec<Employee> {
    vec![
        Employee::new(1, "Alice Johnson", "Engineering", 95000.0, 28, 5, true),
        Employee::new(2, "Bob Smith", "Marketing", 65000.0, 32, 8, false),
        Employee::new(3, "Carol Davis", "Engineering", 110000.0, 35, 12, true),
        Employee::new(4, "David Wilson", "Sales", 75000.0, 29, 6, false),
        Employee::new(5, "Eve Brown", "Engineering", 88000.0, 26, 3, true),
        Employee::new(6, "Frank Miller", "HR", 72000.0, 41, 15, false),
        Employee::new(7, "Grace Lee", "Marketing", 68000.0, 30, 7, true),
        Employee::new(8, "Henry Garcia", "Engineering", 105000.0, 33, 9, false),
        Employee::new(9, "Ivy Martinez", "Sales", 82000.0, 27, 4, true),
        Employee::new(10, "Jack Anderson", "Engineering", 92000.0, 31, 7, false),
        Employee::new(11, "Kate Thompson", "HR", 78000.0, 36, 11, true),
        Employee::new(12, "Liam White", "Marketing", 71000.0, 28, 5, false),
        Employee::new(13, "Mia Harris", "Sales", 79000.0, 34, 10, true),
        Employee::new(14, "Noah Clark", "Engineering", 98000.0, 29, 6, false),
        Employee::new(15, "Olivia Lewis", "Marketing", 64000.0, 25, 2, true),
        Employee::new(16, "Paul Walker", "Sales", 85000.0, 38, 13, false),
        Employee::new(17, "Quinn Hall", "HR", 76000.0, 33, 9, true),
        Employee::new(18, "Ruby Allen", "Engineering", 115000.0, 37, 14, false),
        Employee::new(19, "Sam Young", "Marketing", 69000.0, 31, 8, true),
        Employee::new(20, "Tina King", "Sales", 81000.0, 30, 7, false),
    ]
}

fn advanced_iterator_patterns() {
    let employees = create_employees();

    // 1. for_each — iterate with side effects at the end of a pipeline
    println!("1. All employee names:");
    employees.iter().for_each(|e| println!("   {}", e.name));

    // 2. filter on struct fields
    println!("\n2. High earners (salary > $80,000):");
    employees
        .iter()
        .filter(|e| e.salary > 80_000.0)
        .for_each(|e| println!("   {} — ${}", e.name, e.salary));

    // 3. map to extract tuples
    println!("\n3. Name and salary pairs:");
    let pairs: Vec<_> = employees.iter().map(|e| (&e.name, e.salary)).collect();
    println!("   {pairs:?}");

    // 4. find — first element matching a predicate
    println!("\n4. First remote employee:");
    if let Some(e) = employees.iter().find(|e| e.is_remote) {
        println!("   {} ({})", e.name, e.department);
    }

    // 5. min_by_key / max_by_key — find elements with extreme property values
    //    Key must implement Ord, so cast f64 salary to integer cents.
    println!("\n5. Employee statistics:");
    let youngest = employees.iter().min_by_key(|e| e.age);
    let oldest = employees.iter().max_by_key(|e| e.age);
    println!(
        "   Age range: {} - {}",
        youngest.map(|e| e.age).unwrap_or_default(),
        oldest.map(|e| e.age).unwrap_or_default()
    );

    // For f64, use fold with f64::min / f64::max as function pointers
    let min_salary = employees
        .iter()
        .map(|e| e.salary)
        .fold(f64::INFINITY, f64::min);
    let max_salary = employees.iter().map(|e| e.salary).fold(0.0, f64::max);
    println!("   Salary range: ${min_salary} - ${max_salary}");

    // 6. fold with multi-value accumulator — compute average in one pass
    println!("\n6. Average salary (using fold):");
    let (total, count) = employees
        .iter()
        .fold((0.0, 0), |(sum, count), e| (sum + e.salary, count + 1));
    println!("   Avg salary: ${:.2}", total / count as f64);

    // 7. fold to build a HashMap — group employees by department
    println!("\n7. Employees by department:");
    let by_department = employees.iter().fold(HashMap::new(), |mut acc, e| {
        acc.entry(&e.department).or_insert_with(Vec::new).push(e);
        acc
    });
    for (dept, members) in &by_department {
        println!(
            "   {dept}: {}",
            members
                .iter()
                .map(|e| e.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    // 8. chain — concatenate two filtered iterators
    println!("\n8. Senior employees (10+ years) OR high earners (90k+):");
    let seniors = employees.iter().filter(|e| e.years_experience >= 10);
    let high_earners = employees.iter().filter(|e| e.salary >= 90_000.0);
    seniors
        .chain(high_earners)
        .for_each(|e| println!("   {} — {} yrs, ${}", e.name, e.years_experience, e.salary));

    // 9. enumerate + take — first N elements with their index
    println!("\n9. First 5 employees with index:");
    employees
        .iter()
        .enumerate()
        .take(5)
        .for_each(|(i, e)| println!("   [{i}] {}", e.name));

    // 10. sort_by with partial_cmp + zip with an infinite range for ranking
    //     sort_by operates on Vec, not on an iterator adapter.
    println!("\n10. Top 5 employees by salary:");
    let mut sorted = employees.clone();
    sorted.sort_by(|a, b| b.salary.partial_cmp(&a.salary).unwrap_or(Ordering::Equal));
    (1..)
        .zip(sorted.iter())
        .take(5)
        .for_each(|(rank, e)| println!("   #{rank}: {} — ${}", e.name, e.salary));

    // 11. partition — split into two collections based on a predicate
    println!("\n11. Remote vs Office workers:");
    let (remote, office): (Vec<&Employee>, Vec<&Employee>) =
        employees.iter().partition(|e| e.is_remote);
    println!("   Remote: {}, Office: {}", remote.len(), office.len());

    // 12. Complex fold — department salary statistics (min, max, count)
    println!("\n12. Department salary statistics:");
    let dept_stats = employees.iter().fold(
        HashMap::new(),
        |mut acc: HashMap<&String, (f64, f64, usize)>, e| {
            let entry = acc.entry(&e.department).or_insert((f64::INFINITY, 0.0, 0));
            entry.0 = entry.0.min(e.salary);
            entry.1 = entry.1.max(e.salary);
            entry.2 += 1;
            acc
        },
    );
    for (dept, (min, max, count)) in &dept_stats {
        println!("   {dept}: count={count}, salary=${min} - ${max}");
    }

    // 13. all / any — boolean queries across the collection
    println!("\n13. Boolean checks:");
    let all_adults = employees.iter().all(|e| e.age >= 18);
    let any_remote = employees.iter().any(|e| e.is_remote);
    let any_millionaire = employees.iter().any(|e| e.salary >= 1_000_000.0);
    println!("   All adults (18+): {all_adults}");
    println!("   Any remote: {any_remote}");
    println!("   Any millionaires: {any_millionaire}");

    // 14. skip + take — pagination pattern
    println!("\n14. Pagination (page 2, 5 per page):");
    let page = 2;
    let page_size = 5;
    employees
        .iter()
        .skip((page - 1) * page_size)
        .take(page_size)
        .for_each(|e| println!("   {} — {}", e.name, e.department));

    // 15. Complex pipeline — into_iter (consumes the vec), multi-filter,
    //     collect into a new Vec, sort, and display.
    println!("\n15. Engineering, remote, salary > 90k, sorted by experience:");
    let mut results = employees
        .into_iter()
        .filter(|e| e.department == "Engineering")
        .filter(|e| e.is_remote)
        .filter(|e| e.salary > 90_000.0)
        .collect::<Vec<_>>();
    results.sort_by(|a, b| b.years_experience.cmp(&a.years_experience));
    results
        .iter()
        .for_each(|e| println!("   {} — {} yrs, ${}", e.name, e.years_experience, e.salary));

    // 16. Collect to different types — same iterator, different targets
    let names = ["alice", "bob", "alice", "charlie", "bob"];
    let as_vec: Vec<&str> = names.iter().copied().collect();
    let as_set: HashSet<&str> = names.iter().copied().collect();
    println!("\n16. Same data, different collections:");
    println!("   Vec (preserves order/dupes): {as_vec:?}");
    println!("   HashSet (unique only): {as_set:?}");

    // --- peekable, scan, unzip ---

    // peekable — look at the next element without consuming it
    let mut iter = vec![1, 2, 3].into_iter().peekable();
    println!("peek: {:?}", iter.peek()); // Some(&1)
    println!("next: {:?}", iter.next()); // Some(1)
    println!("peek after next: {:?}", iter.peek()); // Some(&2)

    // scan — like fold but yields intermediate states
    let running_sum: Vec<i32> = vec![1, 2, 3, 4]
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();
    println!("running sum: {running_sum:?}"); // [1, 3, 6, 10]

    // unzip — split an iterator of pairs into two collections
    let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c')];
    let (nums, chars): (Vec<i32>, Vec<char>) = pairs.into_iter().unzip();
    println!("unzip nums: {nums:?}, chars: {chars:?}");

    println!("advanced_iterator_patterns section executed");
}

pub fn run() {
    struct_types();
    struct_features();
    methods_and_associated_functions();
    builder_pattern();
    fallible_constructors();
    enums();
    option_type();
    result_type();
    if_let_and_let_else();
    vec_collection();
    hashmap_collection();
    hashset_collection();
    closures_and_iterators();
    advanced_iterator_patterns();
}
