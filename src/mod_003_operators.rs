// =================================================================================================
// Section 1: Arithmetic Operators
// =================================================================================================

/*
## Arithmetic Operators

- Rust provides five arithmetic operators: `+` (add), `-` (subtract), `*` (multiply), `/` (divide),
  `%` (remainder).
- `^` is **NOT** exponentiation — it is bitwise XOR. For exponentiation, use the `.pow()` method on
  integer types or `.powf()` / `.powi()` on floats.
- Integer division **truncates toward zero**: `7 / 2` yields `3`, not `3.5`. Note: this differs from
  "floor division" in languages like Python — `-7 / 2` yields `-3` in Rust (toward zero), not `-4`
  (toward negative infinity).
- **Remainder, not modulus**: `%` computes the **remainder** — the result keeps the sign of the
  **left** operand: `-7 % 3` gives `-1`, and `7 % -3` gives `1`. This differs from Python's `%`,
  which is a true modulus (result has the sign of the divisor). Float remainder also works: `5.5 %
  2.0` gives `1.5`.
- **Division by zero**: integer division or modulo by zero **panics** at runtime. Float division by
  zero produces `INFINITY` or `NAN` (covered in module 002).
- Both operands must be the **same type** — Rust does not implicitly promote between integer sizes
  or between integers and floats.
- These operators were briefly introduced alongside data types in module 002; this section provides
  the complete reference.
*/

fn arithmetic_operators() {
    let a = 20;
    let b = 7;

    println!("{a} + {b} = {}", a + b);
    println!("{a} - {b} = {}", a - b);
    println!("{a} * {b} = {}", a * b);
    println!("{a} / {b} = {}", a / b); // integer division truncates: 20 / 7 = 2
    println!("{a} % {b} = {}", a % b); // remainder: 20 % 7 = 6

    // Truncation toward zero
    println!("-7 / 2 = {} (toward zero, not -4)", -7_i32 / 2);

    // Remainder (%) keeps the sign of the LEFT operand
    println!("-7 % 3 = {} (not 2 like Python)", -7_i32 % 3); // -1
    println!(" 7 % -3 = {}", 7_i32 % -3); // 1
    println!("-7 % -3 = {}", (-7_i32) % -3); // -1

    // Float remainder
    println!("5.5 % 2.0 = {}", 5.5_f64 % 2.0); // 1.5

    // rem_euclid — Euclidean modulus (always non-negative)
    // `%` gives the remainder with the sign of the LEFT operand.
    // `rem_euclid()` always returns a **non-negative** result, which
    // matches the mathematical definition of modular arithmetic.
    // (For positive divisors this also matches Python's `%`; for
    // negative divisors it does not — Python's result takes the sign
    // of the divisor, whereas rem_euclid is always >= 0.)
    println!("(-7).rem_euclid(3) = {}", (-7_i32).rem_euclid(3)); // 2 (not -1)
    println!("  7.rem_euclid(-3) = {}", 7_i32.rem_euclid(-3)); // 1 (Python: -2)
    // Use rem_euclid when you need wrapping indices (e.g., circular arrays).

    // Exponentiation uses .pow(), not ^
    let base: i32 = 2;
    let power = base.pow(10);
    println!("2.pow(10) = {power}");

    // Float exponentiation
    let result = 2.0f64.powf(0.5);
    println!("2.0^0.5 = {result}");

    // Unary negation — the `-` operator applied to a single operand
    let positive = 42;
    let negative = -positive;
    println!("-{positive} = {negative}");

    // Float division by zero — produces INFINITY or NAN, not a panic
    let inf = 1.0_f64 / 0.0;
    let neg_inf = -1.0_f64 / 0.0;
    let nan = 0.0_f64 / 0.0;
    println!("1.0/0.0 = {inf}, -1.0/0.0 = {neg_inf}, 0.0/0.0 = {nan}");

    // Operands must be the same type — this would not compile:
    // let mixed = 5_i32 + 2.0_f64; // ERROR: cannot add `f64` to `i32`

    // --- Safe arithmetic methods ---
    // Standard +, -, * panic on overflow in debug mode. These methods
    // give you explicit control. They exist for all integer types
    // and all arithmetic operations (add, sub, mul, div, rem, etc.).

    // checked — returns Option: None on overflow
    println!("200u8 checked_add 100: {:?}", 200u8.checked_add(100)); // None
    println!("100u8 checked_add 100: {:?}", 100u8.checked_add(100)); // Some(200)

    // saturating — clamps at the min/max boundary
    println!("200u8 saturating_add 100: {}", 200u8.saturating_add(100)); // 255

    // wrapping — wraps around on overflow (same as release-mode behavior)
    println!("200u8 wrapping_add 100: {}", 200u8.wrapping_add(100)); // 44

    // overflowing — returns (result, did_overflow)
    println!(
        "200u8 overflowing_add 100: {:?}",
        200u8.overflowing_add(100)
    ); // (44, true)
}

// =================================================================================================
// Section 2: Comparison Operators
// =================================================================================================

/*
## Comparison Operators

- Six comparison operators: `==` (equal), `!=` (not equal), `<` (less than), `>` (greater than),
  `<=` (less or equal), `>=` (greater or equal).
- The return type is always `bool`.
- Both operands must be the **same type**.
- `==` is equality comparison — do not confuse with `=` (assignment).
- Comparison operators **cannot be chained**: `a < b < c` is not valid Rust. Use `a < b && b < c`
  instead.
- **Floating-point comparison pitfalls**: due to limited precision, `0.1 + 0.2 != 0.3`. Compare
  floats using an epsilon tolerance instead of `==`. Also, `NAN` never equals anything — even itself
  (covered in module 002).
*/

fn comparison_operators() {
    let a = 10;
    let b = 20;

    println!("{a} == {b}: {}", a == b);
    println!("{a} != {b}: {}", a != b);
    println!("{a} <  {b}: {}", a < b);
    println!("{a} >  {b}: {}", a > b);
    println!("{a} <= {b}: {}", a <= b);
    println!("{a} >= {b}: {}", a >= b);

    // Both operands must be the same type:
    // let _ = 10_i32 == 10_i64; // ERROR: mismatched types

    // Chaining is not allowed:
    // let _ = 1 < 2 < 3; // ERROR: cannot compare `bool` with `{integer}`
    // Instead, use: 1 < 2 && 2 < 3

    // Floating-point precision: 0.1 + 0.2 is not exactly 0.3
    let sum = 0.1_f64 + 0.2;
    println!("0.1 + 0.2 == 0.3: {}", sum == 0.3); // false
    println!("0.1 + 0.2 = {sum:.20}"); // shows imprecision
    // Use an epsilon for approximate comparison (works for values near 0–1;
    // for larger magnitudes a scaled tolerance is needed)
    println!("approx equal: {}", (sum - 0.3).abs() < f64::EPSILON); // true
}

// =================================================================================================
// Section 3: Logical Operators
// =================================================================================================

/*
## Logical Operators

- Three logical operators: `&&` (and), `||` (or), `!` (not).
- Operate on `bool` values **only** — no implicit conversion from integers or other types.
- **Short-circuit evaluation**: `&&` does not evaluate the right operand if the left is `false`;
  `||` does not evaluate the right if the left is `true`.
- When you need the boolean operation **without** short-circuit (e.g., both sides have side effects
  you want to execute), use the bitwise `&` / `|` / `^` operators on `bool` instead (see Section 5).
*/

fn logical_operators() {
    let x = true;
    let y = false;

    println!("true && false = {}", x && y);
    println!("true || false = {}", x || y);
    println!("!true = {}", !x);

    // Short-circuit: the right side is not evaluated if the left
    // side already determines the result
    // && stops early if left is false; || stops early if left is true
    let a = 5;
    let result = (a > 10) && (a < 3); // (a < 3) is never evaluated
    println!("(5 > 10) && (5 < 3) = {result}");

    // Side-effect demonstration: the function on the right is never called
    fn check() -> bool {
        println!("  check() was called");
        true
    }
    println!("short-circuit &&:");
    let _ = false && check(); // check() is NOT called
    println!("short-circuit ||:");
    let _ = true || check(); // check() is NOT called
    println!("no short-circuit:");
    let _ = true && check(); // check() IS called
}

// =================================================================================================
// Section 4: Compound Assignment Operators
// =================================================================================================

/*
## Compound Assignment Operators

- Combine an arithmetic or bitwise operation with assignment: `+=`, `-=`, `*=`, `/=`, `%=`.
- `x += 5` is equivalent to `x = x + 5`.
- The variable must be declared as `mut`.
- Bitwise compound assignments also exist: `&=`, `|=`, `^=`, `<<=`, `>>=`.
*/

fn compound_assignment_operators() {
    let mut x = 10;
    println!("initial x: {x}");

    x += 5; // x = x + 5
    println!("after += 5: {x}");

    x -= 3; // x = x - 3
    println!("after -= 3: {x}");

    x *= 2; // x = x * 2
    println!("after *= 2: {x}");

    x /= 4; // x = x / 4
    println!("after /= 4: {x}");

    x %= 3; // x = x % 3
    println!("after %= 3: {x}");

    // Bitwise compound assignments
    let mut bits: u8 = 0b1111_0000;
    bits &= 0b1010_1010; // AND assign
    println!("after &=: {:08b}", bits);

    bits |= 0b0000_1111; // OR assign
    println!("after |=: {:08b}", bits);

    bits ^= 0b1111_1111; // XOR assign
    println!("after ^=: {:08b}", bits);

    bits <<= 2; // left shift assign
    println!("after <<=2: {:08b}", bits);

    bits >>= 4; // right shift assign
    println!("after >>=4: {:08b}", bits);
}

// =================================================================================================
// Section 5: Bitwise Operators
// =================================================================================================

/*
## Bitwise Operators

- Operate on the **binary representation** of integer values.
- `&` (AND), `|` (OR), `^` (XOR), `!` (NOT).
- `<<` (left shift), `>>` (right shift).
- **Shift overflow**: shifting by an amount **>= the bit width** of the type is treated as overflow.
  - In **debug** builds it **panics**: `1u8 << 8` aborts with "attempt to shift left with overflow".
  - In **release** builds the **shift amount** (not the shifted value) is reduced **modulo the bit
    width**, so `1u8 << 8` becomes `1u8 << (8 % 8) == 1u8 << 0 == 1`.
  When you need wrapping or checked behavior unconditionally, use the explicit `wrapping_shl` /
  `wrapping_shr` (always wraps the shift amount) or `checked_shl` / `checked_shr` (returns `Option`,
  `None` on overflow) methods.
- Right shift on **signed** integers performs **arithmetic shift** — the sign bit is preserved
  (filled with 1s for negative values). Right shift on **unsigned** integers performs **logical
  shift** — filled with 0s.
- For integers, `!` performs bitwise NOT (flips every bit). This is the same `!` operator used for
  logical NOT on `bool` — Rust distinguishes the two by the operand type.
- Operand type rules:
  - The shift operators (`<<`, `>>`) require integer operands.
  - The other bitwise operators (`&`, `|`, `^`, `!`) work on any integer type **and on `bool`**.
    Using `&` / `|` / `^` on `bool` gives you the same truth-table result as `&&` / `||` but
    **without** short-circuit evaluation — handy when you want both operands evaluated for their
    side effects.
- Commonly used in systems programming, embedded code, flags, and performance-critical applications.
*/

fn bitwise_operators() {
    let a: u8 = 0b1100_1010;
    let b: u8 = 0b1010_1010;

    // AND — bit is 1 only if both bits are 1
    println!("{a:08b} & {b:08b} = {:08b}", a & b);

    // OR — bit is 1 if either bit is 1
    println!("{a:08b} | {b:08b} = {:08b}", a | b);

    // XOR — bit is 1 if bits differ
    println!("{a:08b} ^ {b:08b} = {:08b}", a ^ b);

    // NOT — flips every bit (unsigned: straightforward bit flip)
    println!("!{a:08b} = {:08b}", !a);

    // NOT on a signed integer — two's complement means !n == -(n+1)
    let signed: i8 = 5; // 00000101 in binary
    let flipped = !signed; // 11111010 = -6 in two's complement
    println!("!{signed} (i8) = {flipped} (two's complement: !n == -(n+1))");

    // Left shift — shifts bits left, filling with zeros
    let c: u8 = 0b0000_0001;
    println!("{c:08b} << 4 = {:08b}", c << 4);

    // Right shift — shifts bits right, filling with zeros (for unsigned)
    let d: u8 = 0b1000_0000;
    println!("{d:08b} >> 3 = {:08b}", d >> 3);

    // Arithmetic right shift on signed integers — preserves sign bit
    let signed: i32 = -8;
    println!(
        "{signed} >> 1 = {} (arithmetic shift, sign preserved)",
        signed >> 1
    );
    // -8 >> 1 = -4 (not a large positive number)

    // Contrast: unsigned right shift fills with 0s
    let unsigned: u8 = 0b1000_0000; // 128
    println!(
        "{unsigned} >> 1 = {} (logical shift, 0-fill)",
        unsigned >> 1
    ); // 64
    let neg: i8 = -128_i8; // 0b1000_0000 same bit pattern
    println!("{neg} >> 1 = {} (arithmetic shift, sign-fill)", neg >> 1); // -64

    // Shift overflow panics in debug mode; in release the shift amount
    // is reduced mod bit-width, so `1u8 << 8` becomes `1u8 << 0 == 1`.
    // Use wrapping_shl / checked_shl for explicit, build-independent behavior.
    // let _ = 1u8 << 8; // PANIC in debug: attempt to shift left with overflow

    // --- Practical bitwise example: permission flags ---
    const READ: u8 = 0b001;
    const WRITE: u8 = 0b010;
    const EXEC: u8 = 0b100;

    let mut perms: u8 = 0;
    perms |= READ; // grant read
    perms |= WRITE; // grant write
    println!("perms: {:03b} (read+write)", perms);
    println!("has read:  {}", perms & READ != 0); // true
    println!("has exec:  {}", perms & EXEC != 0); // false
    perms &= !WRITE; // revoke write
    println!("after revoke write: {:03b}", perms);

    // --- Bit inspection utility methods ---
    let val: u32 = 0b0010_1100;
    println!("value: {val:#010b}");
    println!("  count_ones:     {}", val.count_ones()); // 3
    println!("  count_zeros:    {}", val.count_zeros()); // 29
    println!("  leading_zeros:  {}", val.leading_zeros()); // 26
    println!("  trailing_zeros: {}", val.trailing_zeros()); // 2
    println!("  rotate_left(1): {:#010b}", val.rotate_left(1));
}

// =================================================================================================
// Section 6: Range Operators
// =================================================================================================

/*
## Range Operators

- Range operators produce range objects used in loops, slicing, and iterator adapters:
  - `a..b` — half-open range, from `a` (inclusive) to `b` (exclusive).
  - `a..=b` — closed range, from `a` to `b` (both inclusive).
  - `..b` — range up to `b` (exclusive), used for slicing.
  - `a..` — range from `a` onward (no upper bound).
  - `..` — full range (all elements).
- Each range syntax produces a distinct type from `std::ops`: `Range<T>`, `RangeInclusive<T>`,
  `RangeTo<T>`, `RangeFrom<T>`, and `RangeFull`. These types matter when you need ranges in type
  annotations or generic code.
- Half-open ranges (`a..b`) and closed ranges (`a..=b`) implement `Iterator` — they produce values
  you can loop over, collect, etc.
- Ranges are ascending only — `5..0` is empty. Use `.rev()` to descend (covered in module 004).
- The `as` keyword for type casting is also an operator; it was covered in module 002 section 10.
- **Operator overloading**: operators like `+`, `-`, `*`, `==` can be implemented for custom types
  via traits in `std::ops` (e.g., `Add`, `Sub`, `Mul`, `PartialEq`). Covered in module 008.
*/

fn range_operators() {
    // Half-open range: 0, 1, 2, 3, 4
    let half_open: Vec<i32> = (0..5).collect();
    println!("0..5:  {half_open:?}");

    // Closed range: 0, 1, 2, 3, 4, 5
    let closed: Vec<i32> = (0..=5).collect();
    println!("0..=5: {closed:?}");

    // Range in pattern matching
    let score = 85;
    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        _ => "other",
    };
    println!("score {score} -> grade {grade}");

    // Ranges are real types from std::ops — useful in type annotations
    let r: std::ops::Range<i32> = 0..5;
    println!("Range type: {r:?}");
    let ri: std::ops::RangeInclusive<i32> = 0..=5;
    println!("RangeInclusive type: {ri:?}");

    // Ranges for slicing
    let data = [10, 20, 30, 40, 50];
    println!("data[1..3]: {:?}", &data[1..3]); // [20, 30]
    println!("data[..2]:  {:?}", &data[..2]); // [10, 20]
    println!("data[3..]:  {:?}", &data[3..]); // [40, 50]

    // Descending: 5..0 is empty — use .rev()
    let descending: Vec<i32> = (0..5).rev().collect();
    println!("(0..5).rev(): {descending:?}"); // [4, 3, 2, 1, 0]

    // .contains() — idiomatic membership test
    // Replaces `x >= 1 && x <= 10` with a more readable form.
    let score = 85;
    println!("score in 1..=100: {}", (1..=100).contains(&score)); // true
    println!("score in 90..100: {}", (90..100).contains(&score)); // false (exclusive end)
}

// =================================================================================================
// Section 7: Operator Precedence and Associativity
// =================================================================================================

/*
## Operator Precedence and Associativity

- **Precedence** determines which operator is evaluated first in a compound expression. From highest
  to lowest (simplified):
  1. Unary: `!`, `-` (negation)
  2. Multiplicative: `*`, `/`, `%`
  3. Additive: `+`, `-`
  4. Shift: `<<`, `>>`
  5. Bitwise AND: `&`
  6. Bitwise XOR: `^`
  7. Bitwise OR: `|`
  8. Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
  9. Logical AND: `&&`
  10. Logical OR: `||`
  11. Range: `..`, `..=`
  12. Assignment: `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`,
      `^=`, `<<=`, `>>=`
- **Associativity**: most operators evaluate **left-to-right**. Assignment operators evaluate
  **right-to-left**.
- Rust does **not** allow chaining assignments: `x = y = 0` does not work because `y = 0` returns
  `()`, not an integer.
- When in doubt, use **parentheses** for clarity. Writing complex precedence-dependent expressions
  is discouraged.
- **Operator overloading**: operators like `+` may behave differently for different types (e.g., `+`
  adds numbers but concatenates `String`). This is implemented via traits, covered in a later
  module.
*/

fn precedence_and_associativity() {
    // Precedence: * binds tighter than +
    let result = 5 + 2 * 3; // 5 + (2 * 3) = 11
    println!("5 + 2 * 3 = {result}");

    // Left-to-right associativity for same-precedence operators
    let result = 16 / 4 * 2; // (16 / 4) * 2 = 8
    println!("16 / 4 * 2 = {result}");

    // Parentheses override precedence
    let result = (5 + 2) * 3; // 7 * 3 = 21
    println!("(5 + 2) * 3 = {result}");

    // Complex expression — step by step:
    // 1. a + b = 7        (additive)
    // 2. 7 - c = 2        (additive, left-to-right)
    // 3. 2 == 2 → true    (comparison)
    // 4. a == 3 → true    (comparison)
    // 5. true || true → true (logical OR)
    let a = 3;
    let b = 4;
    let c = 5;
    let result = a + b - c == 2 || a == 3;
    println!("3 + 4 - 5 == 2 || 3 == 3 = {result}");

    // --- Precedence note: bitwise operators vs comparison ---
    // Unlike C (where == binds tighter than &), Rust gives bitwise
    // operators (&, ^, |) higher precedence than comparison operators.
    // So `flags & mask == 0b0010` already parses as `(flags & mask) == 0b0010`.
    // Parentheses are still recommended for readability:
    let flags: u8 = 0b1010;
    let mask: u8 = 0b0010;
    let result = (flags & mask) == 0b0010; // parentheses for clarity
    println!("(0b1010 & 0b0010) == 0b0010: {result}");

    // --- Other operator-like symbols ---
    // `*` is also the dereference operator (unary, highest precedence):
    //   let x = &5; let y = *x; // dereference — see module 006
    // `&` and `&mut` are borrowing operators, not bitwise AND on bools:
    //   let r = &value;      // shared borrow
    //   let r = &mut value;  // mutable borrow
    // Context determines meaning: `a & b` is bitwise AND on integers;
    // `&a` is a borrow. See module 006 for full coverage.

    // --- No ternary operator ---
    // Rust has no `cond ? a : b` syntax. Use `if` as an expression:
    let flag = true;
    let x = if flag { 1 } else { 0 };
    println!("if-expression (no ternary): {x}");

    // Operator overloading: + concatenates Strings (trait-based,
    // covered in a later module)
    let greeting = String::from("Hello") + " world";
    println!("{greeting}");
}

pub fn run() {
    arithmetic_operators();
    comparison_operators();
    logical_operators();
    compound_assignment_operators();
    bitwise_operators();
    range_operators();
    precedence_and_associativity();
}
