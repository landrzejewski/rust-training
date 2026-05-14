use std::fmt;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

// =================================================================================================
// Section 1: Error Handling Philosophy
// =================================================================================================

/*
## Error Handling Philosophy

- Rust handles errors as **explicit values** in normal control flow. There is no try/catch mechanism
  — errors are returned, not thrown.
- Errors fall into two categories:
  - **Unrecoverable**: the program is in an invalid state and must stop. Handled with `panic!` and
    related macros (`todo!`, `unimplemented!`, `unreachable!`) — covered in module 005.
  - **Recoverable**: the operation failed but the program can continue. Handled with `Result<T, E>`
    and `Option<T>` — basic covered in module 007 (sections 8-10).
- **When to panic vs. return `Result`:**
  - Panic when the program reaches an invalid state that represents a bug: violated invariants,
    corrupted data, or impossible conditions.
  - Return `Result` when failure is an **expected possibility** that the caller should decide how to
    handle: missing files, invalid user input, network timeouts, parse errors.
  - **Library code** should almost always return `Result` — let the caller choose the recovery
    strategy.
  - **Application code** may panic at the top level for truly unrecoverable configuration errors
    (e.g., missing required database connection at startup).
- **`.unwrap()` vs `.expect(msg)` shortcuts.** Both methods on `Result` (and `Option`) unwrap the
  success value or panic on failure. `.unwrap()` uses a generic panic message; `.expect(msg)` lets
  you supply custom context. **Prefer `.expect()` in production code** — a good message (e.g.,
  "config file must exist at startup") turns a crash report into an immediate diagnosis. Use
  `.unwrap()` only in throwaway prototypes, examples, and tests where brevity matters more than the
  error message.
- **When you know more than the compiler.** Sometimes an operation returns `Result` but in *your*
  specific context success is logically guaranteed — e.g., `"127.0.0.1".parse::<IpAddr>()` on a
  hardcoded, valid literal. The compiler still forces you to handle the `Result`. In these cases
  `.expect("reason the Err branch is impossible here")` is idiomatic — the message documents your
  assumption, and if that assumption is ever broken (e.g., the literal is later changed or the input
  starts coming from elsewhere), the panic message immediately tells the next reader why it
  happened.
- **Encoding invariants in types (contract-violation panics).** When a value must always satisfy
  some invariant (e.g., "between 1 and 100", "non-empty", "well-formed percentage"), you can encode
  that invariant directly in a wrapper type: make the field private, validate in the constructor,
  and expose read access through a getter. If validation fails, **`panic!` in the constructor** —
  the caller has violated the contract (a bug, not a user error). Downstream functions taking the
  wrapper type as a parameter can then **trust** the invariant without re-checking. This is distinct
  from the `Result`-returning fallible constructor pattern (module 007 section 5, `Student:: new`):
  use `Result` when invalid input is an expected possibility, and `panic!` when it indicates a
  programmer bug.
- **Runtime knobs for panic.** By default a panic **unwinds** the stack — running destructors for
  every value in scope, which is necessary for `Drop`-based cleanup. You can switch to
  **abort-on-panic** for smaller binaries and faster panics by adding `panic = "abort"` to
  `[profile.release]` (or any profile) in `Cargo.toml`; the OS reclaims memory instead of Rust. For
  debugging a panic, set `RUST_BACKTRACE=1` (or `full`) before running the program — Rust prints the
  call stack leading to the panic. Debug symbols must be enabled (default in `cargo build` / `cargo
  run` without `--release`) for the backtrace to be readable.
- This module builds on the basics from modules 005, 007, and 008 to cover **error propagation**,
  **combining multiple error types**, **custom error types**, and **crate-based error handling**
  with thiserror and anyhow.
*/

// Panic approach — aborts on invalid input (appropriate for bugs/invariants)
fn validate_age_panic(age: i32) -> i32 {
    if age < 0 {
        panic!("age cannot be negative, got {age}");
    }
    age
}

// Result approach — returns an error for invalid input (appropriate for user data)
fn validate_age_result(age: i32) -> Result<i32, String> {
    if age < 0 {
        Err(format!("age cannot be negative, got {age}"))
    } else {
        Ok(age)
    }
}

fn error_handling_philosophy() {
    // Panic approach: would crash if called with -5
    let valid = validate_age_panic(25);
    println!("panic approach with valid input: {valid}");

    // Result approach: caller decides how to handle the error
    match validate_age_result(-5) {
        Ok(age) => println!("valid age: {age}"),
        Err(e) => println!("handled error gracefully: {e}"),
    }

    match validate_age_result(30) {
        Ok(age) => println!("valid age: {age}"),
        Err(e) => println!("error: {e}"),
    }

    println!("error_handling_philosophy section executed");
}

// =================================================================================================
// Section 2: Error Propagation
// =================================================================================================

/*
## Error Propagation

- The `?` operator was introduced in module 007 for single operations. Its real power emerges when
  **propagating errors through chains of multiple fallible operations** in a single function.
- `?` after a `Result<T, E>`: if `Err(e)`, the function returns `Err(e)` immediately. If `Ok(v)`,
  the expression evaluates to `v` and execution continues to the next line.
- `?` after an `Option<T>`: if `None`, the function returns `None` immediately. If `Some(v)`,
  evaluates to `v`.
- Each `?` is a potential early return point. The "happy path" reads linearly from top to bottom —
  all error handling is implicit.
- **Implicit `From` conversion**: when a function returns `Result<_, E2>` and the expression yields
  `Result<_, E1>`, the `?` operator calls `E2::from(e1)` automatically. This means `E2` must
  implement `From<E1>`. This becomes critical in sections 3-4 when dealing with multiple error
  types.
- **Not every `Err` is the same — inspect `io::ErrorKind`.** Propagating with `?` is the common
  case, but sometimes you want different recovery for different failure modes. `std::io::Error`
  exposes `.kind() -> std::io::ErrorKind`, a non-exhaustive enum with variants like `NotFound`,
  `PermissionDenied`, `AlreadyExists`, `TimedOut`, `Interrupted`, etc. Classic pattern: *"if the
  cache file doesn't exist, create it; any other I/O error should propagate"* — `match err.kind() {
  ErrorKind::NotFound => …, _ => return Err(err.into()) }`. Other standard error types expose
  similar classification APIs (e.g., `ParseIntError::kind()` returns `IntErrorKind`).
*/

// Two ? operations: file open can fail, read_to_string can fail
fn read_config_value(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// ? with Option: find returns Option<usize>, get returns Option<&str>
fn extract_domain(email: &str) -> Option<&str> {
    let at_pos = email.find('@')?;
    let domain = email.get(at_pos + 1..)?;
    Some(domain)
}

fn error_propagation() {
    // Result propagation — two fallible operations chained with ?
    match read_config_value("nonexistent.txt") {
        Ok(val) => println!("config value: {val}"),
        Err(e) => println!("config error (expected): {e}"),
    }

    // Option propagation — two fallible lookups chained with ?
    println!(
        "domain of user@example.com: {:?}",
        extract_domain("user@example.com")
    );
    println!(
        "domain of invalid-email: {:?}",
        extract_domain("invalid-email")
    );
    println!("domain of empty string: {:?}", extract_domain(""));
    // Edge case: "user@" has an '@' but the domain is empty — returns Some("")
    // This motivates the improved extract_domain_or_err below that rejects empty domains.
    println!("domain of user@: {:?}", extract_domain("user@"));

    // --- Option::ok_or / ok_or_else for ? propagation ---
    // When a function returns Result but an intermediate step returns
    // Option, convert None into a specific error with ok_or/ok_or_else.
    fn extract_domain_or_err(email: &str) -> Result<&str, String> {
        let at_pos = email.find('@').ok_or("missing @ symbol")?;
        let domain = email
            .get(at_pos + 1..)
            .ok_or_else(|| format!("invalid domain in '{email}'"))?;
        if domain.is_empty() {
            return Err("empty domain".to_string());
        }
        Ok(domain)
    }

    println!(
        "ok_or chain: {:?}",
        extract_domain_or_err("user@example.com")
    );
    println!("ok_or error: {:?}", extract_domain_or_err("no-at-sign"));

    // --- Result::and_then ---
    // Like map(), but the closure returns a Result, avoiding nested
    // Result<Result<...>>. Chains fallible operations.
    let result: Result<i32, String> =
        "42".parse::<i32>()
            .map_err(|e| e.to_string())
            .and_then(|n| {
                if n > 0 {
                    Ok(n)
                } else {
                    Err("must be positive".into())
                }
            });
    println!("and_then: {result:?}");

    println!("error_propagation section executed");
}

// =================================================================================================
// Section 3: Multiple Error Types
// =================================================================================================

/*
## Multiple Error Types

- A function often calls operations that return **different error types**. For example, `File::open`
  returns `io::Error` while `str::parse` returns `ParseIntError`. Using `?` requires all errors to
  resolve to a **single** return error type.
- **Approach 1: `map_err()` at each call site.** Convert each error to a variant of a shared enum at
  the point where `?` is used. The original function signatures remain unchanged.
- **Approach 2: Custom error enum with `From` implementations.** Define an enum with one variant per
  error kind. Implement `From<OriginalError>` for the enum. The `?` operator then calls
  `From::from()` automatically — no `map_err()` needed.
- Approach 2 is generally preferred: conversions are defined once (DRY), new call sites get
  automatic conversion, and it separates error definition from usage. The `From` trait itself is
  covered in module 008; here we show its practical application for errors.
*/

// Two functions that return different error types
fn temperature_from_sensor(sensor_id: u32) -> Result<f64, u8> {
    match sensor_id {
        1 => Ok(22.5),
        2 => Ok(30.0),
        _ => Err(42), // sensor error code
    }
}

fn convert_to_fahrenheit(celsius: f64) -> Result<f64, String> {
    if !(-100.0..=100.0).contains(&celsius) {
        Err(format!("temperature {celsius}°C is out of range"))
    } else {
        Ok(celsius * 1.8 + 32.0)
    }
}

#[derive(Debug)]
#[allow(dead_code)] // Fields read via Debug formatting in examples
enum TemperatureError {
    Sensor(u8),
    Conversion(String),
}

// Approach 1: map_err at each call site
fn get_temp_with_map_err(sensor_id: u32) -> Result<f64, TemperatureError> {
    let celsius = temperature_from_sensor(sensor_id).map_err(TemperatureError::Sensor)?;
    let fahrenheit = convert_to_fahrenheit(celsius).map_err(TemperatureError::Conversion)?;
    Ok(fahrenheit)
}

// Approach 2: From implementations — ? converts automatically
impl From<u8> for TemperatureError {
    fn from(code: u8) -> Self {
        TemperatureError::Sensor(code)
    }
}

impl From<String> for TemperatureError {
    fn from(msg: String) -> Self {
        TemperatureError::Conversion(msg)
    }
}

fn get_temp_with_from(sensor_id: u32) -> Result<f64, TemperatureError> {
    // No map_err needed — ? calls From::from() automatically
    let celsius = temperature_from_sensor(sensor_id)?;
    let fahrenheit = convert_to_fahrenheit(celsius)?;
    Ok(fahrenheit)
}

fn multiple_error_types() {
    // Both approaches produce the same result
    match get_temp_with_map_err(1) {
        Ok(f) => println!("map_err approach: {f:.1}°F"),
        Err(e) => println!("map_err error: {e:?}"),
    }

    match get_temp_with_from(1) {
        Ok(f) => println!("From approach: {f:.1}°F"),
        Err(e) => println!("From error: {e:?}"),
    }

    // Invalid sensor triggers Sensor error variant
    match get_temp_with_from(99) {
        Ok(f) => println!("temp: {f:.1}°F"),
        Err(e) => println!("sensor error (expected): {e:?}"),
    }

    println!("multiple_error_types section executed");
}

// =================================================================================================
// Section 4: Custom Error Types
// =================================================================================================

/*
## Custom Error Types

- A well-defined custom error type implements three things:
  1. **`Debug`** — typically derived with `#[derive(Debug)]`.
  2. **`fmt::Display`** — provides the human-readable error message.
  3. **`std::error::Error`** — the standard error trait. Its supertrait requirements are `Display +
     Debug` (supertraits are covered in module 008 section 9).
- The `std::error::Error` trait is what makes a type a "proper" Rust error. It enables:
  - Storing errors as `Box<dyn std::error::Error>` (type-erased).
  - Chaining errors via `source()` for debugging and logging.
  - Compatibility with the broader Rust error ecosystem.
- Without `std::error::Error`, your enum is just a plain enum — it cannot be used with `Box<dyn
  Error>`, anyhow, or other error infrastructure.
- `From` impls for each source error type enable the `?` operator to convert automatically (as shown
  in section 3).
- This is the "full manual" approach. Sections 7-8 show how thiserror and anyhow reduce this
  boilerplate.
*/

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(ParseIntError),
}

// Display: human-readable error messages
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "I/O error: {e}"),
            AppError::Parse(e) => write!(f, "parse error: {e}"),
        }
    }
}

// std::error::Error: makes AppError a proper Rust error type.
// source() returns the underlying error for error chain traversal.
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::Parse(e) => Some(e),
        }
    }
}

// From impls: enable ? to convert io::Error and ParseIntError automatically
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

// With all three traits + From impls, ? works seamlessly
fn read_and_parse(path: &str) -> Result<i32, AppError> {
    let mut file = File::open(path)?; // io::Error -> AppError via From
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // io::Error -> AppError via From
    let number = contents.trim().parse::<i32>()?; // ParseIntError -> AppError via From
    Ok(number)
}

fn custom_error_types() {
    match read_and_parse("nonexistent.txt") {
        Ok(n) => println!("parsed number: {n}"),
        Err(e) => {
            // Display: human-readable message
            println!("error (Display): {e}");

            // source(): access the underlying error
            if let Some(source) = std::error::Error::source(&e) {
                println!("caused by: {source}");
            }

            // --- Error chain iteration ---
            // Walk the chain of source errors for full diagnostics
            let mut current: Option<&dyn std::error::Error> = Some(&e);
            let mut depth = 0;
            while let Some(err) = current {
                println!("  [{depth}] {err}");
                current = err.source();
                depth += 1;
            }

            // Match on variants for different recovery strategies
            match e {
                AppError::Io(_) => println!("recovery: could create the file"),
                AppError::Parse(_) => println!("recovery: could prompt user for valid input"),
            }
        }
    }

    println!("custom_error_types section executed");
}

// =================================================================================================
// Section 4b: Box<dyn Error> — Quick Type-Erased Errors
// =================================================================================================

/*
## Box<dyn Error>

- An alternative to defining custom error enums (section 4) or reaching for crates (sections 7–8),
  `Box<dyn std::error::Error>` provides the **simplest** way to return any error type from a
  function.
- The `?` operator automatically converts any `E: Error` into `Box<dyn Error>`, so multiple error
  types Just Work.
- **Trade-off**: callers cannot `match` on specific error variants — all type information is erased.
  This is fine for `main()`, scripts, and prototyping but not ideal for library APIs.
- Progression: manual enum → Box<dyn Error> → thiserror → anyhow.
*/

fn box_dyn_error() {
    use std::error::Error;
    use std::fs;

    // One function, two different error types — no custom enum needed
    fn read_first_number(path: &str) -> Result<i32, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?; // std::io::Error
        let n: i32 = contents.trim().parse()?; // ParseIntError
        Ok(n)
    }

    match read_first_number("nonexistent.txt") {
        Ok(n) => println!("parsed number: {n}"),
        Err(e) => println!("box dyn error (expected): {e}"),
    }

    // Also commonly used as the return type of main():
    // fn main() -> Result<(), Box<dyn Error>> { ... }

    println!("box_dyn_error section executed");
}

// =================================================================================================
// Section 5: Method Chaining with ?
// =================================================================================================

/*
## Method Chaining with ?

- Methods that return `Result<&Self, E>` or `Result<&mut Self, E>` enable **chaining fallible
  operations** with `?`.
- Each `?` unwraps the `Ok` value (the reference to self), allowing the next method call on the same
  object.
- If any step fails, the chain **short-circuits** and returns the error immediately.
- Borrowing constraints apply: a method taking `&self` (shared borrow) can precede a chain of
  methods taking `&mut self` (mutable borrows), because the shared borrow ends before the mutable
  chain begins.
*/

struct Order {
    item_count: i32,
    price_per_item: f64,
    balance: f64,
    completed: bool,
}

#[derive(Debug)]
enum OrderError {
    InvalidOrder,
    PaymentFailed,
    ShippingError,
}

impl Order {
    // Shared borrow — does not modify the order
    fn validate(&self) -> Result<(), OrderError> {
        if self.item_count == 0 {
            Err(OrderError::InvalidOrder)
        } else {
            Ok(())
        }
    }

    // Mutable borrow — deducts the payment
    fn process_payment(&mut self) -> Result<&mut Self, OrderError> {
        let total = self.item_count as f64 * self.price_per_item;
        if self.balance < total {
            Err(OrderError::PaymentFailed)
        } else {
            self.balance -= total;
            Ok(self)
        }
    }

    // Mutable borrow — marks the order as completed
    fn ship_order(&mut self) -> Result<&mut Self, OrderError> {
        if self.item_count > 10 {
            Err(OrderError::ShippingError)
        } else {
            self.completed = true;
            Ok(self)
        }
    }

    // Orchestrator: validate (shared borrow ends), then chain mutable operations
    fn complete_order(&mut self) -> Result<(), OrderError> {
        self.validate()?;
        self.process_payment()?.ship_order()?;
        Ok(())
    }
}

fn method_chaining_with_question_mark() {
    // Successful order
    let mut order = Order {
        item_count: 3,
        price_per_item: 25.0,
        balance: 200.0,
        completed: false,
    };
    match order.complete_order() {
        Ok(()) => println!("order completed, remaining balance: {:.2}", order.balance),
        Err(e) => println!("order failed: {e:?}"),
    }

    // Payment failure — insufficient balance
    let mut order = Order {
        item_count: 5,
        price_per_item: 100.0,
        balance: 50.0,
        completed: false,
    };
    match order.complete_order() {
        Ok(()) => println!("order completed"),
        Err(e) => println!("order failed (expected): {e:?}"),
    }

    println!("method_chaining_with_question_mark section executed");
}

// =================================================================================================
// Section 6: Layering Result and Option
// =================================================================================================

/*
## Layering Result and Option

- **`Result<Option<T>, E>`** — three-way outcome:
  - `Ok(Some(value))` — operation succeeded, data found.
  - `Ok(None)` — operation succeeded, no data (expected absence).
  - `Err(e)` — operation failed (actual error).
  - Use case: database queries, API lookups — the query itself might fail (error), succeed and find
    nothing (None), or find data (Some).

- **`Option<Result<T, E>>`** — optional fallible operation:
  - `None` — operation was not attempted.
  - `Some(Ok(value))` — attempted and succeeded.
  - `Some(Err(e))` — attempted and failed.
  - Use case: optional form fields, conditional processing — the operation is only run if input is
    present.

- Key difference: `Result<Option<T>, E>` means the operation **must run** (it can fail or find
  nothing); `Option<Result<T, E>>` means the operation **may not run** at all.

- **`.transpose()`** converts between the two forms: `Option<Result<T, E>>` becomes
  `Result<Option<T>, E>` and vice versa. Useful when you have one form but need the other.
*/

// --- Result<Option<T>, E> example: database product lookup ---

struct Product {
    id: u32,
    name: String,
}

#[derive(Debug)]
enum DbError {
    ConnectionFailed,
}

fn find_product(id: u32) -> Result<Option<Product>, DbError> {
    // Simulate occasional connection failure
    if id == 999 {
        return Err(DbError::ConnectionFailed);
    }
    // Query succeeds — product may or may not exist
    match id {
        1..=100 => Ok(Some(Product {
            id,
            name: "Laptop".to_string(),
        })),
        _ => Ok(None), // no product found, but query succeeded
    }
}

// --- Option<Result<T, E>> example: optional age field ---

fn parse_optional_age(input: Option<&str>) -> Option<Result<u32, ParseIntError>> {
    // If input is None, the operation is not attempted -> returns None
    // If input is Some, attempt parsing -> returns Some(Ok(...)) or Some(Err(...))
    input.map(|s| s.parse::<u32>())
}

fn layering_result_and_option() {
    // Result<Option<T>, E>: three possible outcomes
    match find_product(42) {
        Ok(Some(p)) => println!("found product: {} (id {})", p.name, p.id),
        Ok(None) => println!("no product found"),
        Err(e) => println!("database error: {e:?}"),
    }

    match find_product(200) {
        Ok(Some(p)) => println!("found: {}", p.name),
        Ok(None) => println!("product 200 not found (expected)"),
        Err(e) => println!("error: {e:?}"),
    }

    match find_product(999) {
        Ok(Some(p)) => println!("found: {}", p.name),
        Ok(None) => println!("not found"),
        Err(e) => println!("connection failed (expected): {e:?}"),
    }

    // Option<Result<T, E>>: three possible outcomes
    match parse_optional_age(Some("25")) {
        None => println!("age not provided"),
        Some(Ok(age)) => println!("valid age: {age}"),
        Some(Err(e)) => println!("invalid age: {e}"),
    }

    match parse_optional_age(Some("abc")) {
        None => println!("age not provided"),
        Some(Ok(age)) => println!("valid age: {age}"),
        Some(Err(e)) => println!("invalid age input (expected): {e}"),
    }

    match parse_optional_age(None) {
        None => println!("age not provided (expected)"),
        Some(Ok(age)) => println!("age: {age}"),
        Some(Err(e)) => println!("error: {e}"),
    }

    // transpose: convert between the two forms
    let opt_result: Option<Result<i32, ParseIntError>> = Some("42".parse::<i32>());
    let result_opt: Result<Option<i32>, ParseIntError> = opt_result.transpose();
    println!("transpose Some(Ok(42)): {result_opt:?}"); // Ok(Some(42))

    let opt_result: Option<Result<i32, ParseIntError>> = None;
    let result_opt: Result<Option<i32>, ParseIntError> = opt_result.transpose();
    println!("transpose None: {result_opt:?}"); // Ok(None)

    println!("layering_result_and_option section executed");
}

// =================================================================================================
// Section 7: thiserror
// =================================================================================================

/*
## thiserror

- The `thiserror` crate provides derive macros that automate the manual boilerplate shown in section
  4 (Display, Error, From impls).
- **`#[derive(thiserror::Error)]`** on an enum generates the `std::error::Error` implementation.
- **`#[error("...")]`** on each variant generates the `Display` implementation. Supports
  format-string interpolation with field names (e.g., `#[error("I/O error: {0}")]` for tuple
  variants, `#[error("{msg}")]` for named fields).
- **`#[from]`** on a variant's field generates the `From` impl for that error type, enabling
  automatic conversion via `?`.
- thiserror is designed for **library code** — when callers need to match on specific error variants
  and make decisions based on the error kind.
- It does not change the error type — the enum remains a concrete, matchable type. It only removes
  the manual trait implementations.
- Comparison with section 4: same behavior, significantly less code. The derive macro generates
  exactly what you would write by hand.
*/

fn thiserror_crate() {
    // Scoped import to avoid shadowing std::error::Error
    use thiserror::Error;

    #[derive(Debug, Error)]
    enum FileError {
        #[error("I/O error: {0}")]
        Io(#[from] std::io::Error),

        #[error("parse error: {0}")]
        Parse(#[from] ParseIntError),

        // Named fields with format interpolation
        #[error("value {value} is out of range {min}..={max}")]
        OutOfRange { value: i32, min: i32, max: i32 },
    }

    // Same function as section 4, but using thiserror — no manual impls needed
    fn read_and_parse_thiserror(path: &str) -> Result<i32, FileError> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let number = contents.trim().parse::<i32>()?;

        if !(0..=1000).contains(&number) {
            return Err(FileError::OutOfRange {
                value: number,
                min: 0,
                max: 1000,
            });
        }
        Ok(number)
    }

    // I/O error: file not found
    match read_and_parse_thiserror("nonexistent.txt") {
        Ok(n) => println!("parsed: {n}"),
        Err(e) => {
            println!("thiserror Display: {e}");

            // Callers can still match on specific variants
            match e {
                FileError::Io(_) => println!("  -> I/O failure"),
                FileError::Parse(_) => println!("  -> parse failure"),
                FileError::OutOfRange { value, min, max } => {
                    println!("  -> {value} not in {min}..={max}");
                }
            }
        }
    }

    println!("thiserror_crate section executed");
}

// =================================================================================================
// Section 8: anyhow
// =================================================================================================

/*
## anyhow

- The `anyhow` crate provides `anyhow::Error`, a **type-erased** error wrapper that can hold any
  error implementing `std::error::Error`.
- **`anyhow::Result<T>`** is shorthand for `Result<T, anyhow::Error>`. Any error type implementing
  `std::error::Error` converts to `anyhow::Error` automatically via `?` — no custom enum or `From`
  impls needed.
- **`.context("msg")`** and **`.with_context(|| format!(...))`** wrap errors with additional
  context, creating an error chain. Each layer of context is preserved for debugging.
- anyhow is designed for **application code** — when you need to propagate and report errors but do
  not need the caller to match on specific error variants.
- **When to use anyhow vs thiserror:**
  - **thiserror** for libraries: callers need typed, matchable errors.
  - **anyhow** for applications: collect, annotate, and display errors without defining enum
    variants for every possible failure.
  - They compose well: a library defines errors with thiserror, and the application wraps them with
    anyhow for reporting.
- Printing with `{}` shows the outermost context message. Printing with `{:?}` shows the full error
  chain including all underlying causes.
*/

fn anyhow_crate() {
    // Scoped import to avoid shadowing std::result::Result
    use anyhow::Context;

    fn read_and_parse_anyhow(path: &str) -> anyhow::Result<i32> {
        let mut file = File::open(path).context("failed to open config file")?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context("failed to read file contents")?;

        let number = contents
            .trim()
            .parse::<i32>()
            .with_context(|| format!("failed to parse '{}' as integer", contents.trim()))?;

        Ok(number)
    }

    // The error includes context messages for debugging
    match read_and_parse_anyhow("nonexistent.txt") {
        Ok(n) => println!("parsed: {n}"),
        Err(e) => {
            // Display: outermost context message
            println!("anyhow Display: {e}");
            // Debug: full error chain with causes
            println!("anyhow Debug: {e:?}");
        }
    }

    // --- bail! and ensure! convenience macros ---
    // bail!("msg") is shorthand for return Err(anyhow!("msg"))
    // ensure!(cond, "msg") is shorthand for if !cond { bail!("msg") }
    fn validate_age(age: i32) -> anyhow::Result<()> {
        anyhow::ensure!(age >= 0, "age cannot be negative: {age}");
        anyhow::ensure!(age <= 150, "age unrealistic: {age}");
        Ok(())
    }
    println!("validate_age(25): {:?}", validate_age(25));
    println!("validate_age(-5): {:?}", validate_age(-5));

    // Summary comparison:
    // - Manual (section 4): custom enum + Display + Error + From impls
    // - thiserror (section 7): custom enum + derive macros, callers can match
    // - anyhow (this section): no custom enum, context adds debug info, callers cannot match

    println!("anyhow_crate section executed");
}

pub fn run() {
    error_handling_philosophy();
    error_propagation();
    multiple_error_types();
    custom_error_types();
    box_dyn_error();
    method_chaining_with_question_mark();
    layering_result_and_option();
    thiserror_crate();
    anyhow_crate();
}
