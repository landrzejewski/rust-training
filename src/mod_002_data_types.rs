// =================================================================================================
// Section 1: Scalar and Compound Types
// =================================================================================================

/*
## Scalar and Compound Types

- Rust is **statically typed** — every value's type must be known at compile time.
- The compiler can infer types in most cases; an explicit annotation is needed only when the type is
  ambiguous.
- Built-in types fall into two categories:
  - **Scalar types** represent a single value: integers, floating-point numbers, booleans, and
    characters.
  - **Compound types** group multiple values together: tuples and arrays.
- All scalar types are stored on the **stack** and implement the `Copy` trait — values are copied on
  assignment, not moved.
- Tuples and arrays also implement `Copy` when all of their elements do. Since the scalar types are
  all `Copy`, compound types composed entirely of scalars are `Copy` too.
*/

// =================================================================================================
// Section 2: Integer Types
// =================================================================================================

/*
## Integer Types

- Rust provides signed and unsigned integers in five bit widths:

      Width     Signed    Unsigned
      8-bit     i8        u8
      16-bit    i16       u16
      32-bit    i32       u32
      64-bit    i64       u64
      128-bit   i128      u128

- The default integer type (when no annotation is given) is `i32`.
- `isize` and `usize` are pointer-sized integers — their width matches the target architecture (32
  or 64 bits). `usize` is used for indexing collections and measuring sizes. `isize` is rarely
  needed — mainly for pointer arithmetic and FFI (foreign function interfaces).
- Number literals support several formats:

      Format              Example
      Decimal             98_222
      Hexadecimal         0xff
      Octal               0o77
      Binary              0b1111_0000
      Byte (u8 only)      b'A'

- Underscores in numeric literals are visual separators ignored by the compiler: `1_000_000` is the
  same as `1000000`.
- A type suffix can be appended directly to a literal: `10u8`, `100_000i64`.
- **Integer overflow**: in debug mode the program panics; in release mode values wrap around using
  two's complement arithmetic. This release-mode wrapping is the default, not a guarantee — it can
  be toggled via `overflow-checks = true` in `Cargo.toml`. Relying on the profile-dependent default
  is an antipattern. When overflow is a meaningful part of the algorithm, use the explicit
  `wrapping_*` (always wraps), `checked_*` (returns `Option`), `saturating_*` (clamps to
  `MIN`/`MAX`), or `overflowing_*` (returns `(result, bool)`) methods that every integer type
  provides.
- Every integer type exposes `MIN` and `MAX` associated constants that return the smallest and
  largest representable value.
*/

fn integer_types() {
    // Type inferred as i32 (the default integer type)
    let a = 42;
    println!("a = {a} (inferred i32)");

    // Explicit type annotation
    let b: u8 = 255;
    println!("b = {b} (u8, maximum value)");

    // Number literal formats
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1010_0101;
    let byte = b'A'; // u8 value of the ASCII character 'A'
    println!("hex: {hex}, octal: {octal}, binary: {binary}, byte: {byte}");

    // Type suffix on a literal — the type is part of the literal itself
    let small = 10u8;
    let large = 100_000i64;
    println!("small: {small} (u8), large: {large} (i64)");

    // Underscores as visual separators in large numbers
    let million = 1_000_000;
    println!("one million: {million}");

    // MIN and MAX associated constants (representative subset)
    println!("i8  range: {} to {}", i8::MIN, i8::MAX);
    println!("u8  range: {} to {}", u8::MIN, u8::MAX);
    println!("i32 range: {} to {}", i32::MIN, i32::MAX);
    println!("u32 range: {} to {}", u32::MIN, u32::MAX);

    // usize — pointer-sized unsigned integer, used for indexing
    let index: usize = 0;
    let items = [10, 20, 30];
    println!("items[{index}] = {}", items[index]);

    // Integer overflow:
    // - With literal operands, the compiler catches overflow at
    //   *compile time* (constant folding), regardless of profile:
    //       let overflow: u8 = 255 + 1;
    //       // ERROR: attempt to compute `255_u8 + 1_u8`, which would overflow
    // - With a runtime value, debug builds *panic* and release
    //   builds *wrap* (two's complement) by default:
    //       let mut x: u8 = 255;
    //       x += 1; // debug: panic; release: wraps to 0
}

// =================================================================================================
// Section 3: Floating-Point Types
// =================================================================================================

/*
## Floating-Point Types

- Rust has two floating-point types conforming to the IEEE-754 standard: `f32` (single precision,
  32-bit) and `f64` (double precision, 64-bit).
- The default is `f64` — on modern CPUs double precision has negligible performance cost compared to
  single precision.
- Literal forms: `14.3`, `14.` (shorthand for `14.0`), `0.1f32` (type suffix), `2.99e8` (scientific
  notation).
- Standard arithmetic operators are supported: `+`, `-`, `*`, `/`, `%` (remainder).
- Integer and float types **cannot** be mixed in the same expression — an explicit conversion is
  required.
- Format specifiers for printing: `{:e}` for scientific notation, `{:.4e}` for controlling decimal
  precision.
- **Special values**: `f64::NAN` (Not a Number), `f64::INFINITY`, `f64::NEG_INFINITY`. Float
  division by zero produces `INFINITY` or `NEG_INFINITY` (not a panic). `0.0 / 0.0` produces `NAN`.
- `NAN` has surprising comparison behavior: `NAN != NAN` is `true`, and any comparison (`<`, `>`,
  `==`) with `NAN` returns `false`. Use `.is_nan()` to test for `NAN`.
- **Integer** division or modulo by zero **panics** at runtime.
*/

fn floating_point_types() {
    // Default inference is f64
    let a = 14.3;
    println!("a = {a} (inferred f64)");

    // Shorthand: a trailing dot means .0 (i.e. 14. is the same as 14.0)
    let b = 14.0;
    println!("b = {b} (14.0, same as 14.)");

    // Explicit f32 via type suffix
    let c = 0.1f32;
    println!("c = {c} (f32 via suffix)");

    // Basic arithmetic
    let sum = 5.0 + 10.5;
    let difference = 11.5 - 4.0;
    let product = 4.0 * 12.5;
    let quotient = 56.7 / 32.2;
    let remainder = 54.5 % 5.0;
    println!(
        "sum: {sum}, diff: {difference}, product: {product}, quotient: {quotient:.4}, remainder: {remainder}"
    );

    // Integer division truncates — float division does not
    let int_div = 2 / 3; // result: 0 (truncated toward zero)
    let float_div = 2.0 / 3.0; // result: 0.6666...
    println!("2 / 3 (integer): {int_div}, 2.0 / 3.0 (float): {float_div}");

    // Type inference from context — `other` is inferred as f32
    // because it is used in an expression with an f32 variable
    let explicit: f32 = 5.0;
    let other = 8.5; // inferred as f32 from the operation below
    let result = explicit + other;
    println!("f32 + f32 = {result}");

    // Integer and float types cannot be mixed:
    // let mixed = 5 + 2.0; // ERROR: cannot add a float to an integer

    // Scientific notation and format specifiers
    println!("Electron charge: {0}, {0:e}, {0:.4e}", -1.602_176_63e-19);
    println!("Speed of light:  {0}, {0:e}, {0:.4e}", 2.997_924_58e8);

    // Special float values
    let inf = 1.0_f64 / 0.0; // INFINITY (not a panic)
    let neg_inf = -1.0_f64 / 0.0; // NEG_INFINITY
    let nan = f64::NAN;
    println!("1.0/0.0 = {inf}, -1.0/0.0 = {neg_inf}, NAN = {nan}");

    // NAN comparisons are always false — even NAN == NAN
    #[allow(clippy::eq_op)]
    {
        println!("NAN == NAN: {}", nan == nan); // false
        println!("NAN != NAN: {}", nan != nan); // true
    }
    println!("use .is_nan(): {}", nan.is_nan()); // true

    // NAN is "contagious" — any arithmetic with NAN produces NAN
    println!("NAN + 5.0 = {}", f64::NAN + 5.0); // NAN
    println!("INFINITY + 1.0 = {}", f64::INFINITY + 1.0); // INFINITY
    println!("INFINITY * 0.0 = {}", f64::INFINITY * 0.0); // NAN

    // Integer division by zero panics at runtime:
    // let _ = 5 / 0; // PANIC: attempt to divide by zero
}

// =================================================================================================
// Section 4: Boolean Type
// =================================================================================================

/*
## Boolean Type

- Type `bool` has exactly two values: `true` and `false`.
- Size: 1 byte.
- Unlike C/C++, Rust has **no implicit conversion** between booleans and integers — `0` is not
  `false` and non-zero is not `true`.
- An explicit cast is possible: `true as i32` yields `1`, `false as i32` yields `0`.
- Logical operators `&&` (and), `||` (or), `!` (not) use **short-circuit evaluation** — the right
  operand is not evaluated if the result is already determined by the left operand.
*/

fn boolean_type() {
    // Inferred bool
    let is_active = true;
    println!("is_active: {is_active}");

    // Annotated bool
    let is_deleted: bool = false;
    println!("is_deleted: {is_deleted}");

    // Explicit cast to integer with `as`
    let active_as_int = is_active as i32;
    let deleted_as_int = is_deleted as i32;
    println!("true as i32: {active_as_int}, false as i32: {deleted_as_int}");

    // No implicit conversion from integers — this would not compile:
    // let flag: bool = 1; // ERROR: expected `bool`, found integer

    // Logical NOT
    let negated = !is_active;
    println!("!true = {negated}");

    // Size of bool: 1 byte
    println!("size of bool: {} byte", std::mem::size_of::<bool>());

    // Short-circuit evaluation — `&&` stops if the left side is false,
    // `||` stops if the left side is true
    let a = true;
    let b = false;
    let result = a && b; // b is evaluated, but if a were false, b would be skipped
    println!("true && false = {result}");
}

// =================================================================================================
// Section 5: Character Type
// =================================================================================================

/*
## Character Type

- Type `char` represents a single **Unicode Scalar Value**.
- Always **4 bytes** in size. Unicode Scalar Values need up to 21 bits, so Rust uses 4 bytes (32
  bits) for uniform alignment. This covers every code point except surrogates (U+D800–U+DFFF).
- Literals use **single quotes**: `'a'`, `'ℤ'`, `'😻'`.
- A `char` is not the same as a `u8`: `char` is 4 bytes and represents any Unicode Scalar Value,
  while `u8` is a single byte. In a UTF-8 encoded string, a character may span 1–4 bytes, so a byte
  in the middle of a multi-byte sequence is not a standalone character on its own.
- In UTF-8 encoded strings, characters occupy 1 to 4 bytes depending on the code point, while a
  `char` in memory is always 4 bytes.
- **`char` vs user-perceived character**: what a human sees as *one* letter does not always
  correspond to a single `char`. A *grapheme cluster* — e.g. the family emoji `👨‍👩‍👧`, `é` encoded
  as `e` + combining acute, or the flag `🇵🇱` — is built from multiple Unicode scalar values
  (multiple `char`s), even though it looks like a single glyph. The standard library does **not**
  provide grapheme-cluster iteration; for true "characters as a human reads them" use the
  `unicode-segmentation` crate (`s.graphemes(true)`). This matters whenever you measure the visible
  length of user-facing text — `s.chars().count()` is **not** the number of "letters" a reader
  perceives when the text contains emoji or composed characters.
- Useful methods: `is_alphabetic()`, `is_numeric()`, `is_ascii()`, `to_uppercase()`.
*/

fn character_type() {
    // ASCII letter
    let letter = 'a';
    println!("letter: {letter}");

    // Unicode character
    let math_symbol: char = 'ℤ';
    println!("math symbol: {math_symbol}");

    // Emoji character
    let cat = '😻';
    println!("emoji: {cat}");

    // A char is always 4 bytes in memory
    println!("size of char: {} bytes", std::mem::size_of::<char>());

    // Useful char methods
    let ch = 'a';
    println!("'{}' is_alphabetic: {}", ch, ch.is_alphabetic());
    println!("'{}' is_numeric: {}", ch, ch.is_numeric());
    println!("'{}' is_ascii: {}", ch, ch.is_ascii());
    // to_uppercase() returns an iterator, not a single char — some
    // Unicode characters map to multiple characters when uppercased
    // (e.g., German 'ß' → "SS"). The iterator implements Display,
    // so it prints naturally.
    println!("'{}' to_uppercase: {}", ch, ch.to_uppercase());
    let sharp_s = 'ß';
    let upper: String = sharp_s.to_uppercase().collect();
    println!(
        "'{}' to_uppercase: \"{}\" ({} chars)",
        sharp_s,
        upper,
        upper.chars().count()
    );

    // In a UTF-8 string, byte length can differ from character count.
    // The character 'Ł' occupies 2 bytes in UTF-8, so the byte length
    // of the string exceeds the number of characters.
    let name = "Łukasz";
    println!(
        "\"{name}\" is {} bytes but {} characters",
        name.len(),
        name.chars().count()
    );

    // A single user-perceived "character" may span multiple `char`s.
    // The family emoji 👨‍👩‍👧 is one glyph visually but several Unicode
    // scalar values joined by zero-width joiners (ZWJ, U+200D).
    let family = "👨\u{200D}👩\u{200D}👧";
    println!(
        "\"{family}\" looks like 1 character but is {} chars ({} bytes)",
        family.chars().count(),
        family.len()
    );
    // For true user-perceived "characters", use the `unicode-segmentation`
    // crate: `family.graphemes(true).count()` would return 1.
}

// =================================================================================================
// Section 6: Strings (&str and String)
// =================================================================================================

/*
## Strings (`&str` and `String`)

- Rust has two main string types: **string slices** (`&str`) and **owned strings** (`String`).
- A **string slice** (`&str`) is an immutable, borrowed view of UTF-8 encoded text. String literals
  (`"hello"`) have the type `&'static str` and are embedded directly in the program binary.
- A **`String`** is a heap-allocated, growable, owned UTF-8 string. It is created with
  `String::from("...")` or `.to_string()`.
- Both types store text as UTF-8 — individual characters may occupy 1 to 4 bytes.
- A `String` can be borrowed as `&str` through automatic coercion, making `&str` the preferred type
  for function parameters that only need to read text. A function taking `&str` accepts both `&str`
  and `&String` thanks to this coercion.
- Strings **cannot** be indexed by byte position (`s[0]` does not compile) because a byte index may
  not align with a character boundary. Use `.chars().nth(n)` for character access, or `.get(range)`
  for safe byte-range slicing (returns `Option<&str>`).
- **Range slicing with `&s[i..j]` compiles but can panic at runtime.** Unlike single-byte indexing
  `s[0]` (which doesn't compile), byte-range slicing `&hello[0..1]` **does** compile. At runtime it
  checks whether both endpoints land on UTF-8 character boundaries; if not, it panics with a message
  like `byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2)`. For text that may
  contain multi-byte characters, prefer the non-panicking `.get(range)` method (which returns
  `Option<&str>`) or compute boundaries with `.char_indices()`.
- **String mutation**: owned `String` values support mutation:
  - `.push('c')` appends a single character.
  - `.push_str("text")` appends a string slice.
  - `+` operator concatenates: `String + &str` (moves the left operand).
  - `format!("{a}{b}")` concatenates without moving any values.
- **`.bytes()`** returns an iterator over the raw bytes of a string, useful for understanding UTF-8
  encoding alongside `.chars()`.
- **Byte strings** `b"hello"` have type `&[u8; N]` — a reference to a fixed-size byte array. They
  represent raw ASCII/byte data rather than UTF-8 text. Useful for binary protocols, file headers,
  etc.
*/

// Note: This section is longer than most because strings are
// fundamental to nearly every Rust program. Skim on first read
// and return as needed.

fn strings() {
    // String literal — has type &'static str, stored in the binary
    let greeting: &str = "Hello, world!";
    println!("literal: {greeting}");

    // Creating an owned String with String::from()
    let owned = String::from("Hello");
    println!("owned: {owned}");

    // Creating an owned String with .to_string()
    let also_owned = "Hello".to_string();
    println!("also owned: {also_owned}");

    // Borrowing a String as &str — automatic coercion
    let borrowed: &str = &owned;
    println!("borrowed from owned String: {borrowed}");

    let text = "Łukasz";

    // Accessing a character by index — .chars().nth(n) returns Option<char>
    let third = text.chars().nth(2);
    println!("third character: {:?}", third);

    // Direct indexing does not compile:
    // let byte = text[0]; // ERROR: `str` cannot be indexed by `usize`

    // Safe byte-range slicing with .get() — returns Option<&str>
    let safe = text.get(0..2); // 'Ł' is 2 bytes in UTF-8
    println!("safe get(0..2): {:?}", safe); // Some("Ł")
    let invalid = text.get(0..1); // 1 byte is not a valid char boundary
    println!("safe get(0..1): {:?}", invalid); // None

    // Iterating over characters with .chars()
    print!("characters in \"{text}\": ");
    for ch in text.chars() {
        print!("{ch} ");
    }
    println!();

    // Useful read-only string methods
    let sentence = "  Hello, Rust!  ";
    println!("len: {}", sentence.len()); // byte length
    println!("is_empty: {}", sentence.is_empty()); // false
    println!("contains 'Rust': {}", sentence.contains("Rust"));
    println!("starts_with '  H': {}", sentence.starts_with("  H"));
    println!("trimmed: \"{}\"", sentence.trim()); // removes leading/trailing whitespace
    let words: Vec<&str> = "one,two,three".split(',').collect();
    println!("split by comma: {words:?}");

    // Case-insensitive comparison
    println!(
        "eq_ignore_ascii_case: {}",
        "Hello".eq_ignore_ascii_case("hello")
    ); // true
    // For full Unicode case folding, compare lowercased versions:
    let a = "Straße";
    let b = "STRASSE";
    println!(
        "{a} == {b} (lowercase): {}",
        a.to_lowercase() == b.to_lowercase()
    );

    // Method chaining — transformations compose naturally
    let cleaned = "  hello world  ".trim().to_uppercase();
    println!("chained trim + uppercase: \"{cleaned}\"");

    // --- String::new() and capacity ---
    let mut empty = String::new(); // empty String, no allocation yet
    empty.push_str("hello");
    println!(
        "String::new() + push: \"{empty}\" (capacity: {})",
        empty.capacity()
    );
    // String pre-allocates and doubles capacity on reallocation,
    // similar to Vec.

    // --- String mutation ---
    let mut s = String::from("hello");

    // .push() — append a single character
    s.push(',');
    println!("after push: {s}");

    // .push_str() — append a string slice
    s.push_str(" world");
    println!("after push_str: {s}");

    // + operator — concatenates String + &str (moves the left operand)
    let greeting = String::from("hello");
    let target = String::from(" Rust");
    let combined = greeting + &target; // greeting is moved, target is borrowed
    println!("concatenated with +: {combined}");
    // println!("{greeting}"); // ERROR: greeting was moved
    // The right side must be &str, not String:
    // let a = String::from("x");
    // let b = String::from("y");
    // let c = a + b; // ERROR: expected `&str`, found `String`
    // Fix: use `a + &b` or `format!("{a}{b}")`

    // format! — concatenates without moving any values
    let first = String::from("hello");
    let second = String::from("world");
    let formatted = format!("{first}, {second}!");
    println!("format!: {formatted}");
    println!("first still valid: {first}"); // both originals still valid

    // --- .bytes() iterator — raw UTF-8 bytes ---
    let name = "Łukasz";
    print!("bytes of \"{name}\": ");
    for byte in name.bytes() {
        print!("{byte:02x} ");
    }
    println!("({} bytes, {} chars)", name.len(), name.chars().count());

    // Byte string literal — &[u8; N], raw bytes, not UTF-8 text
    let bytes: &[u8; 5] = b"hello";
    println!("byte string: {bytes:?}");
    println!("first byte: {} (ASCII 'h')", bytes[0]);

    // --- .to_owned() vs .to_string() vs String::from() ---
    // All three produce a String from a &str — choose by intent:
    let s1 = String::from("literal"); // most explicit, idiomatic for literals
    let slice: &str = "variable";
    let s2 = slice.to_owned(); // via ToOwned trait — general-purpose
    let s3 = slice.to_string(); // via Display trait — same result for &str
    println!("{s1}, {s2}, {s3} (all equivalent for &str)");
    // Convention: String::from() for string literals, .to_owned() or
    // .to_string() for &str variables. Performance is identical.
}

// =================================================================================================
// Section 7: Tuples
// =================================================================================================

/*
## Tuples

- A tuple groups a **fixed number** of values that may have **different types**.
- Declared with parentheses: `let t: (i32, f64, bool) = (1, 2.0, true);`.
- The length is fixed at compile time — a tuple cannot grow or shrink after declaration.
- Access individual elements with **dot notation**: `tuple.0`, `tuple.1`, etc.
- **Destructuring** binds each element to a separate variable: `let (a, b, c) = tuple;`. The number
  of variables must match the tuple length.
- Use `_` to ignore specific elements during destructuring.
- A **single-element tuple** requires a trailing comma: `(42,)`. Without the comma, `(42)` is just a
  parenthesized expression, not a tuple.
- The **unit type** `()` is an empty tuple. It represents "no meaningful value" and is the implicit
  return type of functions and expressions that do not return anything.
- Tuples are **stack-allocated**.
*/

fn tuples() {
    // Tuple with mixed types
    let info = ("Rust", 2015, true);
    println!(
        "language: {}, year: {}, is great: {}",
        info.0, info.1, info.2
    );

    // Explicit type annotation
    let point: (f64, f64) = (3.5, -2.1);
    println!("point: ({}, {})", point.0, point.1);

    // Destructuring — number of variables must match tuple length
    let (name, year, _flag) = info;
    println!("name: {name}, year: {year}");

    // Ignoring elements with `_`
    let (_, _, is_great) = info;
    println!("is great: {is_great}");

    // Tuple destructuring in match (pattern matching covered in m004)
    let coordinates = (0, 0);
    match coordinates {
        (0, 0) => println!("origin"),
        (x, 0) => println!("on x-axis at {x}"),
        (0, y) => println!("on y-axis at {y}"),
        (x, y) => println!("at ({x}, {y})"),
    }

    // Single-element tuple — trailing comma is required
    let single = (42,); // this is a tuple: (i32,)
    #[allow(unused_parens)]
    let not_tuple = (42); // this is just an integer: i32
    println!("single-element tuple: {single:?}, plain int: {not_tuple}");

    // The unit type — an empty tuple representing "no value"
    let unit: () = ();
    println!("unit value: {unit:?}");
}

// =================================================================================================
// Section 8: Arrays and Slices
// =================================================================================================

/*
## Arrays and Slices

- An array groups a **fixed number** of values of the **same type**.
- Declared as `[T; N]` where `T` is the element type and `N` is the length (a compile-time
  constant): `let arr: [i32; 3] = [1, 2, 3];`.
- Arrays are **stack-allocated**.
- Fill syntax creates an array of a given size with every element set to the same value: `[0; 5]`
  produces `[0, 0, 0, 0, 0]`.
- Elements are accessed by index with `[]`. Out-of-bounds access **panics** at runtime.
- Multidimensional arrays are arrays of arrays: `[[1, 2], [3, 4]]` has type `[[i32; 2]; 2]`.
- A **slice** (`&[T]`) is a borrowed view into a contiguous sequence of elements. It does not own
  the data.
- Slices are created with range syntax: `&arr[1..3]` (exclusive end), `&arr[1..=3]` (inclusive end),
  `&arr[..3]` (from start), `&arr[2..]` (to end), `&arr[..]` (full).
- A slice is a **fat pointer**: it stores both the starting address and the length.
- **Mutable slices** (`&mut [T]`) allow modifying the elements of the underlying array or Vec
  through a borrowed view.
- **Array vs `Vec`**: use arrays when the size is known at compile time and will not change (e.g.,
  RGB values, matrix dimensions). Use `Vec<T>` (covered in the collections module) when the size is
  determined at runtime or the collection needs to grow/shrink. Arrays are stack-allocated; Vecs are
  heap-allocated.
*/

fn arrays_and_slices() {
    // Array with explicit type annotation
    let mut numbers: [i32; 5] = [10, 20, 30, 40, 50];
    println!("original: {numbers:?}");

    // Mutation of an element
    numbers[0] = -1;
    println!("after mutation: {numbers:?}");

    // Fill syntax — create an array of 7 zeros
    let zeros = [0; 7];
    println!("zeros: {zeros:?}");

    // Multidimensional array — access elements with chained indexing
    let matrix: [[i32; 2]; 2] = [[1, 2], [3, 4]];
    println!("matrix: {matrix:?}");
    println!("matrix[0][1] = {}", matrix[0][1]); // row 0, col 1 → 2
    println!("matrix[1][0] = {}", matrix[1][0]); // row 1, col 0 → 3

    // Indexing — out-of-bounds would panic at runtime:
    // let _ = numbers[10]; // ERROR: index out of bounds (panic)

    // Slices — borrowed views into an array
    let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let exclusive = &data[2..5]; // elements at indices 2, 3, 4
    let inclusive = &data[2..=5]; // elements at indices 2, 3, 4, 5
    let from_start = &data[..4]; // elements at indices 0, 1, 2, 3
    let to_end = &data[7..]; // elements at indices 7, 8, 9
    let full = &data[..]; // all elements

    println!("exclusive [2..5]:  {exclusive:?}");
    println!("inclusive [2..=5]: {inclusive:?}");
    println!("from start [..4]: {from_start:?}");
    println!("to end [7..]:     {to_end:?}");
    println!("full [..]:        {full:?}");

    // Useful array and slice methods
    println!("len: {}", data.len());
    println!("is_empty: {}", data.is_empty());
    println!("contains 5: {}", data.contains(&5));
    println!("first: {:?}, last: {:?}", data.first(), data.last());

    // .get(index) — safe indexing, returns Option<&T>
    println!("get(5): {:?}", data.get(5)); // Some(&5)
    println!("get(100): {:?}", data.get(100)); // None (no panic)
    // Same for range: data.get(2..5) returns Option<&[i32]>

    // Mutable slices — modify elements through a borrowed view
    let mut values = [10, 20, 30, 40, 50];
    let slice = &mut values[1..4]; // mutable slice of elements 1, 2, 3
    slice[0] = 99;
    println!("after mutable slice: {values:?}");

    // Slice patterns — destructure arrays/slices with pattern matching
    let rgb = [255, 128, 0];
    let [r, g, b] = rgb;
    println!("r={r}, g={g}, b={b}");

    // Iterating with .iter()
    let sum: i32 = data.iter().sum();
    println!("sum via .iter().sum(): {sum}");
}

// =================================================================================================
// Section 9: Type Aliasing
// =================================================================================================

/*
## Type Aliasing

- The `type` keyword creates a new name (**alias**) for an existing type.
- The alias is **fully interchangeable** with the original type — it is not a new distinct type. The
  compiler treats `Meters` and `f64` as identical, so a function accepting `Meters` will also accept
  any `f64` without complaint.
- If you need a **distinct type** that the compiler can tell apart (preventing accidental mixing of
  meters and seconds, for example), use the **newtype pattern**: `struct Meters(f64);`. Newtypes are
  covered in the structs module.
- Useful for shortening long or complex type signatures and improving readability.
- Naming convention: `PascalCase`.
*/

fn type_aliasing() {
    // Define a type alias for f64
    type Meters = f64;

    // Use the alias in a variable declaration
    let distance: Meters = 100.0;
    println!("distance: {distance} meters");

    // The alias and original type are fully interchangeable
    let raw: f64 = 50.0;
    let total: Meters = distance + raw; // mixing Meters and f64 is valid
    println!("total: {total} meters");

    // --- Type aliases with generics ---
    // A common pattern: shorten Result types for a specific error.
    // std::io::Result<T> is defined as:
    //   type Result<T> = std::result::Result<T, std::io::Error>;
    type AppResult<T> = Result<T, String>;
    let ok: AppResult<i32> = Ok(42);
    let err: AppResult<i32> = Err("oops".into());
    println!("AppResult ok: {ok:?}, err: {err:?}");
}

// =================================================================================================
// Section 10: Type Conversion
// =================================================================================================

/*
## Type Conversion

- Rust requires **explicit** type conversion between numeric types — there is no implicit coercion.
- The `as` keyword performs primitive type casting: `let y = x as f64;`.
- **Integer → integer**: widening casts (e.g., `u8` → `i32`) are safe. Narrowing casts (e.g., `i32`
  → `u8`) keep only the **low-order bits** of the source — equivalent to wrapping modulo `2^N`,
  *not* clamping. For example, `300i32 as u8` yields `44` (the low 8 bits of 300), not `255`.
- **Float → integer**: the fractional part is truncated toward zero. Values outside the target range
  saturate to `MIN` or `MAX`.
- **Integer → float**: may lose precision for large values. `f32` has a 24-bit significand (~7
  decimal digits), `f64` has 53 bits (~15 digits). Any integer exceeding the significand width will
  be rounded to the nearest representable float.
- **Bool → integer**: `true as i32` yields `1`, `false as i32` yields `0`. The reverse direction is
  not allowed.
- `as` is for **primitive types only**. For complex conversions, use the `From`/`Into` traits
  (covered in a later module).
*/

fn type_conversion() {
    // Integer to float — widening, safe
    let integer = 42i32;
    let float = integer as f64;
    println!("{integer} as f64 = {float}");

    // Float to integer — truncates toward zero
    let pi = 3.99;
    let truncated = pi as i32;
    println!("{pi} as i32 = {truncated}");

    // Narrowing integer cast — value is truncated (wraps)
    let large = 300i32;
    let small = large as u8; // 300 % 256 = 44
    println!("{large} as u8 = {small}");

    // Widening integer cast — always safe
    let byte: u8 = 200;
    let wider = byte as i32;
    println!("{byte} as i32 = {wider}");

    // Boolean to integer
    let flag = true;
    let flag_int = flag as i32;
    println!("true as i32 = {flag_int}");

    // Negative integer to unsigned — wraps via two's complement
    let negative = -1i32;
    let as_unsigned = negative as u32; // wraps to u32::MAX (4294967295)
    println!("{negative} as u32 = {as_unsigned} (two's complement wrap)");

    // Large integer to f32 — precision loss
    let big = 16_777_217i32; // exceeds f32 precision (~7 decimal digits)
    let approx = big as f32;
    println!("{big} as f32 = {approx} (precision lost)");

    // Float to integer — out-of-range values saturate to MIN or MAX
    let huge = 1_000.0f64;
    let saturated = huge as i8; // 1000 exceeds i8::MAX (127), saturates to 127
    println!("{huge} as i8 = {saturated} (saturated to i8::MAX)");

    // `as` is for primitive types only — for String, Vec, etc.
    // use From/Into traits (covered in a later module)
}

pub fn run() {
    integer_types();
    floating_point_types();
    boolean_type();
    character_type();
    strings();
    tuples();
    arrays_and_slices();
    type_aliasing();
    type_conversion();
}
