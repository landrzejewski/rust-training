#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    /// Fallible constructor — returns Err if dimensions are not positive.
    /// Used to demonstrate Result-returning tests.
    pub fn try_new(width: f64, height: f64) -> Result<Self, String> {
        if width <= 0.0 || height <= 0.0 {
            Err(format!(
                "dimensions must be positive, got {}x{}",
                width, height
            ))
        } else {
            Ok(Rectangle { width, height })
        }
    }

    /// Constructor that panics on invalid input.
    /// Used to demonstrate #[should_panic] tests.
    pub fn from_square(side: f64) -> Self {
        if side <= 0.0 {
            panic!("side length must be positive, got {}", side);
        }
        Rectangle {
            width: side,
            height: side,
        }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }

    pub fn can_contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn scale(&self, factor: f64) -> Rectangle {
        Rectangle {
            width: self.width * factor,
            height: self.height * factor,
        }
    }
}

/// Private helper function — demonstrates that unit tests can
/// access private items via `use super::*`.
fn classify_area(rect: &Rectangle) -> &'static str {
    // as u64 truncates toward zero (e.g., 99.9 → 99)
    match rect.area() as u64 {
        0..=99 => "small",
        100..=999 => "medium",
        _ => "large",
    }
}

// =================================================================================================
// Section 1: Unit Testing Fundamentals
// =================================================================================================

/*
## Unit Testing Fundamentals

- A **unit test** is a function annotated with the `#[test]` attribute. When `cargo test` runs, the
  test harness calls each `#[test]` function. If the function **panics**, the test fails; if it
  returns normally, the test passes.
- Tests are conventionally placed inside a module named `tests` at the bottom of the file they test,
  annotated with `#[cfg(test)]`. The `#[cfg(test)]` attribute ensures the module is **only compiled
  during testing** — it is stripped from normal builds. (Conditional compilation with `#[cfg(...)]`
  is covered in module 005.)
- Inside the test module, `use super::*;` imports everything from the parent module, including
  **private** items. This is a key advantage of unit tests — they can verify internal implementation
  details that are not part of the public API.
- The standard structure for a test function follows the **Arrange–Act–Assert** pattern:
  1. **Arrange** — set up the required data and initial conditions.
  2. **Act** — call the function or method under test.
  3. **Assert** — verify the result matches the expectation using assertion macros.
- `cargo test` runs all tests (unit, integration, and doc tests). `cargo test --lib` runs only unit
  tests in the library crate. Use `cargo test --bin <name>` for binary crate tests.
- **TDD with `todo!()`**: write the test first, then stub the implementation with `todo!()`. The
  test compiles but panics when run. Replace `todo!()` with real code to make the test pass. See
  module 005 section 4 for `todo!()` details.

### Test Helpers (Fixtures)

- Rust has no built-in `setUp` / `tearDown` mechanism. Instead, write plain helper functions that
  construct test data.
- Each test calls the helper explicitly. This keeps tests self-contained and easy to read.
*/

fn unit_testing_fundamentals() {
    // Demonstrate the production code that the tests verify
    let rect = Rectangle::new(4.0, 5.0);
    println!("Rectangle: {:?}", rect);
    println!("  area: {}", rect.area());
    println!("  perimeter: {}", rect.perimeter());

    // Private function — not accessible from outside this module,
    // but unit tests can reach it via use super::*
    println!("  classification: {}", classify_area(&rect));

    // The Arrange-Act-Assert pattern:
    // Arrange: let rect = Rectangle::new(4.0, 5.0);
    // Act:     let area = rect.area();
    // Assert:  assert_eq!(area, 20.0);

    println!("unit_testing_fundamentals section executed");
}

// =================================================================================================
// Section 2: Assertion Macros
// =================================================================================================

/*
## Assertion Macros

- **`assert!(expression)`** — panics if `expression` evaluates to `false`. Used for boolean
  conditions.
- **`assert_eq!(left, right)`** — panics if `left != right`. On failure, it prints both values using
  `Debug` formatting so you can see what went wrong. Requires both operands to implement `PartialEq`
  and `Debug`.
- **`assert_ne!(left, right)`** — panics if `left == right`. Same trait requirements as
  `assert_eq!`.
- All three macros accept an **optional trailing format string** for custom failure messages:
  `assert_eq!(a, b, "expected {} but got {}", b, a)`. Custom messages help identify which assertion
  failed in tests with multiple checks.
- On failure, `assert_eq!` and `assert_ne!` display the values labeled as "left" and "right". This
  is why `#[derive(Debug)]` is essential for custom types used in assertions — without it, the
  compiler cannot format the failure output.
- **Order of `left` and `right` does not matter**. In some frameworks (JUnit, pytest, Jest),
  equality assertions take arguments named `expected` and `actual`, and the convention dictates
  which goes first. Rust deliberately avoids that framing — the arguments are just `left` and
  `right`, and `assert_eq!(4, result)` produces the same failure output as `assert_eq!(result, 4)`.
  Pick whichever reads more naturally at the call site; do not spend time enforcing a convention.
- **Floating-point comparison**: avoid `assert_eq!` with floats due to precision issues (see module
  003). Instead, use epsilon-based comparison: `assert!((actual - expected).abs() < f64::EPSILON)`.
  For results with accumulated rounding error, use a small multiple: `assert!((actual -
  expected).abs() < f64::EPSILON * 10.0)`. Note: `f64::EPSILON` is the spacing *at 1.0*, so
  absolute-epsilon comparison only works when the expected value is near 1.0. For larger magnitudes,
  scale the tolerance to the expected value — e.g., `(actual - expected).abs() < expected.abs() *
  1e-10`.
- **`debug_assert!`**, **`debug_assert_eq!`**, and **`debug_assert_ne!`** behave identically but are
  **only enabled in debug builds**. In release builds (`cargo build --release`), they are compiled
  out entirely. Useful for expensive invariant checks that should not affect production performance.
*/

fn assertion_macros() {
    let big = Rectangle::new(10.0, 10.0);
    let small = Rectangle::new(3.0, 3.0);

    // assert! — checks a boolean condition
    println!("big can contain small: {}", big.can_contain(&small));
    assert!(big.can_contain(&small));

    // assert_eq! — checks equality, prints both values on failure
    let rect = Rectangle::new(6.0, 3.0);
    println!("area of 6x3: {}", rect.area());
    assert_eq!(rect.area(), 18.0);

    // assert_ne! — checks inequality
    assert_ne!(rect.area(), 0.0);

    // assert_eq! with structs — compares Rectangle instances directly
    // (requires #[derive(PartialEq, Debug)] on Rectangle)
    let a = Rectangle::new(5.0, 5.0);
    let b = Rectangle::from_square(5.0);
    println!("new(5,5) == from_square(5): {}", a == b);
    assert_eq!(a, b);

    // Custom failure message example (would panic if assertion failed):
    // assert_eq!(rect.area(), 99.0, "expected area 99.0, got {}", rect.area());

    // debug_assert! — only checked in debug builds, compiled out in release
    debug_assert!(rect.area() > 0.0);

    println!("assertion_macros section executed");
}

// =================================================================================================
// Section 3: Testing Results and Panics
// =================================================================================================

/*
## Testing Results and Panics

### Result-Returning Tests

- Test functions can return `Result<(), E>` where `E` implements `Debug`. This allows using the `?`
  operator to propagate errors concisely instead of manually unwrapping.
- The test **passes** if it returns `Ok(())` and **fails** if it returns `Err(...)`. The error value
  is printed using `Debug` formatting.
- This is particularly useful when calling fallible functions — you can chain multiple `?` calls
  without explicit unwrapping.
- **Asserting an expected `Err` inside a `Result`-returning test**. The `?` operator propagates
  `Err` as test failure, which is the opposite of what you want when a function is **supposed** to
  return `Err`. In that case, use `assert!(result.is_err())` instead of `?`. For more specific
  checks, match on the error value or call `assert_eq!(result.unwrap_err(), expected_err)`. This is
  the positive counterpart to the `#[should_panic]` limitation below: `should_panic` handles
  expected panics, `assert!(...is_err())` handles expected `Err` returns, and both approaches remain
  compatible with a `Result<(), E>` test signature.

### Testing Panics with `#[should_panic]`

- `#[should_panic]` placed below `#[test]` marks a test that is expected to panic. The test
  **passes** only if the function panics, and **fails** if it returns normally.
- `#[should_panic(expected = "substring")]` adds a check on the panic message — the test passes only
  if the panic message **contains** the given substring. This prevents false positives from
  unrelated panics.
- **Caution**: the `expected` substring matches panics from **any** code in the test — not just the
  function under test. An `.unwrap()` in setup code could accidentally produce a matching panic.
  Keep `#[should_panic]` tests minimal and focused on a single operation.
- **Limitation**: `#[should_panic]` cannot be combined with a `Result<(), E>` return type. The
  compiler will reject it because the two mechanisms are incompatible — one expects a panic, the
  other expects a return value.

### Choosing Between the Two

- Use **`Result<(), E>`** when testing fallible operations where errors are returned as values
  (functions returning `Result`). Clean, composable with `?`.
- Use **`#[should_panic]`** when verifying that a function correctly panics under invalid
  conditions. Always prefer the `expected = "..."` form to avoid false positives.
*/

fn testing_results_and_panics() {
    // try_new returns Result — Ok for valid dimensions, Err for invalid
    let valid = Rectangle::try_new(5.0, 3.0);
    println!("try_new(5.0, 3.0): {:?}", valid);

    let invalid = Rectangle::try_new(-1.0, 5.0);
    println!("try_new(-1.0, 5.0): {:?}", invalid);

    // from_square works for valid input
    let square = Rectangle::from_square(4.0);
    println!("from_square(4.0): {:?}", square);
    // from_square(-1.0) would panic — verified by #[should_panic] tests

    println!("testing_results_and_panics section executed");
}

// =================================================================================================
// Section 4: Controlling Test Execution
// =================================================================================================

/*
## Controlling Test Execution

### The `--` Separator

- `cargo test` command line splits into two parts separated by `--`. Arguments **before** `--` are
  consumed by `cargo test` itself (e.g., `--lib`, `--doc`, `--test <file>`, test name filter).
  Arguments **after** `--` are forwarded to the compiled test binary (e.g., `--show-output`,
  `--ignored`, `--test-threads=1`, `--include-ignored`).
- This is why examples like `cargo test -- --show-output` contain two dashes: the first `--` is the
  separator, the second `--` is the start of the flag name for the test binary.
- Discover flags on both sides: `cargo test --help` lists options before the separator, `cargo test
  -- --help` lists options after it.

### Parallel Execution (Default)

- `cargo test` runs all tests **in parallel** using threads. This keeps test suites fast and gives
  feedback sooner, but it means every test must be independent — no two tests should assume they are
  the only code touching a given resource.
- "Shared state" here covers more than global variables: the **current working directory**,
  **environment variables**, a **scratch file** on disk, a **network port**, a **database table** —
  anything the process as a whole can mutate. Two tests that both write to `/tmp/test.txt` will
  race; two tests that both bind port 8080 will collide; two tests that both call
  `std::env::set_var(...)` see each other's mutations.
- The standard fixes: make each test use a **unique path** (e.g., `tempfile::NamedTempFile`), pick
  an OS-assigned **ephemeral port** (`:0`), use a separate in-memory fixture per test, or drop back
  to serial execution with `--test-threads=1` (see "Other Useful Flags" below) when none of the
  above is practical.

### Test Output

- By default, `cargo test` **captures** `println!` output from passing tests. Output is only
  displayed for failing tests.
- Use `cargo test -- --show-output` to see output from all tests, including those that pass.

### Filtering Tests

- `cargo test name_pattern` runs only tests whose fully qualified name contains `name_pattern`. For
  example, `cargo test area` runs all tests with "area" in the name.
- A single test can be run by specifying its exact name: `cargo test
  rectangle_area_is_width_times_height`.

### The `#[ignore]` Attribute

- `#[ignore]` marks a test to be **skipped** during normal `cargo test` runs. The test appears in
  output as "ignored".
- Useful for tests that are slow, resource-intensive, or depend on external services.
- Run ignored tests explicitly:
  - `cargo test -- --ignored` — runs **only** ignored tests.
  - `cargo test -- --include-ignored` — runs **all** tests including ignored ones.

### Other Useful Flags

- `cargo test --lib` — unit tests only (no integration or doc tests).
- `cargo test --doc` — doc tests only.
- `cargo test --test file_name` — a specific integration test file.
- `cargo test -- --test-threads=1` — run tests serially instead of in parallel. Necessary when tests
  share external state (files, databases, network ports).
- **When to serialize**: tests that read/write the same file, bind to the same network port, or
  depend on shared mutable state. When all tests use independent data, parallel execution is safe
  and faster.
- For selective serialization without slowing all tests, the `serial_test` crate provides a
  `#[serial]` attribute.
- **Mocking**: Rust has no built-in mocking. Popular crates: `mockall` (trait-based mocking),
  `mockito` (HTTP mocking).
- **Code coverage**: use `cargo llvm-cov` (from `cargo-llvm-cov`) or `cargo tarpaulin` to measure
  test coverage.
*/

fn controlling_test_execution() {
    println!("Cargo test command cheat sheet:");
    println!("  cargo test                         run all tests");
    println!("  cargo test --lib                   unit tests only");
    println!("  cargo test --doc                   doc tests only");
    println!("  cargo test --test <file>           specific integration test");
    println!("  cargo test <pattern>               filter by name pattern");
    println!("  cargo test -- --show-output        show println! from passing tests");
    println!("  cargo test -- --ignored            run only ignored tests");
    println!("  cargo test -- --include-ignored    run all including ignored");
    println!("  cargo test -- --test-threads=1     run tests serially");

    println!("controlling_test_execution section executed");
}

// =================================================================================================
// Section 5: Integration Tests
// =================================================================================================

/*
## Integration Tests

Integration tests verify that different parts of a library work together correctly. They live
**outside** the source code and can only access the **public API**.

### Location and Compilation

- Integration tests are placed in a top-level `tests/` directory, alongside `src/`.
- Each `.rs` file in `tests/` is compiled as an **independent crate**. There is no need for
  `#[cfg(test)]` — the entire file is test code.
- Tests import from the library crate by name: `use my_crate::SomeType;`.
- **Crate naming**: the crate name comes from the `name` field in `Cargo.toml`, with hyphens (`-`)
  converted to underscores (`_`). For example, a package named `playground-rust` is imported as `use
  playground_rust::...;`.

### Example File Structure

```text
project/
  src/
    lib.rs              <-- library crate root
    lib             <-- binary crate (optional)
  tests/
    order_tests.rs      <-- integration test (separate crate)
    payment_tests.rs    <-- integration test (separate crate)
    helpers/
      mod.rs            <-- shared helper code (NOT a test crate)
```

### Example Integration Test File (`tests/order_tests.rs`)

```rust
use my_crate::{Product, Order};

mod helpers;

#[test]
fn order_total_is_price_times_quantity() {
    helpers::setup();
    let product = Product::new("Book", 19.99);
    let order = Order::new(product, 3);
    assert_eq!(format!("{:.2}", order.total()), "59.97");
}
```

### Helper Modules

- Top-level `.rs` files in `tests/` are automatically treated as test crates, which means a helper
  file like `tests/helpers.rs` would be compiled as a test file (with its own test report).
- To avoid this, place helper code in a **subdirectory** with `mod.rs`: `tests/helpers/mod.rs`.
  Cargo does not treat subdirectory modules as test crates.
- Test files import the helper with `mod helpers;`.

### Running Integration Tests

- `cargo test` runs all tests, including integration tests.
- `cargo test --test order_tests` runs only the tests in `tests/order_tests.rs`.
- Cargo displays separate sections in the test report for unit tests and each integration test file.

### Binary Crate Limitation

- Integration tests **cannot** directly import from a binary crate (`src/lib`). The binary's
  `main` function is an entry point, not a library API.
- The common practice is to keep the core logic in a **library crate** (`src/lib.rs`) and have the
  binary be a thin wrapper that calls into the library. This way, integration tests can test the
  library's public API.
*/

// =================================================================================================
// Public entry point
// =================================================================================================

pub fn run() {
    unit_testing_fundamentals();
    assertion_macros();
    testing_results_and_panics();
    controlling_test_execution();
}

// =================================================================================================
// Tests
// =================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Section 1: Unit Testing Fundamentals ---

    #[test]
    fn rectangle_area_is_width_times_height() {
        // Arrange
        let rect = Rectangle::new(4.0, 5.0);
        // Act
        let area = rect.area();
        // Assert
        assert_eq!(area, 20.0);
    }

    #[test]
    fn rectangle_perimeter_calculation() {
        let rect = Rectangle::new(3.0, 7.0);
        assert_eq!(rect.perimeter(), 20.0);
    }

    #[test]
    fn private_function_accessible_from_tests() {
        // Unit tests can access private functions via use super::*
        let small = Rectangle::new(5.0, 5.0);
        assert_eq!(classify_area(&small), "small");

        let medium = Rectangle::new(10.0, 20.0);
        assert_eq!(classify_area(&medium), "medium");

        let large = Rectangle::new(100.0, 100.0);
        assert_eq!(classify_area(&large), "large");
    }

    // --- Section 2: Assertion Macros ---

    #[test]
    fn assert_boolean_condition() {
        let big = Rectangle::new(10.0, 10.0);
        let small = Rectangle::new(3.0, 3.0);

        // assert! checks that the expression is true
        assert!(big.can_contain(&small));
        assert!(!small.can_contain(&big));
    }

    #[test]
    fn assert_eq_compares_values() {
        let rect = Rectangle::new(6.0, 3.0);
        // On failure, prints both "left" and "right" values
        assert_eq!(rect.area(), 18.0);
    }

    #[test]
    fn assert_ne_checks_inequality() {
        let rect = Rectangle::new(6.0, 3.0);
        assert_ne!(rect.area(), 0.0);
    }

    #[test]
    fn assert_eq_with_structs() {
        // Comparing Rectangle instances directly requires PartialEq + Debug
        let a = Rectangle::new(5.0, 5.0);
        let b = Rectangle::from_square(5.0);
        assert_eq!(a, b);
    }

    #[test]
    fn custom_failure_message() {
        let rect = Rectangle::new(4.0, 5.0);
        let expected = 20.0;
        // Optional format string after the assertion arguments
        assert_eq!(
            rect.area(),
            expected,
            "Rectangle {}x{} should have area {}, got {}",
            rect.width,
            rect.height,
            expected,
            rect.area()
        );
    }

    // --- Section 3: Testing Results and Panics ---

    #[test]
    fn try_new_with_valid_dimensions() -> Result<(), String> {
        // Returning Result<(), E> allows using the ? operator
        let rect = Rectangle::try_new(5.0, 3.0)?;
        assert_eq!(rect.area(), 15.0);
        Ok(())
    }

    #[test]
    fn try_new_returns_error_for_invalid_dimensions() {
        let result = Rectangle::try_new(-1.0, 5.0);
        assert!(result.is_err());
        // Verify the error message content
        assert!(result.unwrap_err().contains("dimensions must be positive"));
    }

    #[test]
    #[should_panic]
    fn from_square_panics_on_negative_side() {
        // Test passes only if this function panics
        Rectangle::from_square(-5.0);
    }

    #[test]
    #[should_panic(expected = "side length must be positive")]
    fn from_square_panics_with_expected_message() {
        // Test passes only if the panic message contains the expected substring
        Rectangle::from_square(-3.0);
    }

    // --- Section 4: Controlling Test Execution ---

    #[test]
    fn scale_produces_proportional_rectangle() {
        let rect = Rectangle::new(3.0, 4.0);
        let scaled = rect.scale(2.0);
        assert_eq!(scaled, Rectangle::new(6.0, 8.0));
        // Area scales by factor squared
        assert_eq!(scaled.area(), rect.area() * 4.0);
    }

    #[test]
    fn output_captured_by_default() {
        // This println! is captured by the test harness and only shown
        // if this test fails or if --show-output is used
        println!("this output is captured unless --show-output is passed");
        let rect = Rectangle::new(2.0, 3.0);
        assert_eq!(rect.area(), 6.0);
    }

    #[test]
    #[ignore]
    fn slow_exhaustive_check() {
        // Ignored by default — run with: cargo test -- --ignored
        for w in 1..=100 {
            for h in 1..=100 {
                let rect = Rectangle::new(w as f64, h as f64);
                assert_eq!(rect.area(), (w * h) as f64);
            }
        }
    }

    // --- Floating-point comparison ---

    #[test]
    fn float_comparison_with_epsilon() {
        let rect = Rectangle::new(1.0, 1.0);
        let scaled = rect.scale(std::f64::consts::SQRT_2);
        // Don't use assert_eq! for floats — use epsilon comparison
        assert!(
            (scaled.area() - 2.0).abs() < f64::EPSILON * 10.0,
            "expected area ~2.0, got {}",
            scaled.area()
        );
    }

    // --- Test fixtures / setup pattern ---

    /// Shared test setup — Rust uses plain helper functions instead
    /// of JUnit-style setUp/tearDown.
    fn setup_standard_rect() -> Rectangle {
        Rectangle::new(10.0, 5.0)
    }

    #[test]
    fn fixture_area() {
        let rect = setup_standard_rect();
        assert_eq!(rect.area(), 50.0);
    }

    #[test]
    fn fixture_perimeter() {
        let rect = setup_standard_rect();
        assert_eq!(rect.perimeter(), 30.0);
    }
}
