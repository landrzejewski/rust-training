// =================================================================================================
// Section 1: print! and println!
// =================================================================================================

/*
## `print!` and `println!`

- `print!` outputs text to standard output **without** a trailing newline.
- `println!` outputs text **with** a trailing newline.
- Both are **macros** (indicated by the `!`). The compiler validates the format string and argument
  types at compile time.
- The format string uses `{}` as a placeholder for values. The value is converted to text using the
  **`Display` trait**.
- Simple types (integers, floats, booleans, strings, characters) implement `Display` and can be
  printed with `{}`.
- For types that do not implement `Display` (arrays, tuples, structs), use `{:?}` (**`Debug`
  formatting**) or `{:#?}` (**pretty Debug**). These require the `Debug` trait.
- Variables can be interpolated directly in the format string: `println!("{variable}")` — no need
  for a separate argument.
*/

fn print_basics() {
    // print! — no newline after output
    print!("hello ");
    print!("world");
    println!(); // just a newline to finish the line

    // println! — appends a newline
    println!("this is on its own line");

    // Printing simple values with {} (Display trait)
    let name = "Rust";
    let version = 2024;
    let stable = true;
    println!(
        "language: {}, edition: {}, stable: {}",
        name, version, stable
    );

    // Inline variable interpolation — variable name directly in {}
    println!("{name} edition {version}");

    // Printing complex types with {:?} (Debug trait)
    let numbers = [1, 2, 3, 4, 5];
    let point = (3.5, -2.1);
    println!("array: {:?}", numbers);
    println!("tuple: {:?}", point);

    // Pretty Debug with {:#?} — adds indentation and newlines
    let matrix = [[1, 2], [3, 4]];
    println!("pretty matrix: {:#?}", matrix);
}

// =================================================================================================
// Section 2: Escape Sequences
// =================================================================================================

/*
## Escape Sequences

- Escape sequences begin with a backslash `\` inside string literals and represent special
  characters.
- Common escape sequences:
  - `\n` — newline
  - `\t` — horizontal tab
  - `\r` — carriage return
  - `\"` — literal double quote
  - `\\` — literal backslash
  - `\0` — null character
  - `\u{XXXX}` — Unicode code point (1–6 hex digits): `\u{1F600}` produces the grinning face emoji.
- **Raw strings** `r"..."` or `r#"..."#` disable all escape processing — backslashes are treated as
  literal characters. Use `r#"..."#` when the string itself contains double quotes.
*/

fn escape_sequences() {
    // \n — newline
    println!("first line\nsecond line");

    // \t — tab
    println!("\tindented with a tab");

    // \" — literal double quote inside a string
    println!("she said \"hello\"");

    // \\ — literal backslash
    println!("file path: C:\\Users\\Rust");

    // Raw string — no escape processing
    println!(r"raw: \n is not a newline here");

    // \r — carriage return (overwrites from the start of the line)
    println!("ABCDEF\rXY"); // output: XYCDEF

    // \0 — null character (rarely needed in everyday code)
    let with_null = "hello\0world";
    println!("string with null: {:?}", with_null);

    // \u{...} — Unicode code point escape
    println!("Unicode escape: \u{1F600}");

    // Raw string with quotes using r#"..."#
    // The # delimiters let you include literal double quotes inside
    println!(r#"JSON: {{"key": "value", "count": 42}}"#);

    // Extra # for strings containing "#:
    // r##"she said: r#"nested"# wow"##

    // Practical raw string use cases — regex patterns and file paths
    // avoid escaping backslashes entirely
    let regex_pattern = r"\d{3}-\d{3}-\d{4}"; // phone number pattern
    println!("regex pattern: {regex_pattern}");
    let windows_path = r"C:\Users\lukas\Documents\file.txt";
    println!("windows path: {windows_path}");
}

// =================================================================================================
// Section 3: Format String Arguments
// =================================================================================================

/*
## Format String Arguments

- **Positional arguments**: `{0}`, `{1}` — reference arguments by index. The same argument can be
  reused multiple times.
- **Named arguments**: `{name}` with `name = value` — improves readability for complex format
  strings.
- Positional, named, and implicit `{}` arguments can be mixed in a single format string.
- **Format specifiers** control how a value is displayed:
  - **Width**: `{:10}` — minimum width, padded with spaces.
  - **Fill and alignment**: `{:*>10}` — fill character, alignment (`<` left, `^` center, `>` right),
    and width.
  - **Precision**: `{:.4}` — decimal places for floats, or max character count for strings.
  - **Combined (zero padding)**: `{:08.2}` — sign-aware zero padding, width 8, 2 decimal places. The
    `0` here is a dedicated **flag** (not a fill character) that places padding zeros *after* any
    sign and before the digits, so `-3.14` formats as `-0003.14`. Using `0` as an explicit fill
    character instead (`{:0>8.2}`) would pad zeros *before* the sign (`000-3.14`), which is usually
    not what you want.
- **Numeric bases**: `{:b}` (binary), `{:o}` (octal), `{:x}` (lowercase hex), `{:X}` (uppercase
  hex). Prefix `#` for `0b`, `0o`, `0x`: `{:#x}` produces `0xff`.
- **Scientific notation**: `{:e}` (lowercase) and `{:E}` (uppercase).
- Precision for strings: `{:.5}` truncates to 5 characters.
- `format!` works like `println!` but returns a `String` instead of printing. Note: `format!`
  **allocates** a new `String` on the heap and returns it, while `println!` writes directly to
  stdout without producing a `String`.
- `eprint!` and `eprintln!` print to **standard error** instead of standard output.
- **Why stderr exists — shell redirection rationale**. CLI tools are expected to write errors to
  stderr and results to stdout so the two streams can be redirected independently. The shell `>`
  operator redirects **only** stdout: running `./tool > out.txt` captures results into the file
  while error messages printed via `eprintln!` still reach the terminal. If a tool printed errors
  via `println!`, redirecting its output would also hide its errors — the user would see an empty
  output file and no clue what went wrong. Rule of thumb for command-line programs: `println!` for
  successful output (results, data, computation), `eprintln!` for diagnostics (errors, warnings,
  progress messages).
- `write!` and `writeln!` use the same format syntax but write to a buffer (such as a `String`)
  instead of stdout. Requires `use std::fmt::Write` for `String` targets.
- **`dbg!`** prints the file name, line number, expression text, and its value to **stderr**. It
  returns ownership of the value, so it can be inserted into expressions: `let x = dbg!(2 + 3);`.
  Useful for quick debugging without writing full `println!` calls. Note: `dbg!` adds overhead and
  should be **removed before shipping** production code — it is a development-only tool.
*/

fn format_arguments() {
    // Positional arguments — reorder or reuse by index
    println!("{0} is {1} and {0} is great", "Rust", "fast");

    // Named arguments — key = value syntax
    println!(
        "{language} is a {kind} programming language",
        kind = "systems",
        language = "Rust"
    );

    // Width — minimum field width (right-aligned for numbers)
    println!("number: [{:10}]", 42);
    // Left-aligned string (default for strings)
    println!("text:   [{:10}]", "hi");

    // Fill and alignment
    println!("left:   [{:<10}]", 42);
    println!("center: [{:^10}]", 42);
    println!("right:  [{:>10}]", 42);
    println!("fill:   [{:*^10}]", 42);

    // Precision — decimal places for floats
    let pi = 3.141592653589793;
    println!("default: {pi}");
    println!("2 decimals: {pi:.2}");
    println!("6 decimals: {pi:.6}");

    // Combined — zero-padded, 8 wide, 2 decimal places
    println!("combined: [{:08.2}]", pi);

    // Numeric base formatting
    println!("binary:  {:08b}", 42);
    println!("octal:   {:o}", 42);
    println!("hex:     {:x}", 255);
    println!("HEX:     {:X}", 255);
    println!("hex 0x:  {:#x}", 255);

    // Sign formatting — always show + or -
    println!("signed:  {:+}", 42);
    println!("signed:  {:+}", -42);

    // Scientific notation — lowercase and uppercase
    println!("sci:     {:e}", 1234.5);
    println!("SCI:     {:E}", 1234.5); // uppercase E
    println!("sci 2dp: {:.2e}", 1234.5); // precision with scientific

    // String precision — truncates to N characters
    println!("truncated: {:.3}", "hello");

    // format! — returns a String instead of printing
    let msg = format!("{} is {}", "Rust", "fast");
    println!("{msg}");

    // eprint! / eprintln! — write to standard error
    eprintln!("this goes to stderr");

    // Zero-padding for integers — {:06} pads with zeros to width 6
    println!("zero-padded: [{:06}]", 42); // [000042]
    println!("zero-padded: [{:06}]", -42); // [-00042]

    // Combined format specifiers — sign + zero-pad + width + precision
    println!("combined: [{:+010.2}]", 3.14159); // [+000003.14]
    println!("combined: [{:+010.2}]", -3.14159); // [-000003.14]
    // Reads as: show sign (+), fill with 0, minimum width 10, 2 decimal places

    // Dynamic width and precision — use $ suffix to read from a variable
    let width = 12;
    let precision = 3;
    println!("dynamic: [{:>width$.precision$}]", 3.14159);
    // Equivalent to [{:>12.3}] → [       3.142]

    // dbg! — prints file, line, expression, and value to stderr.
    // Returns ownership of the value, so it can wrap expressions.
    let x = dbg!(2 + 3); // prints: [src/basic/...rs:LINE] 2 + 3 = 5
    println!("dbg! returned: {x}");

    // dbg! in a method chain — it returns ownership, so you can wrap
    // any sub-expression without disrupting the surrounding code
    let len = dbg!([1, 2, 3]).len();
    println!("dbg! in chain — len: {len}");

    // write! / writeln! — same format syntax but writes to a buffer
    // instead of stdout. Requires `use std::fmt::Write` for String targets.
    use std::fmt::Write;
    let mut buf = String::new();
    // .unwrap() handles the possibility of a write error — error handling
    // is covered in a later module
    write!(buf, "name: {}, ", "Rust").unwrap();
    writeln!(buf, "version: {}", 2024).unwrap();
    println!("write! built: {buf}");
    // Note: for writing to files or network streams, use std::io::Write
    // instead of std::fmt::Write. These are separate traits.

    // Implementing Display for custom types is covered in module 008
    // (section 5: Implementing Standard Library Traits).
}

// =================================================================================================
// Section 4: Diagnostic and Placeholder Macros
// =================================================================================================

/*
## Diagnostic and Placeholder Macros

- **`panic!("msg")`** — immediately aborts the current thread with an error message. This is the
  mechanism behind integer overflow (debug mode), out-of-bounds indexing, and `.unwrap()` on `None`.
- **`todo!()`** — marks code that is planned but not yet implemented. Panics with "not yet
  implemented" when reached. Commonly used as a placeholder while prototyping.
- **`unimplemented!()`** — similar to `todo!()`, but signals that a feature is deliberately not
  implemented (e.g., an unused trait method). Panics with "not implemented".
- **`unreachable!()`** — marks code paths that should logically never execute. Panics if reached,
  indicating a logic bug.
- All four macros produce a **panic** and have return type `!` (never type), so they can appear in
  any type context.
*/

fn diagnostic_macros() {
    // panic! — aborts execution with a message
    // Uncomment to see the panic:
    // panic!("something went terribly wrong");

    // todo! — placeholder for unfinished code
    fn calculate_tax(_income: f64) -> f64 {
        // Panics with "not yet implemented" if called
        todo!()
    }
    // Uncomment to see the panic:
    // calculate_tax(50_000.0);
    _ = calculate_tax; // suppress unused warning

    // unimplemented! — signals deliberately missing functionality
    fn export_to_pdf() {
        unimplemented!("PDF export is not supported yet");
    }
    // Uncomment to see the panic:
    // export_to_pdf();
    _ = export_to_pdf; // suppress unused warning

    // unreachable! — marks impossible code paths
    // Using a string here for simplicity — enums are covered in a later module
    let direction = "north";
    let _code = match direction {
        "north" => 0,
        "south" => 1,
        "east" => 2,
        "west" => 3,
        _ => unreachable!("only cardinal directions are valid"),
    };

    // All four can appear in type contexts (return type is !)
    let _value: i32 = if true { 42 } else { todo!() };

    println!("diagnostic_macros section executed");
}

// =================================================================================================
// Section 5: Compiler Directives (Attributes)
// =================================================================================================

/*
## Compiler Directives (Attributes)

- **Attributes** are metadata annotations that instruct the compiler how to handle code.
- **Outer attributes** `#[...]` apply to the item that **follows** them (a function, struct,
  variable declaration, etc.).
- **Inner attributes** `#![...]` apply to the **enclosing** item (typically the crate root or module
  file).
- **Lint levels** control how the compiler reports diagnostics:
  - `#[allow(name)]` — silences the warning entirely.
  - `#[warn(name)]` — emits a warning (the default for most lints).
  - `#[deny(name)]` — turns the lint into a **compile error**.
  - `#[forbid(name)]` — like `deny`, but cannot be overridden by inner code.
  Common lint targets: `unused_variables`, `dead_code`, `unused_imports`.
- An inner attribute `#![allow(...)]` placed at the top of a file applies the lint level to the
  **entire file**.
- Attributes use the syntax `#[directive(arguments)]`.
- **`#[cfg(...)]`** enables **conditional compilation** — the annotated item is only compiled if the
  condition is met.
  - `#[cfg(test)]` — compile only when running tests.
  - `#[cfg(target_os = "linux")]` — compile only on Linux.
  - Conditions can be combined with logical operators: `all(...)` (AND), `any(...)` (OR), `not(...)`
    (NOT).
  - The `cfg!()` **macro** evaluates the condition at compile time and returns a `bool` for use in
    runtime expressions. Important distinction: `#[cfg(...)]` on an item **removes it entirely**
    from compilation when the condition is false (dead code is not compiled at all). `cfg!(...)` is
    a regular `bool` expression, so **both branches must be valid Rust on every target** — you
    cannot use `cfg!()` to guard references to items that exist only on specific platforms. Use
    `#[cfg(...)]` for that.
*/

fn compiler_directives() {
    // #[allow(dead_code)] suppresses "never used" warning for a function
    #[allow(dead_code)]
    fn unused_helper() -> i32 {
        42
    }

    // #[allow(unused_variables)] suppresses "unused variable" warning
    #[allow(unused_variables)]
    let unused_binding = 100;

    // --- Lint levels ---
    // #[warn(...)] is the default — explicitly setting it has no visible
    // effect here, but you can escalate to #[deny(...)] to turn a
    // warning into a compile error:
    // #[deny(unused_variables)]
    // let unused = 42; // would be a compile ERROR, not just a warning

    // File-level directive (placed at the very top of a file):
    // #![allow(unused_variables)]
    // This would suppress the warning for the entire file.

    // #[forbid(...)] vs #[deny(...)]:
    // Both turn a lint into a compile error, but #[forbid] prevents
    // inner code from downgrading it back to #[allow]. Example:
    // #[forbid(unused_variables)]
    // fn strict() {
    //     #[allow(unused_variables)] // ERROR: allow(unused_variables)
    //     let x = 5;                // overruled by outer forbid
    // }

    // --- Conditional compilation with #[cfg(...)] ---
    // The cfg!() macro returns a bool at compile time
    if cfg!(target_os = "macos") {
        println!("compiled on macOS");
    } else if cfg!(target_os = "linux") {
        println!("compiled on Linux");
    } else if cfg!(target_os = "windows") {
        println!("compiled on Windows");
    } else {
        println!("compiled on another OS");
    }

    // --- cfg combinations: all(), any(), not() ---
    // all() — true when ALL conditions are met (logical AND)
    if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        println!("running on Apple Silicon Mac");
    }

    // any() — true when ANY condition is met (logical OR)
    if cfg!(any(target_os = "macos", target_os = "linux")) {
        println!("running on a Unix-like OS");
    }

    // not() — inverts the condition
    if cfg!(not(target_os = "windows")) {
        println!("not running on Windows");
    }

    // debug_assertions — true in debug builds, false in release
    if cfg!(debug_assertions) {
        println!("debug mode (cargo build)");
    } else {
        println!("release mode (cargo build --release)");
    }

    // #[cfg(test)] is used to compile code only during `cargo test`
    // — shown as a comment since it only applies to test modules:
    // #[cfg(test)]
    // mod tests { ... }

    // --- #[cfg(...)] as an attribute on items ---
    // Unlike cfg!() which returns bool at runtime, #[cfg(...)] on an
    // item removes it entirely from compilation if the condition is false.

    #[cfg(target_os = "macos")]
    fn platform_info() -> &'static str {
        "macOS"
    }

    #[cfg(target_os = "linux")]
    fn platform_info() -> &'static str {
        "Linux"
    }

    #[cfg(target_os = "windows")]
    fn platform_info() -> &'static str {
        "Windows"
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    fn platform_info() -> &'static str {
        "other"
    }

    println!("platform via #[cfg] attribute: {}", platform_info());

    println!("compiler directives section executed");
}

// =================================================================================================
// Section 5b: The `#[inline]` Attribute Family
// =================================================================================================

/*
## `#[inline]` — Cross-Crate Inlining Hints

- Inlining is the compile-time transformation of replacing a function call site with the body of the
  callee. It saves the call overhead and unlocks further optimizations (constant folding, dead-code
  elimination) across the call boundary.
- **Within a single crate** the compiler decides on its own which functions to inline. You almost
  never need to annotate anything.
- **Across crate boundaries** the situation is different. Non- generic functions are compiled in
  their defining crate and published as plain symbols — the consuming crate has no access to the
  function body and therefore **cannot** inline the call, no matter how hot it is. Generic functions
  are unaffected: because monomorphization happens in the consumer's crate, the compiler already has
  the body.
- **`#[inline]`** is a *hint* that asks the compiler to emit the function's body in the callee's
  metadata so downstream crates can inline it. It is the right annotation for small, hot, non-
  generic library functions.
- **`#[inline(always)]`** is a stronger request: "inline this at every call site you possibly can".
  Use sparingly — it overrides the compiler's cost-based heuristics and can grow binary size.
- **`#[inline(never)]`** is the opposite: "please do not inline this". Useful for cold
  error-handling paths, to keep hot paths compact, or to make flame-graph profiling easier.
- Attribute style is the same as every other attribute — outer `#[inline]` directly above the
  function item. Inside a generic crate that ships almost everything with inlining already
  available, there is usually nothing to add.
*/

// A small, non-generic helper that a downstream crate would want
// inlined at every call site.
#[inline]
fn square(x: i32) -> i32 {
    x * x
}

// `#[inline(always)]` requests unconditional inlining. The compiler
// usually respects this for small functions.
#[inline(always)]
fn is_even(x: i32) -> bool {
    x & 1 == 0
}

// `#[inline(never)]` keeps a cold error-reporting path out of the
// hot code, making the main path compile to a tighter loop.
#[inline(never)]
fn cold_path(reason: &str) -> String {
    format!("cold path reached: {reason}")
}

fn inline_attribute() {
    // The #[inline] attribute does not affect runtime semantics —
    // these calls look identical to any other function call. The
    // attribute only changes how the compiler emits and chooses to
    // inline the code downstream.
    let sum_of_squares: i32 = (1..=5).map(square).sum();
    println!("sum_of_squares(1..=5) = {sum_of_squares}"); // 55

    let evens = (1..=8).filter(|&n| is_even(n)).collect::<Vec<_>>();
    println!("evens in 1..=8        = {evens:?}");

    let msg = cold_path("example");
    println!("{msg}");

    println!("inline_attribute section executed");
}

// =================================================================================================
// Section 5c: Build-Environment Macros
// =================================================================================================

/*
## Build-Environment Macros

Rust has a handful of built-in macros that turn compile-time information — environment variables,
file contents, and source locations — into constant values in the compiled binary. They all run at
**compile time**, so their arguments must be string literals (not runtime `String` values) and any
file paths are resolved relative to the current source file (or `CARGO_MANIFEST_DIR` for `env!`,
depending on the macro).

- **`env!("NAME")`** — substitutes the value of the environment variable `NAME` at compile time and
  returns it as a `&'static str`. Compilation **fails** if the variable is unset. Cargo sets many
  `CARGO_*` variables for every build (`CARGO_PKG_VERSION`, `CARGO_PKG_NAME`, `CARGO_MANIFEST_DIR`,
  ...), which is the most common source of `env!` arguments.
- **`option_env!("NAME")`** — the fallible counterpart: `Option<&'static str>`, `None` when the
  variable is unset. Compilation does not fail. Use this when the variable is genuinely optional
  (for example, to embed a Git commit hash only when one is available).
- **`include_str!("path.txt")`** — inlines the referenced file's contents into the binary as a
  `&'static str`. The file must exist at compile time; the path is relative to the current source
  file (so from a module in `src/basic/mod.rs` the path `"../../Cargo.toml"` climbs to the workspace
  root).
- **`include_bytes!("path.dat")`** — the binary counterpart of `include_str!`: returns a reference
  to a fixed-size array `&'static [u8; N]`, where `N` is the file's length in bytes. Convenient for
  embedding static assets (icons, fonts, shader sources) directly into the executable.
- **`include!("other.rs")`** — inlines the target file as Rust **source code**: it expands to
  whatever expressions or items the file contains. Unlike `include_str!`, the file must parse as
  Rust. Commonly used to glue in auto-generated code produced by a `build.rs` script (see module 011
  §7 for the workflow).
- **Why compile-time?** Values produced by these macros become part of the binary, so a release
  artifact always knows its own version, its build-time commit hash, or embedded assets without any
  runtime file access. `const` items can be initialized from them (e.g., `const VERSION: &str =
  env!("CARGO_PKG_VERSION");`), which is impossible with a regular `std::env::var` call.
*/

// Embed the crate's version, read from Cargo.toml at compile time.
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

// `option_env!` returns None when the variable is not set. Pick a
// variable that is essentially never set at build time, so this
// consistently demonstrates the absent branch.
const BUILD_GIT_HASH: Option<&str> = option_env!("BUILD_GIT_HASH_FROM_CI");

fn build_environment_macros() {
    // env! — compile-time environment variable, baked into the binary.
    println!("built from package: {PKG_NAME} v{PKG_VERSION}");

    // option_env! — same thing, but the value is optional.
    match BUILD_GIT_HASH {
        Some(hash) => println!("build hash: {hash}"),
        None => println!("build hash: <not provided at compile time>"),
    }

    // include_str! — inline a file as a &'static str at compile time.
    // We point at Cargo.toml from this source file (src/basic/...),
    // which is two levels above us on disk.
    const CARGO_TOML: &str = include_str!("../../Cargo.toml");
    let first_line = CARGO_TOML.lines().next().unwrap_or("<empty>");
    println!(
        "include_str!(\"../../Cargo.toml\") — {} bytes, first line: {first_line:?}",
        CARGO_TOML.len()
    );

    // include_bytes! — inline the same file as a &'static [u8; N].
    // The macro returns a reference to a fixed-size array, so its
    // length is known at compile time.
    const CARGO_TOML_BYTES: &[u8] = include_bytes!("../../Cargo.toml");
    println!(
        "include_bytes!(\"../../Cargo.toml\") — {} bytes, first byte = {:#04x}",
        CARGO_TOML_BYTES.len(),
        CARGO_TOML_BYTES[0]
    );

    // include! — inlines a file as Rust source. Shown as a comment
    // because there is no generated-Rust file to pull in here; the
    // typical shape is:
    //
    //     include!(concat!(env!("OUT_DIR"), "/generated.rs"));
    //
    // …in combination with a build.rs script (module 011 §7) that
    // writes `generated.rs` into Cargo's OUT_DIR.

    println!("build_environment_macros section executed");
}

// =================================================================================================
// Section 6: Error Messages and Error Codes
// =================================================================================================

/*
## Error Messages and Error Codes

- The Rust compiler produces detailed, structured error messages consisting of:
  1. An **error code** starting with `E` followed by four digits (e.g., `E0308` for a type
     mismatch).
  2. A **human-readable description** of the problem.
  3. A **code span** highlighting the exact source location and the offending code.
  4. A **suggested fix** when the compiler can infer one.
- Extended explanations are available via the command: `rustc --explain E0308`.
- The full error code index is part of the official Rust documentation.
- **Warnings** follow the same format but do not prevent compilation. They signal potential issues
  (unused variables, unreachable code, etc.).
*/

fn error_messages() {
    // Error messages are a compile-time feature — we cannot trigger
    // one at runtime. Here is an example of code that would produce
    // an error, shown as a comment:

    // let x: i32 = "hello";
    // ERROR[E0308]: mismatched types
    //   |
    //   | let x: i32 = "hello";
    //   |        ---   ^^^^^^^ expected `i32`, found `&str`
    //   |        |
    //   |        expected due to this

    println!("Rust error messages include:");
    println!("  1. error code (e.g., E0308)");
    println!("  2. description of the problem");
    println!("  3. highlighted source location");
    println!("  4. suggested fix (when possible)");
    println!();
    println!("tip: run `rustc --explain E0308` for a detailed explanation");
}

pub fn run() {
    print_basics();
    escape_sequences();
    format_arguments();
    diagnostic_macros();
    compiler_directives();
    inline_attribute();
    build_environment_macros();
    error_messages();
}
