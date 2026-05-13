// =================================================================================================
// Section 1: Functions
// =================================================================================================

/*
## Functions

- Declared with the `fn` keyword. Naming convention: `snake_case`.
- Parameters require **explicit type annotations**: `fn add(a: i32, b: i32)`.
- The return type is declared with `->`: `fn add(a: i32, b: i32) -> i32`.
- **Implicit return**: the last expression in the function body (without a semicolon) is the return
  value. This follows the same block-expression rule covered in module 001.
- **Explicit return**: the `return` keyword can be used for early returns before the end of the
  function body.
- A function without a declared return type implicitly returns `()` (the unit type).
- Parameters are **immutable by default**, following Rust's standard mutability rules. Prefix a
  parameter with `mut` to allow mutation of that local copy inside the function body.
- Rust passes **values** to functions, not mutability — a `mut` variable can be passed to an
  immutable parameter (the value is copied/moved), and the receiving function decides its own
  mutability.
- A parameter's mutability can also be changed inside the function body via **shadowing**: `let mut
  x = x;`.
- Similarly, **returned values carry no mutability** — the caller decides whether to bind the result
  as `let` or `let mut`.
- To **return multiple values**, use a compound type (tuple, struct): `fn bounds(a: i32, b: i32) ->
  (i32, i32)`.
- Functions can be called before their definition in the same scope — Rust does not require forward
  declarations.
- Rust does **not** support **default parameter values**. Instead, use the builder pattern or
  provide multiple constructor functions (e.g., `new()` and `with_options()`).
- Functions can be **recursive** — a function can call itself. Rust has no tail-call optimization
  guarantee, so deep recursion can overflow the stack. The main thread inherits its stack from the
  OS (typically 8 MiB on Linux/macOS, 1 MiB on Windows), while threads spawned via
  `std::thread::spawn` default to 2 MiB on all Tier-1 platforms (configurable via
  `thread::Builder::stack_size` or the `RUST_MIN_STACK` environment variable). Prefer iterative
  solutions for unbounded depth.
- Functions can be defined **inside** other functions (nested functions). Nested functions cannot
  capture variables from the enclosing scope — unlike closures, which are covered in module 007.
- **Destructuring in parameters**: function parameters can use patterns to destructure tuples,
  structs, and nested types directly in the parameter list — no separate `let` needed.
- **Function pointers**: functions have a type (`fn(i32) -> i32`) that can be used as a parameter
  type. This allows passing named functions to other functions. Function pointers are `Copy` and
  implement all three closure traits (`Fn`, `FnMut`, `FnOnce`). Closures (anonymous functions that
  capture their environment) are covered in module 007.
- Functions can also be **generic** — parameterized over types with `<T>`. Generic functions are
  covered in module 008.
*/

// Function with two parameters and a return type — implicit return
// (no semicolon on the last expression)
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Function with explicit early return
fn classify_number(n: i32) -> String {
    if n > 0 {
        return "positive".to_string();
    }
    if n < 0 {
        return "negative".to_string();
    }
    "zero".to_string() // implicit return for the last case
}

// Mutable parameter — `mut` allows modifying the local copy inside
// the function. The caller's original value is unaffected.
fn increment(mut value: i32) -> i32 {
    value += 1;
    value
}

fn functions() {
    // Calling a function with parameters and a return value
    let sum = add(3, 7);
    println!("3 + 7 = {sum}");

    // Explicit return — classify_number uses `return` for early exits
    println!("10 is {}", classify_number(10));
    println!("-5 is {}", classify_number(-5));
    println!("0 is {}", classify_number(0));

    // Mutable parameter — the function receives a copy and mutates it
    let original = 10;
    let result = increment(original);
    println!("increment({original}) = {result}");

    // A function without a return type returns () implicitly
    fn greet(name: &str) {
        println!("hello, {name}!");
    }
    greet("Rust");

    // Nested function — defined inside another function.
    // It cannot access variables from the enclosing scope.
    fn double(x: i32) -> i32 {
        // println!("{original}"); // ERROR: can't capture enclosing variable
        x * 2
    }
    println!("double(8) = {}", double(8));

    // Functions can be called before their definition (no forward
    // declaration needed) — see add() and classify_number() above

    // Recursive function — computes factorial
    fn factorial(n: u64) -> u64 {
        if n <= 1 { 1 } else { n * factorial(n - 1) }
    }
    println!("factorial(6) = {}", factorial(6)); // 720

    // No default parameters — Rust requires explicit arguments.
    // Instead, provide multiple constructors or use the builder pattern:
    // fn connect(host: &str, port: u16) { ... }
    // fn connect_default() { connect("localhost", 8080) }

    // --- Values, mutability, and returns ---

    // Passing a `mut` variable to an immutable parameter works fine —
    // only the value is transferred, not the mutability.
    fn print_value(v: i32) {
        println!("received: {v}");
    }
    let mut x = 42;
    print_value(x); // x is mut, parameter is not — perfectly fine
    x += 1; // x is still usable and mutable
    println!("x after call: {x}");

    // Shadowing a parameter to change its mutability inside the body
    fn adjust(value: i32) -> i32 {
        let mut value = value; // shadow with `let mut` to allow mutation
        value *= 2;
        value += 1;
        value
    }
    println!("adjust(5) = {}", adjust(5));

    // Returned values carry no mutability — the caller decides
    let immutable_result = adjust(3);
    let mut mutable_result = adjust(3);
    mutable_result += 100;
    println!("immutable: {immutable_result}, mutable: {mutable_result}");

    // Returning multiple values via a tuple
    fn min_max(a: i32, b: i32) -> (i32, i32) {
        if a < b { (a, b) } else { (b, a) }
    }
    let (lo, hi) = min_max(7, 3);
    println!("min_max(7, 3) = ({lo}, {hi})");

    // --- Destructuring in function parameters ---

    // Tuple destructuring — bind elements directly in the parameter list
    fn sum_pair((a, b): (i32, i32)) -> i32 {
        a + b
    }
    println!("sum_pair((3, 7)) = {}", sum_pair((3, 7)));

    // Struct destructuring in parameters
    // (structs are formally introduced in a later module — this previews
    // their use in parameter destructuring only)
    struct Pos {
        x: i32,
        y: i32,
    }

    fn print_pos(Pos { x, y }: Pos) {
        println!("position: ({x}, {y})");
    }
    print_pos(Pos { x: 10, y: 20 });

    // Ignoring fields with `_` or `..`
    fn x_only(Pos { x, .. }: &Pos) -> i32 {
        *x
    }
    let p = Pos { x: 5, y: 99 };
    println!("x_only = {}", x_only(&p));

    // Ignoring elements with `_` in tuple parameters
    fn first_only((first, _): (i32, i32)) -> i32 {
        first
    }
    println!("first_only((42, 99)) = {}", first_only((42, 99)));

    // --- Function pointers ---
    // Functions can be passed as arguments using function pointer types.
    fn apply(f: fn(i32) -> i32, value: i32) -> i32 {
        f(value)
    }
    fn square(x: i32) -> i32 {
        x * x
    }
    fn negate(x: i32) -> i32 {
        -x
    }

    println!("apply(square, 5) = {}", apply(square, 5));
    println!("apply(negate, 5) = {}", apply(negate, 5));

    // Function pointers can be stored in variables and collections
    let operations: [fn(i32) -> i32; 2] = [square, negate];
    for op in &operations {
        println!("  op(3) = {}", op(3));
    }

    // A function can return a function pointer — factory pattern
    fn identity(x: i32) -> i32 {
        x
    }
    fn make_operation(name: &str) -> fn(i32) -> i32 {
        match name {
            "square" => square,
            "negate" => negate,
            _ => identity,
        }
    }
    let op = make_operation("square");
    println!("make_operation(\"square\")(7) = {}", op(7));
}

// =================================================================================================
// Section 2: If Expressions
// =================================================================================================

/*
## If Expressions

- `if` allows conditional execution of code. The condition must be of type `bool` — there is no
  implicit conversion from integers or other types.
- Optional `else if` and `else` branches handle additional conditions and the fallback case.
- Rust is an **expression-oriented language** — most constructs (`if`, `match`, `loop`, blocks `{}`)
  produce values. This eliminates the statement-vs-expression divide found in C-style languages,
  enabling patterns like `let x = match ...`.
- `if` is an **expression** in Rust — it evaluates to a value and can be assigned to a variable:
  `let x = if condition { a } else { b };`.
- When used as an expression, **all branches must return the same type**, and the `else` branch is
  required (so the expression always produces a value).
- Unlike `match`, an `if/else if` chain is **not exhaustive** — the compiler does not verify that
  every possible value is handled.
*/

fn if_expressions() {
    let number = 7;

    // Basic if/else
    if number % 2 == 0 {
        println!("{number} is even");
    } else {
        println!("{number} is odd");
    }

    // If/else if/else chain
    let marks = 85;
    if marks >= 90 {
        println!("grade: A");
    } else if marks >= 80 {
        println!("grade: B");
    } else if marks >= 70 {
        println!("grade: C");
    } else {
        println!("grade: F");
    }

    // If as an expression — assign the result to a variable.
    // All branches must return the same type; else is required.
    let grade = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else {
        'F'
    };
    println!("marks {marks} => grade '{grade}'");

    // The condition must be bool — no implicit conversion:
    // if 1 { ... } // ERROR: expected `bool`, found integer
}

// =================================================================================================
// Section 3: Match Expressions
// =================================================================================================

/*
## Match Expressions

- `match` compares a value against a series of **patterns** and executes the code for the first
  matching pattern.
- `match` is **exhaustive** — every possible value must be covered. The `_` wildcard pattern matches
  anything not explicitly handled.
- Patterns can be:
  - **Literal values**: `1`, `"hello"`.
  - **Multiple values** (or-pattern): `1 | 2 | 3`.
  - **Ranges**: `1..=5` (inclusive range).
  - **Wildcards**: `_` (match anything, discard the value).
  - **Bindings**: `value` captures the matched value into a variable; `val @ (1 | 2)` combines a
    binding with a pattern.
- **Match guards** add an extra `if` condition to a pattern arm: `n if n > 0 => ...`.
- **Guards do not participate in exhaustiveness checking**: the compiler cannot reason about guard
  expressions, so a guarded arm is always treated as "might not match". For example, `match n { x if
  x >= 0 => ..., x if x < 0 => ... }` is rejected as non-exhaustive (E0004) — an unguarded catch-all
  (`_ =>` or a bare binding like `n =>`) is required.
- **Guard + or-pattern precedence**: when a match guard is combined with an or-pattern, the guard
  applies to the **whole** or-pattern. `4 | 5 | 6 if y` is parsed as `(4 | 5 | 6) if y`, not as `4 |
  5 | (6 if y)` — the entire arm matches any of the three values, but only when the guard is true.
- Matching on **tuples**: `match (a, b) { (1, 2) => ..., _ => ... }`.
- `match` is an **expression** — it returns a value. All arms must return the same type.
- **Common match mistakes**:
  - **Unreachable patterns**: placing a broader pattern (like `_`) before a more specific one makes
    the specific arm dead code.
  - **Overlapping patterns**: when multiple arms match the same value, only the first matching arm
    executes — the rest are unreachable. The compiler warns about both.
*/

fn match_expressions() {
    // Match on integer with literal patterns and a wildcard catch-all
    let dice_roll = 6;
    match dice_roll {
        6 => println!("you rolled a six — you win!"),
        1 => println!("you rolled a one — critical fail!"),
        _ => println!("you rolled {dice_roll} — try again"),
    }

    // Multiple values with `|` (or-pattern)
    let direction = 'N';
    match direction {
        'N' | 'S' => println!("vertical axis"),
        'E' | 'W' => println!("horizontal axis"),
        _ => println!("unknown direction"),
    }

    // Range pattern — inclusive range with `..=`
    let score = 75;
    let rating = match score {
        90..=100 => "excellent",
        70..=89 => "good",
        50..=69 => "average",
        _ => "below average",
    };
    println!("score {score} => {rating}");

    // Binding — capture the matched value into a variable
    let code = 404;
    match code {
        200 => println!("OK"),
        val @ (400 | 404 | 500) => println!("error code: {val}"),
        other => println!("unhandled code: {other}"),
    }

    // Match guard — additional `if` condition on a pattern arm
    let number = 4;
    match number {
        n if n % 2 == 0 => println!("{n} is even"),
        n => println!("{n} is odd"),
    }

    // Compound match guard — multiple conditions
    let temp = 22;
    let weather = match temp {
        t if t < 0 => "freezing",
        t if t >= 0 && t < 15 => "cold",
        t if t >= 15 && t < 30 => "pleasant",
        _ => "hot",
    };
    println!("temperature {temp}°C is {weather}");

    // Matching on a tuple
    let weather = ("cloudy", "warm");
    match weather {
        ("clear", "warm") => println!("nice day"),
        ("cloudy", "cold") => println!("dark and unpleasant"),
        ("cloudy", _) => println!("cloudy but not bad"),
        _ => println!("weather unknown"),
    }

    // Exhaustiveness — the compiler rejects non-exhaustive matches:
    // let val = 5;
    // match val {
    //     1 => "one",
    //     2 => "two",
    // }
    // ERROR: non-exhaustive patterns: `i32::MIN..=0_i32`
    //        and `3_i32..=i32::MAX` not covered

    // Common match mistakes — the compiler warns about these:

    // Unreachable pattern — wildcard before a specific arm
    // match 5 {
    //     _ => println!("catch-all"),
    //     5 => println!("five"),  // WARNING: unreachable pattern
    // }

    // Overlapping patterns — first match wins, rest are dead code
    // match 3 {
    //     1..=5 => println!("one to five"),
    //     3 => println!("three"),  // WARNING: unreachable pattern
    //     _ => println!("other"),
    // }

    // Match as expression — all arms return the same type
    let day = 3;
    let name = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        _ => "weekend",
    };
    println!("day {day} = {name}");

    // --- Pattern matching on strings ---
    let greeting = "hello";
    match greeting {
        "hello" | "hi" | "hey" => println!("informal greeting"),
        "good morning" => println!("formal greeting"),
        s if s.starts_with("good") => println!("starts with 'good': {s}"),
        _ => println!("unknown greeting: {greeting}"),
    }

    // Match guard + or-pattern precedence: the `if` applies to the
    // whole `4 | 5 | 6`, not just the last value. The arm matches
    // any of the three numbers, but only when the guard is true.
    let n = 4;
    let y = false;
    #[allow(clippy::manual_range_patterns)] // demonstrating the or-pattern syntax
    match n {
        4 | 5 | 6 if y => println!("one of {{4, 5, 6}} and y is true"),
        _ => println!("anything else (here because y is false)"),
    }
}

// =================================================================================================
// Section 4: Infinite Loops (loop)
// =================================================================================================

/*
## Infinite Loops (`loop`)

- `loop` creates an infinite loop that runs until explicitly stopped with `break`.
- `break` exits the current loop. `continue` skips the rest of the current iteration and starts the
  next one.
- **Labeled loops**: a label (`'label:`) placed before a loop allows `break 'label` to exit that
  specific loop, or `continue 'label` to skip to the next iteration of that specific loop. This is
  useful for controlling nested loops.
- `loop` can **return a value** via `break value;` — this makes the loop an expression whose result
  can be assigned to a variable.
- **`break` vs `return`** — different scopes. `break` exits only the *current* loop (or the loop
  named by a label, e.g. `break 'outer`). `return` always exits the *entire function*, no matter how
  deeply nested the loop is. When you want to leave the function from inside a nested loop, use
  `return`; when you only want to end the iteration, use `break` (optionally with a label). The same
  distinction applies to their value-carrying forms: `break value;` yields a value from the loop
  expression, while `return value;` yields a value from the whole function.
- **`loop` vs `while true`**: these are **not** interchangeable.
  - **`break value` is loop-only**: using `break expr;` inside a `while` or `for` loop is a hard
    compile error (E0571: "`break` with value from a `while` loop"). Only `loop` supports carrying a
    value out via `break`. The reason is that `while` and `for` always have a fall-through path
    (condition becomes false / iterator exhausted), so their result type must stay `()`.
  - **`loop {}` has type `!` (never)** when it has no reachable `break`, so code after it is
    unreachable and it can appear in any type context (e.g., `let x: i32 = loop {};`). `while true
    {}` has type `()` — the compiler does not infer `!` for it, even though it never terminates.
    (Clippy's `while_true` lint flags `while true` for this reason.)
*/

fn infinite_loops() {
    // Basic loop with break — count up to 5
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 5 {
            break;
        }
    }
    println!("counter after loop: {counter}");

    // Loop returning a value via `break value`
    let mut n = 0;
    let result = loop {
        n += 1;
        if n == 10 {
            break n * 2; // the loop evaluates to 20
        }
    };
    println!("loop result: {result}");

    // Labeled loop — break out of the outer loop from an inner loop
    let mut outer_count = 0;
    'outer: loop {
        let mut inner_count = 0;
        loop {
            inner_count += 1;
            if inner_count == 3 {
                outer_count += 1;
                if outer_count == 2 {
                    break 'outer; // exits the outer loop
                }
                break; // exits only the inner loop
            }
        }
    }
    println!("outer loop ran {outer_count} times");

    // Labeled loop returning a value — combine labels with break-value
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let target = 5;
    let position = 'search: loop {
        let mut row = 0;
        while row < matrix.len() {
            let mut col = 0;
            while col < matrix[row].len() {
                if matrix[row][col] == target {
                    break 'search (row, col); // return from labeled loop
                }
                col += 1;
            }
            row += 1;
        }
        break 'search (usize::MAX, usize::MAX); // not found
    };
    println!("found {target} at row={}, col={}", position.0, position.1);

    // Continue — skip the rest of the current iteration
    let mut i = 0;
    let mut sum = 0;
    loop {
        i += 1;
        if i > 10 {
            break;
        }
        if i % 2 != 0 {
            continue; // skip odd numbers
        }
        sum += i;
    }
    println!("sum of even numbers 1..=10: {sum}");

    // continue 'label — skip to the next iteration of a labeled outer loop
    'outer: for i in 0..5 {
        for j in 0..5 {
            if j == 3 {
                continue 'outer; // skip the rest of inner loop AND advance outer
            }
            if i == j {
                println!("  diagonal match: ({i}, {j})");
            }
        }
    }

    // `return` inside a loop exits the entire function, not just the loop.
    // Contrast with `break` (only exits the innermost / labeled loop).
    fn find_first_negative(grid: &[[i32; 3]]) -> Option<(usize, usize)> {
        for (r, row) in grid.iter().enumerate() {
            for (c, &value) in row.iter().enumerate() {
                if value < 0 {
                    // exits find_first_negative entirely — both loops included
                    return Some((r, c));
                }
            }
        }
        None
    }

    let grid = [[1, 2, 3], [4, -5, 6], [7, 8, 9]];
    println!("first negative at: {:?}", find_first_negative(&grid));
}

// =================================================================================================
// Section 5: While Loops
// =================================================================================================

/*
## While Loops

- `while` runs a block of code **as long as a condition is true**.
- The condition is evaluated **before** each iteration. If the condition is `false` from the start,
  the body never executes.
- Suitable when the number of iterations is **not known in advance** and depends on a runtime
  condition.
*/

fn while_loops() {
    // Basic while loop — count down from 5
    let mut countdown = 5;
    while countdown > 0 {
        println!("countdown: {countdown}");
        countdown -= 1;
    }
    println!("liftoff!");

    // While loop that may not execute at all — condition is false
    // from the start
    let threshold = 100;
    let value = 50;
    while value > threshold {
        // this body never runs because 50 > 100 is false
        println!("this will not print");
    }
    println!("value ({value}) was never above threshold ({threshold})");
}

// =================================================================================================
// Section 6: For Loops and Ranges
// =================================================================================================

/*
## For Loops and Ranges

- `for` iterates over an **iterator**. The most common forms are iterating over ranges and
  collections.
- **Ranges**: `0..5` produces values 0, 1, 2, 3, 4 (exclusive end). `0..=5` produces 0, 1, 2, 3, 4,
  5 (inclusive end).
- Ranges are **ascending only** — `5..0` is empty and produces no iterations. Use `.rev()` to
  iterate in descending order: `(0..=5).rev()`.
- `.step_by(n)` changes the increment: `(0..10).step_by(2)` produces 0, 2, 4, 6, 8.
- `.enumerate()` yields `(index, value)` pairs, enabling access to the iteration count alongside
  each value.
- Iterating over arrays: `for element in array` moves values out; `for element in &array` borrows
  them.
- Destructuring in for loops: `for (key, value) in pairs` unpacks tuples directly.
- Ranges are **types** (`Range<i32>`, `RangeInclusive<i32>`, etc.) with their own methods —
  `.rev()`, `.step_by()`, `.count()`, and more. **Dot notation** (`value.method()`) is how you call
  methods on a value.
- `0..` (`RangeFrom`) creates an **unbounded** ascending range with no upper limit. In a `for` loop
  this runs forever unless stopped with `break`.
- `.count()` consumes a range and returns the number of elements.
*/

fn for_loops_and_ranges() {
    // Basic range — exclusive end
    print!("0..5:    ");
    for i in 0..5 {
        print!("{i} ");
    }
    println!();

    // Inclusive range
    print!("0..=5:   ");
    for i in 0..=5 {
        print!("{i} ");
    }
    println!();

    // Descending order with .rev()
    print!("rev:     ");
    for i in (0..=5).rev() {
        print!("{i} ");
    }
    println!();

    // Step size with .step_by()
    print!("step(2): ");
    for i in (0..10).step_by(2) {
        print!("{i} ");
    }
    println!();

    // Descending range is empty — produces no iterations
    print!("5..0:    ");
    for i in 5..0 {
        print!("{i} "); // this never runs
    }
    println!("(empty — descending ranges produce nothing)");

    // Iterating over an array by reference (borrow)
    let colors = ["red", "green", "blue"];
    for color in &colors {
        print!("{color} ");
    }
    println!();

    // Iterating by value (move) — for Copy types this works the same,
    // but for non-Copy types the collection would be consumed
    for color in colors {
        print!("{color} ");
    }
    println!("(moved)");

    // colors is still valid here because [&str; 3] is Copy — with
    // non-Copy types this would fail (the array was consumed above)

    // .enumerate() — get (index, value) pairs
    for (index, color) in colors.iter().enumerate() {
        println!("  [{index}] {color}");
    }

    // Destructuring tuple pairs in a for loop
    let points = [(0, 0), (3, 4), (6, 8)];
    for (x, y) in points {
        println!("  point: ({x}, {y})");
    }

    // Unbounded range (RangeFrom) — runs forever without break
    let mut sum = 0;
    for i in 0.. {
        if i >= 5 {
            break;
        }
        sum += i;
    }
    println!("sum of 0..5 via unbounded range: {sum}");

    // Ranges are types with methods — .count() consumes the range
    let how_many = (0..10).count();
    println!("(0..10).count() = {how_many}");

    let inclusive_count = (1..=5).count();
    println!("(1..=5).count() = {inclusive_count}");
}

// =================================================================================================
// Section 7: Diverging Functions (-> !)
// =================================================================================================

/*
## Diverging Functions (`-> !`)

- A function with return type `!` (the **never type**) is one that **never returns** to the caller.
  It either panics, loops forever, or exits the process.
- The `!` type is compatible with every other type, so a diverging expression can appear in any type
  context (e.g., both arms of an `if` expression where one arm panics and the other returns `i32`).
- Common diverging expressions: `panic!()`, `std::process::exit()`, `loop {}` (infinite loop without
  `break`).
- Common diverging **macros** (`todo!()`, `unimplemented!()`, `unreachable!()`) are covered in
  module 005 section 4.
- Useful for error-handling helpers that always abort.
*/

fn always_fails() -> ! {
    panic!("this function never returns");
}

fn diverging_functions() {
    // A diverging function can be used in a type context that
    // expects any type — the `!` type coerces to `i32` here
    let value: i32 = if true {
        42
    } else {
        always_fails() // returns !, which coerces to i32
    };
    println!("value: {value}");

    // std::process::exit() is another diverging function — we won't
    // call it here to keep the program running, but its signature is:
    // fn exit(code: i32) -> !

    println!("diverging_functions section executed");
}

pub fn run() {
    functions();
    if_expressions();
    match_expressions();
    infinite_loops();
    while_loops();
    for_loops_and_ranges();
    diverging_functions();
}
