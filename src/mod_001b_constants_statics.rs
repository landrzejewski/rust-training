use std::sync::LazyLock;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicI32, Ordering};

// =================================================================================================
// Section 6: Constants
// =================================================================================================

/*
## Constants

- Declared using the `const` keyword.
- Require an **explicit type annotation** — the type is never inferred.
- The value must be a **constant expression**, computable at compile
time.
- Constants have **no guaranteed fixed memory address**. The compiler
may inline them at each usage site or place them in read-only memory — you cannot rely on a `const`
having a single address. This is the key difference from `static`, which is guaranteed to have
exactly one address.
- Constants cannot be `mut` — they are always immutable.
- Constants can be declared at any scope level: module-level, inside
a function body, or as associated constants inside `impl` blocks (trait-associated constants are
covered with traits in a later module).
- A `const fn` is a function that **can also** be evaluated at
compile time — it is still callable normally at runtime, but it gains the extra power of being
usable wherever a constant expression is required (e.g., as the value of a `const` or `static`).
- **`const fn` limitations**: `const fn` cannot perform heap
allocation (no `String`, `Vec`, `Box::new`), call non-const functions (no `.to_string()`,
`.to_uppercase()`, etc.), or use `dyn Trait`. Only a subset of operations is available at compile
time. However, `if`/`else`, `match`, `while`, and `loop` are all supported in const fn. Note: `for`
loops are **not** currently supported in `const fn` on stable Rust, because `Iterator::next` is not
a const method — use `while` with a manual counter instead (as in `factorial` below).
- Naming convention: `SCREAMING_SNAKE_CASE`.
*/

// Module-level constant — visible throughout this module
const MAX_CONNECTIONS: u32 = 100;

fn constants() {
    // Function-local constant
    const TIMEOUT_SECONDS: u64 = 3600;

    println!("max connections: {MAX_CONNECTIONS}");
    println!("timeout: {TIMEOUT_SECONDS} seconds");

    // Constants can be used in constant expressions to define other constants
    const TIMEOUT_MILLIS: u64 = TIMEOUT_SECONDS * 1000;
    println!("timeout: {TIMEOUT_MILLIS} milliseconds");

    // const fn — a function evaluated at compile time, usable in
    // constant expressions
    const fn square(x: u32) -> u32 {
        x * x
    }
    const SQUARED: u32 = square(12);
    println!("const fn square(12) = {SQUARED}");

    // const fn with conditional logic — computed entirely at compile time
    const fn clamp(value: i32, min: i32, max: i32) -> i32 {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }
    const CLAMPED: i32 = clamp(150, 0, 100);
    println!("const fn clamp(150, 0, 100) = {CLAMPED}");

    // const fn with loops — fully supported in modern Rust
    const fn factorial(n: u64) -> u64 {
        let mut result = 1;
        let mut i = 2;
        while i <= n {
            result *= i;
            i += 1;
        }
        result
    }
    const FACT_10: u64 = factorial(10);
    println!("const fn factorial(10) = {FACT_10}");

    // const fn cannot use heap allocation, trait methods, or dyn:
    // const fn bad() -> String {
    //     String::from("hello") // ERROR: cannot call non-const fn `String::from`
    // }
    // const fn also_bad(s: &str) -> String {
    //     s.to_uppercase() // ERROR: cannot call non-const fn `to_uppercase`
    // }
}

// =================================================================================================
// Section 7: Static Variables
// =================================================================================================

/*
## Static Variables

- Declared using the `static` keyword.
- Have a **fixed memory address** and a `'static` lifetime — they
live for the entire duration of the program.
- Require an **explicit type annotation**.
- The initial value must be a constant expression computable at
compile time (for plain `static`; see Lazy Initialization below for runtime values).
- `static mut` variables can be declared, but accessing them requires
an `unsafe` block. The fundamental issue: Rust's `&mut T` guarantees **exclusive access**, but any
code anywhere in the program can reach a static — so the compiler cannot enforce exclusivity, and
data races become possible. In practice, prefer atomic types or synchronization primitives over
`static mut`.
- Naming convention: `SCREAMING_SNAKE_CASE`.

### Constants vs. Statics

- `const`: no guaranteed fixed memory address, purely a compile-time
value — the compiler may inline it at each usage site.
- `static`: has a single fixed address in memory, lives for the
entire program, and because any thread can reach it, plain `static` **requires** the type to be
`Sync`.
- Use `const` when you just need a compile-time value. Use `static`
when you need a stable memory address, or when you want to hold a value with interior mutability
(`AtomicI32`, `Mutex<T>`, ...) for the program's entire lifetime.

### Atomic Types (Preview)

- Atomic types like `AtomicI32` provide thread-safe mutation of
statics without `unsafe`. They use hardware-level atomic instructions instead of locks.
- Every atomic operation requires a **memory ordering** parameter
(`Ordering::Relaxed`, `SeqCst`, etc.) that controls how the operation is visible to other threads.
`Relaxed` is the weakest ordering — it guarantees atomicity but not ordering relative to other
operations. Atomic orderings are covered in depth in the concurrency module.

### Lazy Initialization

Plain `static` values must be initialized with compile-time constant expressions. When you need a
static value that is computed at **runtime**, Rust provides two standard library types:

**Note:** This section uses closures and concepts from `std::sync` not yet introduced — treat it as
a preview.

**`std::sync::LazyLock<T>`**
- Wraps a value that is computed lazily on first access using a
provided closure.
- The closure runs **exactly once**; all subsequent accesses return
the already-computed value.
- Thread-safe: if multiple threads access the value simultaneously,
only one will run the closure; the others will block until it completes.
- Use `LazyLock` when you know the initialization logic at the
declaration site.

**`std::sync::OnceLock<T>`**
- A cell that can be written to **exactly once**.
- Starts empty (`OnceLock::new()`) and is initialized later via
`set()` or `get_or_init()`.
- Thread-safe: concurrent calls to `set()` are safe; only the first
one succeeds.
- Use `OnceLock` when initialization is **deferred** — the value or
the initialization logic is not available at the declaration site (e.g., configuration loaded from a
file or command-line arguments).

### When to choose `LazyLock` vs `OnceLock`

- **`LazyLock`**: the initialization closure is known at the
declaration site. Think of it as "self-initializing static". Simpler API — just access it and it
initializes itself.
- **`OnceLock`**: the initialization value or logic depends on
runtime context (e.g., CLI args, config files, environment variables). You initialize it explicitly
with `set()` or `get_or_init()` at the appropriate point in your program.
*/

// Plain static — compile-time value with a fixed memory address
static APP_NAME: &str = "Playground Rust";

// LazyLock — the closure runs on first access, computing a runtime value
static COMPUTED_VALUE: LazyLock<String> = LazyLock::new(|| {
    let base = 6;
    let result = base * 7;
    format!("The answer is {result}")
});

// OnceLock — starts empty, initialized later at runtime
static CONFIG_VALUE: OnceLock<String> = OnceLock::new();

fn static_variables() {
    // --- Plain static ---
    println!("application name: {APP_NAME}");

    // --- Thread-safe mutation via AtomicI32 (no `unsafe` needed) ---
    static CALL_COUNT: AtomicI32 = AtomicI32::new(0);
    let count = CALL_COUNT.fetch_add(1, Ordering::Relaxed);
    println!("static_variables() called {} time(s)", count + 1);

    // --- LazyLock ---
    // First access triggers the closure; the value is computed and cached
    println!("LazyLock value: {}", *COMPUTED_VALUE);
    // Second access returns the cached value (the closure does not run again)
    println!("LazyLock value (cached): {}", *COMPUTED_VALUE);

    // --- OnceLock ---
    // Before initialization, get() returns None
    println!("before set: {:?}", CONFIG_VALUE.get()); // None

    // Initialize with set() — only the first call succeeds
    let _ = CONFIG_VALUE.set("production".to_string());
    // A second set() does not overwrite; it returns Err with the rejected value.
    let second_set = CONFIG_VALUE.set("staging".to_string());
    println!("second set attempt succeeded: {}", second_set.is_ok()); // false

    // Read the stored value
    println!("config: {}", CONFIG_VALUE.get().unwrap());

    // get_or_init() — returns the existing value or initializes it with
    // the provided closure if not yet set. Realistic use: loading config
    // from the environment at first access.
    static APP_PORT: OnceLock<u16> = OnceLock::new();
    let port = APP_PORT.get_or_init(|| {
        // In real code: std::env::var("PORT").unwrap_or("8080").parse().unwrap()
        8080
    });
    println!("app port (get_or_init): {port}");

    // --- static mut (discouraged — shown for completeness) ---
    // `unsafe` is an advanced topic covered in a dedicated module;
    // this is a brief preview.
    // Accessing a mutable static requires an unsafe block because the
    // compiler cannot verify thread safety. Since Rust 2024,
    // creating references to `static mut` is denied — use raw
    // pointers via std::ptr::addr_of_mut! instead.
    // Prefer AtomicI32 or other synchronization primitives (shown above).
    static mut UNSAFE_COUNTER: i32 = 0;
    unsafe {
        let ptr = std::ptr::addr_of_mut!(UNSAFE_COUNTER);
        *ptr += 1;
        println!("static mut counter: {}", *ptr);
    }
    // WARNING: in multi-threaded code, `static mut` causes undefined
    // behavior without external synchronization. Always prefer atomics.
}

pub fn run() {
    constants();
    static_variables();
}
