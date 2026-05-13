// =================================================================================================
// Section 1: Comments
// =================================================================================================

/*
## Comments

- **Single-line comments** start with `//`. Everything from `//` to
the end of the line is ignored by the compiler.
- **Block comments** use `/* ... */` and can span multiple lines.
- Unlike C/C++, Rust block comments **can be nested**:
`/* outer /* inner */ still outer */` is valid.
- **Documentation comments** use `///` (for the following item) or
`//!` (for the enclosing item). They support Markdown and are used by `cargo doc` to generate HTML
documentation.
- Block forms `/** ... */` (outer) and `/*! ... */` (inner) also
exist as the block-comment equivalents of `///` and `//!`, though the line forms are far more common
in practice.
*/

fn comments() {
    // This is a single-line comment

    /*
    This is a block comment
    spanning multiple lines.
    */

    /* Nested block comments are valid in Rust:
    /* this inner block is fine */
    and the outer block continues here.
    */

    println!("comments section executed");
}

// =================================================================================================
// Section 2: Variables and Type Inference
// =================================================================================================

/*
## Variables and Type Inference

- Variables are declared using the `let` keyword. This is called
**binding** — associating a name with a value.
- Rust **infers the type** from the assigned value when no type
annotation is provided. The default integer type is `i32`, and the default floating-point type is
`f64`.
- You can explicitly annotate the type: `let name: Type = value;`.
An explicit annotation overrides inference and causes a compile-time error if the value is
incompatible with the declared type.
- Type inference has **limits** — when the compiler cannot determine
the concrete type from context alone, you must provide an explicit annotation. Common cases:
- Empty collections: `Vec::new()` has no elements to infer from.
- Methods like `.collect()` that can produce multiple collection types.
- `.parse()` which can return any type implementing `FromStr`.
- The **turbofish** syntax `::<T>` lets you specify type parameters
on a function or method call directly: `"42".parse::<i32>()`. It is an alternative to annotating the
binding with `let x: i32`.
- A variable does **not** need to be initialized at declaration, but
it **must** be initialized before its first use. The compiler enforces this statically.
- Prefixing a variable name with `_` suppresses the "unused variable"
compiler warning. This is a convention recognized by the compiler, not special syntax.
- Important distinction: `_` (bare underscore) never creates a binding —
the value is dropped **immediately**. `_name` creates a real binding that lives until end of scope.
This distinction matters only for types that perform cleanup on drop (e.g., file handles, `String`).
For `Copy` types like integers, there is no observable difference. This is the reason `let _ =
mutex.lock();` **does not hold the lock** (the guard drops immediately), while `let _guard =
mutex.lock();` does.
- Naming convention for variables: `snake_case` (lowercase with
underscores). Rust enforces its naming conventions through compiler warnings.
*/

fn variables_and_type_inference() {
    // Type inferred as i32 (default integer type)
    let a = 10;
    println!("a = {a} (type inferred as i32)");

    // Type inferred as f64 (default floating-point type)
    let pi = 3.14;
    println!("pi = {pi} (type inferred as f64)");

    // Explicit type annotation
    let b: u8 = 255;
    println!("b = {b} (explicitly typed as u8)");

    // Declaration without initialization — allowed, but the variable
    // must be assigned before its first use
    let c: i32;
    c = 42;
    println!("c = {c} (declared first, assigned later)");

    // The underscore prefix suppresses "unused variable" warnings
    let _unused = 100;

    // --- Type inference limits ---

    // Empty collection: compiler cannot infer the element type
    let numbers: Vec<i32> = Vec::new(); // no later usage to constrain T — annotation needed here
    println!("empty vec: {numbers:?}");

    // Turbofish syntax — specify the type parameter on the call itself
    let parsed = "42".parse::<i32>().unwrap();
    println!("turbofish parse: {parsed}");

    // Collect needs a target type — either via annotation or turbofish
    let squares: Vec<i32> = (1..=5).map(|x| x * x).collect();
    let squares2 = (1..=5).map(|x| x * x).collect::<Vec<i32>>(); // equivalent
    println!("squares: {squares:?}");
    println!("squares (turbofish): {squares2:?}");

    // --- Destructuring let ---
    // `let` accepts patterns, not just single names.
    // Pattern matching and destructuring are covered in depth in a
    // later module; this is a preview of the simplest case.
    let (x, y) = (10, 20); // tuple destructuring
    println!("destructured: x={x}, y={y}");

    let (_, second, _) = (1, 2, 3); // ignore first and third
    println!("kept only second: {second}");

    // Struct destructuring works too (covered fully in module 007)
    // let Point { x, y } = point;

    // --- let _ = expr: discard a #[must_use] value ---
    // Some return values produce warnings if unused; `let _ = expr`
    // explicitly discards them.
    let _ = "42".parse::<i32>(); // Result is #[must_use]; _ discards it
}

// =================================================================================================
// Section 3: Mutability
// =================================================================================================

/*
## Mutability

- Variables in Rust are **immutable by default**. Attempting to
reassign an immutable variable causes a compile-time error.
- Use `let mut` to declare a mutable variable, allowing its value
to be changed after initialization.
- Mutation changes the **value** but not the **type** — assigning a
value of a different type to a mutable variable is a compile-time error.
- Immutability by default is a deliberate design choice that
encourages safer, more predictable code. You opt into mutability explicitly.
*/

fn mutability() {
    // Immutable variable — reassigning would cause a compile-time error
    let a = 10;
    println!("a = {a}");
    // a = 20; // ERROR: cannot assign twice to immutable variable `a`

    // Mutable variable — reassignment is allowed
    let mut b = 10;
    println!("b before mutation: {b}");
    b = 30;
    println!("b after mutation: {b}");

    // Mutation cannot change the type:
    // b = "hello"; // ERROR: expected integer, found `&str`
}

// =================================================================================================
// Section 4: Scope
// =================================================================================================

/*
## Scope

- A scope is a region of code delimited by curly braces `{}`.
- Variables are valid from the point of their declaration to the end
of the enclosing scope.
- When a scope ends, all variables declared within it are **dropped** —
their destructors run in reverse order of declaration. (Drop order and what "dropping" actually does
are covered more fully in the ownership module.)
- Inner scopes can access variables from outer scopes, but outer
scopes **cannot** access variables declared in inner scopes.
- A block `{}` is an **expression** — it evaluates to the value of
its last expression (without a semicolon). If the last statement ends with a semicolon, the block
evaluates to `()` (the unit type).
- `let` is a **statement**, not an expression — unlike C/Ruby where
`x = y = 6` assigns `6` to both variables, `let x = (let y = 6);` is rejected by the Rust compiler
with "expected expression, found `let` statement". You cannot nest a `let` inside another `let`'s
initializer; each `let` is a separate statement. This is why multi-step transformations are done
with shadowing or block expressions rather than chained assignments.
*/

fn scope() {
    let outer = 10;

    {
        // Inner scope can read variables from the outer scope
        println!("outer variable seen from inner scope: {outer}");

        let inner = 20;
        println!("inner variable: {inner}");
    }
    // `inner` is no longer accessible here — it was dropped when
    // the inner scope ended
    // println!("{inner}"); // ERROR: cannot find value `inner` in this scope

    println!("outer variable after inner scope ends: {outer}");

    // A block is an expression — it returns the value of its last
    // expression (note: no semicolon on the last line inside the block)
    let result = {
        let a = 5;
        let b = 3;
        a + b // no semicolon — this value is returned from the block
    };
    println!("block expression result: {result}");

    // Adding a semicolon to the last line makes the block return ()
    let unit_result: () = {
        let _ = 5 + 3; // a `let` statement; block has no trailing expression → ()
    };
    println!("block with semicolon returns unit: {unit_result:?}");

    // `let` is a statement, not an expression — nesting it fails to compile.
    // Unlike C/Ruby (`x = y = 6`), the following is rejected by the compiler:
    //
    //     let x = (let y = 6);
    //     //       ^^^ error: expected expression, found `let` statement
    //
    // Use shadowing or separate statements for multi-step transformations:
    let y = 6;
    let x = y; // separate statements
    println!("x = {x}, y = {y}");
}

// =================================================================================================
// Section 5: Shadowing
// =================================================================================================

/*
## Shadowing

- Shadowing allows declaring a new variable with the same name as an
existing one using `let`. The new variable **shadows** (replaces) the previous one within the
current scope.
- Unlike mutation, shadowing creates an **entirely new variable**.
This means the type can change.
- Shadowing in an inner scope only affects that inner scope; the
original variable is restored when the inner scope ends.
- A common use case is transforming a value through multiple steps
without needing distinct names for each intermediate result.
- Shadowing is idiomatic when the transformation pipeline is clear
(e.g., `let data = data.trim(); let data = data.to_uppercase()`). Avoid shadowing when it obscures
meaning — e.g., reusing a name for a value with a completely unrelated purpose.

### Shadowing vs. Mutation

- **Mutation** (`mut`): modifies the same variable in place; the type
cannot change.
- **Shadowing** (`let`): creates a new variable that happens to have
the same name; the type can change. The previous binding becomes inaccessible in the same scope, or
is restored after an inner scope ends.
*/

fn shadowing() {
    // Shadowing with a type change — the name stays, the type changes
    let value = "42"; // &str
    let value = value.len(); // usize (different type, same name)
    println!("shadowed value (length of \"42\"): {value}");

    // Shadowing in an inner scope — outer variable is restored after
    let x = 10;
    {
        let x = x as f64 * 2.5; // shadows outer x with a new f64 variable
        println!("x in inner scope: {x}");
    }
    println!("x in outer scope (unchanged): {x}");

    // Multi-step transformation using shadowing — no intermediate
    // variable names needed
    let data = "  hello world  ";
    let data = data.trim(); // &str, whitespace removed
    let data = data.to_uppercase(); // String, converted to uppercase
    println!("transformed data: {data}");
}

pub fn run() {
    comments();
    variables_and_type_inference();
    mutability();
    scope();
    shadowing();
}
