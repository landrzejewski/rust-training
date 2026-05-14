use fmt::Display;
use std::any::type_name;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, Neg};

// =================================================================================================
// Section 1: Generic Functions
// =================================================================================================

/*
## Generic Functions

- Generics eliminate code duplication while preserving type safety. Without generics, you would need
  a separate function for every type: `i32_as_string`, `f64_as_string`, etc. A single generic
  function replaces them all.
- Syntax: `fn name<T>(param: T) -> ReturnType`. The angle brackets `<T>` introduce a **type
  parameter** — a placeholder for a concrete type that is filled in at each call site.
- Type parameter names are conventionally single uppercase letters (`T`, `U`, `A`, `B`), but any
  valid identifier works. `T` stands for "type" by convention.
- A bare type parameter `T` (without bounds) tells the compiler nothing about what `T` can do — you
  cannot print it, compare it, or perform arithmetic on it. To use operations on `T`, you need
  **trait bounds** (covered in section 6).
- **Monomorphization**: the compiler generates a separate, specialized copy of the function for each
  concrete type used at a call site. `as_string(42)` and `as_string(true)` produce two distinct
  functions in the compiled binary. This makes generics a **zero-cost abstraction** — no runtime
  overhead compared to hand-written specialized functions.
- `std::any::type_name::<T>()` returns the compiler's name for `T` as a `&str`. Useful for
  demonstration; not for production logic.
*/

// Concrete functions — one per type, repetitive
fn i32_as_string(value: i32) -> String {
    format!("{value}:i32")
}

fn f64_as_string(value: f64) -> String {
    format!("{value}:f64")
}

// Generic replacement — one function handles all Display types
fn as_string<T: Display>(value: T) -> String {
    format!("{value}:{}", type_name::<T>())
}

// Multiple type parameters — A and B can be different types
fn make_pair<A, B>(a: A, b: B) -> (A, B) {
    (a, b)
}

fn generic_functions() {
    // The concrete versions work, but require duplication
    println!("{}", i32_as_string(42));
    println!("{}", f64_as_string(3.14));

    // The generic version handles all types that implement Display
    println!("{}", as_string(42)); // T = i32
    println!("{}", as_string(true)); // T = bool
    println!("{}", as_string(3.14_f64)); // T = f64
    println!("{}", as_string("hello")); // T = &str

    // Multiple type parameters — each inferred independently
    let pair_a = make_pair(1, "hello");
    println!("pair_a: ({}, {})", pair_a.0, pair_a.1);

    let pair_b = make_pair(true, 3.14);
    println!("pair_b: ({}, {})", pair_b.0, pair_b.1);

    // Turbofish is REQUIRED when the compiler cannot infer T.
    // Here, collect() could return any collection type — the compiler
    // has no way to know which one without an explicit annotation.
    let collected = (0..5).collect::<Vec<i32>>();
    // Without turbofish: let collected = (0..5).collect(); // ERROR: type annotations needed
    println!("turbofish required: {collected:?}");

    println!("generic_functions section executed");
}

// =================================================================================================
// Section 2: Generic Structs and Enums
// =================================================================================================

/*
## Generic Structs and Enums

- Structs can be parameterized over one or more types: `struct Pair<T> { first: T, second: T }`.
- A single type parameter constrains all uses to the same type: `Pair { first: 1, second: 2 }` works
  (both `i32`), but `Pair { first: 1, second: "two" }` does not compile because `T` cannot be both
  `i32` and `&str`.
- Multiple type parameters allow different types per field: `struct Coordinate<T, U> { x: T, y: U }`
  lets you write `Coordinate { x: 5, y: 10.0 }`.
- Enums can also be generic. `Option<T>` and `Result<T, E>` from the standard library (covered in
  module 007) are themselves generic enums — defined with the same mechanism shown here.
- Generic types follow the same monomorphization model as generic functions: `Pair<i32>` and
  `Pair<f64>` become two distinct types in the compiled binary.
*/

struct Pair<T> {
    first: T,
    second: T,
}

struct Coordinate<T, U> {
    x: T,
    y: U,
}

#[allow(dead_code)]
enum Outcome<S, E> {
    Success(S),
    Failure(E),
}

fn generic_structs_and_enums() {
    // Single type parameter — both fields must be the same type
    let int_pair = Pair {
        first: 1,
        second: 2,
    };
    println!("int pair: ({}, {})", int_pair.first, int_pair.second);

    let str_pair = Pair {
        first: "hello",
        second: "world",
    };
    println!("str pair: ({}, {})", str_pair.first, str_pair.second);

    // ERROR: T cannot be both i32 and &str
    // let bad = Pair { first: 1, second: "two" };

    // Two type parameters — each field can differ
    let coord_a = Coordinate { x: 5, y: 10.0 };
    println!("coord_a: ({}, {})", coord_a.x, coord_a.y);

    let coord_b = Coordinate { x: 1.5, y: 2.5 };
    println!("coord_b: ({}, {})", coord_b.x, coord_b.y);

    // Generic enum — same mechanism behind Option<T> and Result<T,E>
    let ok: Outcome<i32, String> = Outcome::Success(42);
    let err: Outcome<i32, String> = Outcome::Failure(String::from("something failed"));

    match ok {
        Outcome::Success(v) => println!("outcome ok: {v}"),
        Outcome::Failure(e) => println!("outcome err: {e}"),
    }
    match err {
        Outcome::Success(v) => println!("outcome ok: {v}"),
        Outcome::Failure(e) => println!("outcome err: {e}"),
    }

    println!("generic_structs_and_enums section executed");
}

// =================================================================================================
// Section 3: Generic impl Blocks
// =================================================================================================

/*
## Generic impl Blocks

- To implement methods on a generic struct, declare the type parameter on the `impl` keyword:
  `impl<T> Pair<T> { ... }`. The `<T>` after `impl` introduces the type parameter for the entire
  block.
- Methods can add **additional bounds** beyond what the struct requires: `impl<T: Display> Pair<T> {
  fn show(&self) { ... } }` makes `show()` available only when `T` implements `Display`. Other
  methods in a less-bounded impl block remain available for all `T`.
- **Specialized impl blocks** target a specific concrete type: `impl Coordinate<i32, i32> { ... }`.
  No `<T>` after `impl` — these methods exist only for that exact type combination.
- You can freely mix generic and specialized impl blocks on the same struct. The compiler selects
  the applicable impl based on the concrete types at the call site.
- **Method-level generic parameters**. Methods inside a generic `impl` block can declare their
  **own** type parameters, separate from those on the struct. For example, given `struct Point<X1,
  Y1>`, you can write `impl<X1, Y1> Point<X1, Y1> { fn mixup<X2, Y2>(self, other: Point<X2, Y2>) ->
  Point<X1, Y2> { ... } }` — the method accepts a `Point` with entirely different type parameters
  and returns yet another combination. Parameters declared on `impl` belong to the struct type;
  parameters declared on `fn` belong to the individual method call. This is the same mechanism
  `ArrayProcessor::update_data<'b>` uses for lifetimes in module 006 — just applied to type
  parameters.
- Basic `impl` block syntax (`&self`, `&mut self`, `Self`, associated functions) is covered in
  module 007 section 3.
*/

// Generic impl — available for all Pair<T>
impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}

// Bounded generic impl — only available when T: Display

impl<T: Display> Pair<T> {
    fn show(&self) {
        println!("({}, {})", self.first, self.second);
    }
}

// Specialized impl — only for Coordinate<i32, i32>
impl Coordinate<i32, i32> {
    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

// Specialized impl — only for Coordinate<f64, f64>
impl Coordinate<f64, f64> {
    fn euclidean_distance(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn generic_impl_blocks() {
    // new() is available for any T
    let p = Pair::new(10, 20);

    // show() works because i32 implements Display
    p.show();

    // manhattan_distance() only exists for Coordinate<i32, i32>
    let c_int = Coordinate { x: -3, y: 4 };
    println!("manhattan distance: {}", c_int.manhattan_distance());

    // euclidean_distance() only exists for Coordinate<f64, f64>
    let c_float = Coordinate { x: 3.0, y: 4.0 };
    println!("euclidean distance: {}", c_float.euclidean_distance());

    // A mixed Coordinate<i32, f64> has neither specialized method:
    let _c_mixed = Coordinate { x: 3, y: 4.0 };
    // _c_mixed.manhattan_distance(); // ERROR: method not found
    // _c_mixed.euclidean_distance(); // ERROR: method not found

    println!("generic_impl_blocks section executed");
}

// =================================================================================================
// Section 4: Trait Definitions and Implementations
// =================================================================================================

/*
## Trait Definitions and Implementations

- A **trait** defines a set of methods that a type can implement. Similar to interfaces in other
  languages, but more powerful: traits can provide default method bodies and participate in generic
  bounds.
- Syntax: `trait Name { fn method(&self) -> ReturnType; }`.
- Implementing a trait for a type: `impl TraitName for Type { fn method(&self) -> ReturnType { ... }
  }`.
- **Required methods** have no body in the trait — every implementor must provide one.
- **Default methods** have a body in the trait. Implementors can override the default or inherit it.
  Default methods can call other methods in the same trait, including required ones.
- **The orphan rule**: you can implement a trait for a type only if either the trait or the type (or
  both) is defined in your crate. Without this rule, two crates could each implement the same trait
  for the same type differently, and the compiler would have no way to choose between them. This
  guarantees **coherence** — every trait-type combination has at most one implementation.
- **Fully qualified syntax**: when a type implements multiple traits that define methods with the
  same name, calling `value.method()` is ambiguous. Use `<Type as Trait>::method(&value)` to
  disambiguate.
- **Inherent methods win over trait methods.** When a type has an *inherent* method (defined in a
  plain `impl Type { … }` block rather than through a trait), dot-syntax calls resolve to the
  inherent method and `value.method()` is **not** ambiguous even if traits with the same name are in
  scope. The trait versions remain reachable through `<Type as Trait>::method(&value)` — so here FQS
  is an *option*, not a requirement.
- **Associated functions without `self` are stricter.** With no receiver, the compiler has no way to
  infer which implementation to pick. If several `impl`s define the same associated function name,
  FQS `<Type as Trait>::func()` is **required** for the trait version; calling `Trait::func()` alone
  fails with E0790.
- Note: basic `impl` blocks without traits (attaching methods directly to a type) are covered in
  module 007 section 3.
*/

trait Describable {
    // Required — every implementor must provide this
    fn describe(&self) -> String;

    // Default method — can be overridden, calls the required method
    fn label(&self) -> String {
        format!("Item: {}", self.describe())
    }
}

#[derive(Clone)]
struct Product {
    name: String,
    price: f64,
}

impl Describable for Product {
    fn describe(&self) -> String {
        format!("{} (${:.2})", self.name, self.price)
    }
    // label() uses the default implementation
}

struct Service {
    name: String,
    hourly_rate: f64,
}

impl Describable for Service {
    fn describe(&self) -> String {
        format!("{} (${:.2}/hr)", self.name, self.hourly_rate)
    }

    // Overrides the default — provides a custom label
    fn label(&self) -> String {
        format!("Service: {}", self.describe())
    }
}

fn trait_definitions_and_implementations() {
    let p = Product {
        name: String::from("Widget"),
        price: 9.99,
    };
    let s = Service {
        name: String::from("Consulting"),
        hourly_rate: 150.0,
    };

    // Product uses the default label()
    println!("{}", p.label()); // "Item: Widget ($9.99)"

    // Service overrides label()
    println!("{}", s.label()); // "Service: Consulting ($150.00/hr)"

    // describe() is always available directly
    println!("{}", p.describe());
    println!("{}", s.describe());

    // --- Fully qualified method syntax ---
    // When a type implements multiple traits with the same method name,
    // use <Type as Trait>::method() to disambiguate.
    trait Greeter {
        fn name(&self) -> &str;
    }
    trait Logger {
        fn name(&self) -> &str;
    }

    struct Bot;
    impl Greeter for Bot {
        fn name(&self) -> &str {
            "GreeterBot"
        }
    }
    impl Logger for Bot {
        fn name(&self) -> &str {
            "LoggerBot"
        }
    }

    // Inherent method on Bot — wins over trait methods for `bot.name()`.
    impl Bot {
        fn name(&self) -> &str {
            "direct Bot"
        }
    }

    let bot = Bot;
    // bot.name() is NOT ambiguous: the inherent method takes priority
    // over trait methods of the same name.
    println!("inherent: {}", bot.name()); // "direct Bot"
    // The trait methods remain reachable via fully qualified syntax:
    println!("as Greeter: {}", <Bot as Greeter>::name(&bot));
    println!("as Logger:  {}", <Bot as Logger>::name(&bot));

    // --- Fully qualified syntax for associated functions without self ---
    // Without a `self` receiver, the compiler has no way to infer which
    // implementation to pick — FQS is REQUIRED for the trait version.
    trait Breed {
        fn kind() -> &'static str;
    }
    struct Cat;
    impl Cat {
        // Inherent associated fn — wins for `Cat::kind()`.
        fn kind() -> &'static str {
            "inherent cat"
        }
    }
    impl Breed for Cat {
        fn kind() -> &'static str {
            "mammal"
        }
    }

    // `Cat::kind()` picks the inherent one:
    println!("Cat::kind()            = {}", Cat::kind());
    // `Breed::kind()` alone fails — no receiver, nothing to infer from:
    //   Breed::kind();
    //   ERROR[E0790]: cannot call associated function on trait without
    //                 specifying the corresponding `impl` type
    // Fully qualified syntax is REQUIRED:
    println!("<Cat as Breed>::kind() = {}", <Cat as Breed>::kind());

    // --- Orphan rule newtype workaround ---
    // The orphan rule prevents: impl fmt::Display for Vec<String> { ... }
    // Workaround: wrap the foreign type in a local newtype.
    struct PrintableVec(Vec<String>);

    impl Display for PrintableVec {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    let items = PrintableVec(vec!["one".into(), "two".into()]);
    println!("newtype Display: {items}");
    // Access inner Vec: items.0.len(), or implement Deref (section 5).

    // --- Extension traits ---
    // Add methods to existing types without wrapping them.
    trait StrExt {
        fn is_blank(&self) -> bool;
    }
    impl StrExt for str {
        fn is_blank(&self) -> bool {
            self.trim().is_empty()
        }
    }
    // Extension traits must be `use`d to be callable.
    println!("\"  \".is_blank(): {}", "  ".is_blank());
    println!("\"hi\".is_blank(): {}", "hi".is_blank());

    println!("trait_definitions_and_implementations section executed");
}

// =================================================================================================
// Section 5: Implementing Standard Library Traits
// =================================================================================================

/*
## Implementing Standard Library Traits

- The standard library defines many traits you can implement manually to integrate your types with
  Rust's ecosystem.
- **`fmt::Display`** enables `{}` formatting with `println!` and `format!`. You implement it by
  writing a `fmt` method that uses the `write!` macro. Module 005 covered print macros and format
  specifiers; this section shows how to make your own types support them.
- **`From<T>`** and **`Into<T>`** provide idiomatic type conversion. Implementing `From<A> for B`
  automatically gives you `Into<B> for A` (via a blanket implementation in `std`). Prefer
  implementing `From` — you get `Into` for free.
- The `into()` method is often used in generic contexts where the compiler can infer the target type
  from context.
- **`TryFrom<T>`** and **`TryInto<T>`** are the fallible counterparts of `From`/`Into`. They return
  `Result<T, E>` instead of `T`, used when conversion can fail (e.g., converting a large integer to
  a smaller type). Implementing `TryFrom` automatically gives `TryInto`.
- **`PartialOrd`** and **`Ord`** enable ordering comparisons (`<`, `>`, `<=`, `>=`) and total
  ordering (for sorting, BTreeMap keys). `PartialOrd` allows partial ordering (floats: NaN has no
  order). `Ord` requires total ordering (every pair is comparable). Both can be derived.
- **`PartialEq` and `PartialOrd` take `&self` / `&other`.** Unlike the arithmetic operator traits
  (`Add`, `Mul`, etc., covered in section 16), which take `self` and `rhs` by value and *consume*
  their operands, comparison traits take references. That way `a == b` on non-`Copy` types like
  `String` or `Vec<T>` does not move the operands, and you can still use them afterwards.
- **`ToOwned`** is the abstraction behind the `.to_owned()` method. `Clone::clone` returns `Self`,
  which requires `Self: Sized` — that rules out unsized types like `str` and `[T]`. `ToOwned` solves
  this by declaring an associated type:
  ```
  trait ToOwned {
      type Owned: Borrow<Self>;
      fn to_owned(&self) -> Self::Owned;
  }
  ```
  The standard library provides a blanket `impl<T: Clone> ToOwned for T`, so every `Clone` type gets
  `ToOwned<Owned = Self>` for free. Special implementations then cover the unsized cases: `str:
  ToOwned<Owned = String>`, `[T]: ToOwned<Owned = Vec<T>>`, `Path: ToOwned<Owned = PathBuf>`.
  `ToOwned` is also the trait `Cow<T>` (advanced module 001) uses to convert its borrowed side into
  an owned value.
- Note: `#[derive(Debug, Clone, PartialEq, Copy)]` for automatically generating trait
  implementations is covered in module 007 section 2. This section focuses on traits you implement
  **manually**.
*/

struct Temperature {
    celsius: f64,
}

// Display — enables {} formatting
impl Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.celsius)
    }
}

// From<f64> for Temperature — enables Temperature::from(100.0)
impl From<f64> for Temperature {
    fn from(c: f64) -> Self {
        Temperature { celsius: c }
    }
}

// From<Temperature> for f64 — enables converting back
impl From<Temperature> for f64 {
    fn from(t: Temperature) -> Self {
        t.celsius
    }
}

fn implementing_std_traits() {
    // Display trait — used by println!("{}", ...)
    let boiling = Temperature { celsius: 100.0 };
    println!("boiling point: {boiling}");

    // From trait — explicit conversion
    let body_temp = Temperature::from(36.6);
    println!("body temp: {body_temp}");

    // Into trait — available automatically because From is implemented
    let freezing: Temperature = 0.0.into();
    println!("freezing point: {freezing}");

    // Converting back: Temperature -> f64
    let raw: f64 = freezing.into();
    println!("raw celsius value: {raw}");

    // --- TryFrom / TryInto ---
    // Fallible conversion — returns Result
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Percentage(u8);

    impl TryFrom<i32> for Percentage {
        type Error = String;
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if (0..=100).contains(&value) {
                Ok(Percentage(value as u8))
            } else {
                Err(format!("{value} is not a valid percentage (0-100)"))
            }
        }
    }

    let valid: Result<Percentage, _> = 85.try_into();
    println!("TryFrom 85: {valid:?}");
    let invalid: Result<Percentage, _> = Percentage::try_from(150);
    println!("TryFrom 150: {invalid:?}");

    // --- PartialOrd / Ord ---
    // PartialOrd enables <, >, <=, >= comparisons
    // Ord enables total ordering (needed for sorting, BTreeMap)
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct Priority {
        level: u8,
        name: String,
    }

    let mut tasks = vec![
        Priority {
            level: 3,
            name: String::from("low"),
        },
        Priority {
            level: 1,
            name: String::from("critical"),
        },
        Priority {
            level: 2,
            name: String::from("medium"),
        },
    ];
    tasks.sort(); // requires Ord — sorts by level first, then name
    println!("sorted priorities: {tasks:?}");

    // --- Hash trait ---
    // Derivable: `#[derive(Hash)]`. Required for HashMap keys / HashSet elements.
    // Rule: if a == b (PartialEq), then hash(a) == hash(b).
    // Always derive Hash and PartialEq together.
    use std::hash::Hash;
    #[derive(PartialEq, Eq, Hash, Debug)]
    struct UserId(u64);
    // UserId can now be a HashMap key:
    let mut names = std::collections::HashMap::new();
    names.insert(UserId(1), "Alice");
    println!("UserId(1) -> {:?}", names.get(&UserId(1)));

    // --- Deref and DerefMut ---
    // Implementing Deref<Target = T> lets a type auto-dereference to &T.
    // Module 006 covered deref coercion usage; here we implement it.
    use std::ops::{Deref, DerefMut};

    struct Wrapper<T>(T);

    impl<T> Deref for Wrapper<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }
    impl<T> DerefMut for Wrapper<T> {
        fn deref_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    let w = Wrapper(String::from("hello"));
    // Methods on String are callable directly on Wrapper<String>:
    println!("Wrapper len via Deref: {}", w.len());

    let mut w = Wrapper(vec![1, 2, 3]);
    w.push(4); // DerefMut lets us call Vec::push
    println!("Wrapper after DerefMut push: {:?}", *w);

    // --- Derivable vs manually-implemented traits ---
    // Derivable (#[derive(...)]):
    //   Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default
    // Must be implemented manually:
    //   Display, From/Into, TryFrom/TryInto, Deref/DerefMut,
    //   Iterator, Drop, Error

    // --- ToOwned: producing an owned counterpart of a borrowed value ---
    // `.to_owned()` comes from the `ToOwned` trait, not `Clone`. The
    // standard library provides a blanket `impl<T: Clone> ToOwned for T`
    // with `type Owned = Self`, plus specialized impls for unsized types
    // so they can produce a sized, owned form:
    //   str  -> String
    //   [T]  -> Vec<T>     (where T: Clone)
    //   Path -> PathBuf
    // `Clone::clone(&str)` would need to return `str` by value, which
    // is impossible because `str` is unsized — ToOwned exists to cover
    // that gap.
    let borrowed_str: &str = "hello";
    let owned_string: String = borrowed_str.to_owned(); // str -> String
    let borrowed_slice: &[i32] = &[1, 2, 3];
    let owned_vec: Vec<i32> = borrowed_slice.to_owned(); // [i32] -> Vec<i32>
    let borrowed_path: &std::path::Path = std::path::Path::new("/tmp/x");
    let owned_pathbuf: std::path::PathBuf = borrowed_path.to_owned(); // Path -> PathBuf
    println!("to_owned: str -> String   = {owned_string:?}");
    println!("to_owned: [i32] -> Vec    = {owned_vec:?}");
    println!("to_owned: Path -> PathBuf = {owned_pathbuf:?}");
    // For a type that is already `Clone`, `.to_owned()` just delegates
    // to `.clone()` via the blanket impl:
    let n: i32 = 42;
    let m: i32 = n.to_owned(); // equivalent to n.clone()
    println!("to_owned on i32 (blanket impl) = {m}");

    println!("implementing_std_traits section executed");
}

// =================================================================================================
// Section 6: Trait Bounds
// =================================================================================================

/*
## Trait Bounds

- **Trait bounds** constrain generic type parameters, specifying what capabilities `T` must have:
  `fn foo<T: TraitName>(x: T)` means "`T` must implement `TraitName`".
- Multiple bounds use `+`: `fn foo<T: Display + Clone>(x: T)` — `T` must implement both.
- **`where` clause** — equivalent syntax, more readable for complex bounds:
  ```
  fn foo<T, U>(x: T, y: U)
  where
      T: Display + Clone,
      U: Debug,
  ```
- Rule of thumb: use inline bounds for simple cases (one parameter, one or two traits). Use `where`
  for anything more complex or when the function signature gets long.
- **`impl Trait` in argument position**: `fn foo(x: &impl Display)` is syntactic sugar for `fn
  foo<T: Display>(x: &T)`. It is shorter but has limitations: you cannot use turbofish syntax
  (`::<>`) and the caller cannot explicitly specify the type. Functionally equivalent for single
  parameters.
- **`impl Trait` vs `<T: Trait>` — crucial difference with multiple parameters.** For a single
  parameter the two forms are equivalent, but with two or more they are not. `fn notify(a: &impl
  Summary, b: &impl Summary)` desugars to **two independent** type parameters `<T1, T2>` — `a` and
  `b` can be different concrete types as long as each implements `Summary`. In contrast, `fn
  notify<T: Summary>(a: &T, b: &T)` forces `a` and `b` to be the **same** concrete type `T`. If you
  need "same type" semantics (e.g., comparing two values that must line up), reach for the generic
  form; if the parameters are independent, `impl Trait` reads more naturally.
*/

// Inline bound syntax
fn print_describable<T: Describable>(item: &T) {
    println!("{}", item.label());
}

// Multiple bounds with +
fn print_and_clone<T: Describable + Clone>(item: &T) {
    let cloned = item.clone();
    println!("original: {}", item.describe());
    println!("cloned:   {}", cloned.describe());
}

// where clause — cleaner for multiple parameters
fn compare_and_display<T, U>(a: &T, b: &U)
where
    T: Display,
    U: Display,
{
    println!("a = {a}, b = {b}");
}

// impl Trait in argument position — sugar for <T: Display>
fn print_displayable(item: &impl Display) {
    println!("{item}");
}

fn trait_bounds() {
    let product = Product {
        name: String::from("Gadget"),
        price: 29.99,
    };

    // Inline bound — T is inferred as Product (implements Describable)
    print_describable(&product);

    // Multiple bounds — Product must implement both Describable and Clone
    print_and_clone(&product);

    // where clause — two Display types
    let temp = Temperature::from(22.5);
    compare_and_display(&42, &temp);

    // impl Trait — sugar for a generic bound
    print_displayable(&temp);
    print_displayable(&"hello");

    println!("trait_bounds section executed");
}

// =================================================================================================
// Section 7: impl Trait in Return Position
// =================================================================================================

/*
## impl Trait in Return Position

- `fn foo() -> impl Trait` means "this function returns some type that implements `Trait`, but the
  caller does not know the concrete type". The return type is **opaque**.
- The function body must return **exactly one concrete type**. You cannot conditionally return
  different types — that requires trait objects (covered in section 8).
- This is useful for:
  - **Returning closures**, which have compiler-generated types that cannot be named.
  - **Hiding implementation details** — the caller depends only on the trait, not the concrete type.
- `impl Trait` in return position is a genuinely different feature from `impl Trait` in argument
  position. In argument position, it is sugar for generics (the caller picks the type). In return
  position, it is an **opaque type** (the function picks the type, and the caller only sees the
  trait).
- **Each `impl Trait` return produces a unique opaque type.** Two functions both declared `-> impl
  Fn(i32) -> i32` still have *distinct* opaque return types, so their results cannot be collected
  into a single `Vec<impl Fn(…)>`. The fix is to return `Box<dyn Fn(…)>` — both functions then share
  the same concrete return type (a trait-object pointer), at the cost of one heap allocation and a
  dynamic dispatch per call.
*/

// Returns some type implementing Describable — caller sees it as opaque
fn make_product() -> impl Describable {
    Product {
        name: String::from("Gadget"),
        price: 19.99,
    }
}

// ERROR: cannot return different concrete types with impl Trait
// fn make_item(is_product: bool) -> impl Describable {
//     if is_product {
//         Product { name: String::from("X"), price: 1.0 }
//     } else {
//         Service { name: String::from("Y"), hourly_rate: 2.0 }
//     }
// }

// Returning a closure — closures have unnameable types
fn make_adder(base: i32) -> impl Fn(i32) -> i32 {
    move |x| base + x
}

fn impl_trait_return_position() {
    // The caller only knows the return type implements Describable
    let item = make_product();
    println!("{}", item.label());

    // Returning closures — each call to make_adder creates a new closure type
    let add_ten = make_adder(10);
    println!("add_ten(5) = {}", add_ten(5));
    println!("add_ten(20) = {}", add_ten(20));

    let add_hundred = make_adder(100);
    println!("add_hundred(1) = {}", add_hundred(1));

    // --- RPITIT: Return Position Impl Trait in Trait ---
    // Since Rust 1.75, trait methods can use -> impl Trait.
    // Each implementor can return a different concrete type.
    trait Collection {
        fn items(&self) -> impl Iterator<Item = &str>;
    }
    struct CsvLine(String);
    impl Collection for CsvLine {
        fn items(&self) -> impl Iterator<Item = &str> {
            self.0.split(',').map(|s| s.trim())
        }
    }
    let csv = CsvLine("a, b, c".to_string());
    let items: Vec<&str> = csv.items().collect();
    println!("RPITIT items: {items:?}");

    // --- Heterogeneous closures: impl Fn vs Box<dyn Fn> ---
    // Two functions both returning `impl Fn(i32) -> i32` create
    // DIFFERENT opaque types — even though both implement the same
    // Fn trait. They cannot share a Vec:
    fn increment() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }
    fn double() -> impl Fn(i32) -> i32 {
        |x| x * 2
    }
    // let ops = vec![increment(), double()];
    // ERROR: mismatched types — expected opaque type, found a
    //        different opaque type
    // The `impl Fn` from each function is a distinct anonymous type,
    // so even calling the single-typed form doesn't help.
    println!("increment()(10) = {}", increment()(10));
    println!("double()(10)    = {}", double()(10));

    // Fix: return trait objects so both functions share the same
    // concrete return type (Box<dyn Fn…>).
    fn increment_dyn() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
    fn double_dyn() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x * 2)
    }
    let ops: Vec<Box<dyn Fn(i32) -> i32>> = vec![increment_dyn(), double_dyn()];
    for (i, op) in ops.iter().enumerate() {
        println!("  ops[{i}](10) = {}", op(10));
    }

    println!("impl_trait_return_position section executed");
}

// =================================================================================================
// Section 8: Trait Objects and Dynamic Dispatch
// =================================================================================================

/*
## Trait Objects and Dynamic Dispatch

- **Static dispatch** (default with generics): the compiler monomorphizes — generates a separate
  function copy for each concrete type. This is fast (can be inlined) but increases binary size.
- **Dynamic dispatch**: using `dyn Trait`, the concrete type is resolved at runtime via a **vtable**
  — a table of function pointers to the type's method implementations. There is a slight runtime
  cost (pointer indirection, no inlining), but it enables runtime polymorphism.
- `dyn Trait` is **unsized** — it cannot live on the stack directly. It must always be behind a
  pointer:
  - `Box<dyn Trait>` — owned trait object on the heap.
  - `&dyn Trait` — borrowed trait object.
- **Use cases for dynamic dispatch**:
  - **Heterogeneous collections**: `Vec<Box<dyn Trait>>` stores different concrete types in the same
    collection.
  - **Factory functions**: returning `Box<dyn Trait>` allows returning different types based on
    runtime conditions.
- **Duck typing with compile-time checking**: dynamically typed languages use the "if it walks like
  a duck and quacks like a duck, it must be a duck" style — a value is accepted anywhere a
  `.quack()` call is made, and missing methods explode at runtime. Trait objects give you the same
  flexibility (a `Vec<Box<dyn Renderer>>` accepts any type that implements `Renderer`) but the
  compiler rejects the program at compile time if you try to push a type that does not implement the
  trait. The runtime cost is just the vtable indirection; the runtime *risk* of duck typing is
  eliminated.
- **Object safety**: not all traits can be used as `dyn Trait`. A trait is object-safe if none of
  its methods return `Self` by value or have generic type parameters. **Why these rules?** A `dyn
  Trait` object is accessed through a vtable of function pointers — methods returning `Self` by
  value require knowing the concrete type's size (unknown behind `dyn`), and generic methods would
  require an infinite number of vtable entries (one per `T`). Methods with `where Self: Sized` are
  excluded from the vtable and do not affect object safety.
*/

trait Renderer {
    fn render(&self) -> String;
}

struct HtmlRenderer {
    tag: String,
}

impl Renderer for HtmlRenderer {
    fn render(&self) -> String {
        format!("<{0}>content</{0}>", self.tag)
    }
}

struct PlainTextRenderer;

impl Renderer for PlainTextRenderer {
    fn render(&self) -> String {
        String::from("content")
    }
}

// Static dispatch — the compiler monomorphizes one copy per type
fn render_static<T: Renderer>(r: &T) {
// fn render_static(r: &impl Renderer) {
    println!("static:  {}", r.render());
}

// Dynamic dispatch — vtable lookup at runtime
fn render_dynamic(r: &dyn Renderer) {
    println!("dynamic: {}", r.render());
}

// Factory — returns different types at runtime via Box<dyn Renderer>
fn create_renderer(html: bool) -> Box<dyn Renderer> {
    if html {
        Box::new(HtmlRenderer {
            tag: String::from("p"),
        })
    } else {
        Box::new(PlainTextRenderer)
    }
}

fn trait_objects_and_dynamic_dispatch() {
    let html = HtmlRenderer {
        tag: String::from("h1"),
    };
    let plain = PlainTextRenderer;

    // Static dispatch — separate monomorphized copies
    render_static(&html);
    render_static(&plain);

    // Dynamic dispatch — resolved via vtable at runtime
    render_dynamic(&html);
    render_dynamic(&plain);

    // Factory returns Box<dyn Renderer> — type decided at runtime
    let r = create_renderer(true);
    println!("factory: {}", r.render());

    // Heterogeneous collection — different types in one Vec
    let renderers: Vec<Box<dyn Renderer>> = vec![
        Box::new(HtmlRenderer {
            tag: String::from("div"),
        }),
        Box::new(PlainTextRenderer),
        Box::new(HtmlRenderer {
            tag: String::from("span"),
        }),
    ];
    for r in &renderers {
        println!("  collection: {}", r.render());
    }

    // --- Object safety violations (what makes a trait NOT object-safe) ---
    // These are shown as comments because they would not compile:

    // 1. Method returns Self by value — can't know the concrete size
    // trait NotSafe1 {
    //     fn clone_self(&self) -> Self; // ERROR as trait object
    // }

    // 2. Method has generic type parameters — vtable can't store them
    // trait NotSafe2 {
    //     fn convert<T>(&self) -> T; // ERROR as trait object
    // }

    // Workaround: use `where Self: Sized` to exclude methods from the vtable
    #[allow(dead_code)]
    trait SafeWithExclusion {
        fn render_safe(&self) -> String;
        fn clone_self(&self) -> Self
        where
            Self: Sized; // excluded from vtable
    }
    // Now `dyn SafeWithExclusion` is valid — clone_self is just unavailable

    // --- Trait object as struct field ---
    struct App {
        renderer: Box<dyn Renderer>,
    }
    let app = App {
        renderer: create_renderer(true),
    };
    println!("app renderer: {}", app.renderer.render());

    println!("trait_objects_and_dynamic_dispatch section executed");
}

// =================================================================================================
// Section 9: Supertraits
// =================================================================================================

/*
## Supertraits

- A **supertrait** is a trait bound placed on another trait definition: `trait Loggable: Printable`
  means "any type implementing `Loggable` must also implement `Printable`".
- The subtrait's default methods can call methods from its supertrait(s), because the compiler
  guarantees they are available.
- Multiple supertraits: `trait B: A + C + D`.
- This is **not** inheritance in the object-oriented sense. There is no shared state, no method
  resolution order, and no "is-a" relationship between types. It is purely a **constraint**:
  implementing the subtrait requires also implementing the supertrait(s).
- A function with a subtrait bound can call methods from both the subtrait and all its supertraits.
- Real-world example: `std::error::Error: Display + Debug` — any error type must be both displayable
  and debuggable.
*/

trait Printable {
    fn print_out(&self);
}

// Loggable requires Printable as a supertrait
trait Loggable: Printable {
    fn log_level(&self) -> &str;

    // Default method — calls print_out() from the supertrait
    fn log(&self) {
        print!("[{}] ", self.log_level());
        self.print_out();
    }
}

struct Event {
    message: String,
}

impl Printable for Event {
    fn print_out(&self) {
        println!("{}", self.message);
    }
}

impl Loggable for Event {
    fn log_level(&self) -> &str {
        "INFO"
    }
    // log() uses the default, which calls self.print_out()
}

// A function requiring the subtrait can call both subtrait and
// supertrait methods
fn log_item(item: &impl Loggable) {
    item.log(); // from Loggable (default, calls print_out)
    item.print_out(); // from Printable — guaranteed by supertrait bound
}

fn supertraits() {
    let event = Event {
        message: String::from("server started"),
    };

    // Call supertrait method directly
    event.print_out();

    // Call subtrait default method (which internally calls print_out)
    event.log();

    // Function that requires the subtrait
    log_item(&event);

    println!("supertraits section executed");
}

// =================================================================================================
// Section 10: Associated Types
// =================================================================================================

/*
## Associated Types

- Traits can declare **associated types**: a type placeholder that each implementor specifies
  concretely.
  ```
  trait Convert {
      type Target;
      fn convert(&self) -> Self::Target;
  }
  ```
- In the implementation, the associated type is pinned to a concrete type:
  ```
  impl Convert for Kilometers {
      type Target = Miles;
      fn convert(&self) -> Miles { ... }
  }
  ```
- Associated types are appropriate when there is **exactly one** logical output type per
  implementor. Each type can implement the trait only once, and the associated type is fixed by that
  implementation.
- When you need a type to implement the same trait **multiple times** with different type arguments,
  use generic type parameters on the trait instead (covered in section 11).
- Real-world example: the `Iterator` trait has `type Item;` — each iterator produces exactly one
  item type.
- In generic contexts, you can constrain the associated type: `T: Convert<Target = Miles>` or use
  `where T::Target: Debug`.
- Traits can also have **associated constants**: `const MAX: usize;` in the trait, with each
  implementor providing a value. Example: `trait Buffer { const CAPACITY: usize; }`.
*/

#[derive(Debug)]
struct Kilometers {
    value: f64,
}

#[derive(Debug)]
struct Miles {
    value: f64,
}

trait DistanceConvert {
    type Target;
    fn convert(&self) -> Self::Target;
}

impl DistanceConvert for Kilometers {
    type Target = Miles;
    fn convert(&self) -> Miles {
        Miles {
            value: self.value * 0.621_371,
        }
    }
}

impl DistanceConvert for Miles {
    type Target = Kilometers;
    fn convert(&self) -> Kilometers {
        Kilometers {
            value: self.value * 1.609_344,
        }
    }
}

// Function using the associated type — requires Target to be Debug
fn show_conversion<T: DistanceConvert + fmt::Debug>(d: &T)
where
    T::Target: fmt::Debug,
{
    println!("{:?} -> {:?}", d, d.convert());
}

fn associated_types() {
    let km = Kilometers { value: 100.0 };
    let mi = Miles { value: 60.0 };

    show_conversion(&km); // Kilometers -> Miles
    show_conversion(&mi); // Miles -> Kilometers

    // Direct conversion
    let converted = km.convert();
    println!("100 km = {:.1} miles", converted.value);

    println!("associated_types section executed");
}

// =================================================================================================
// Section 11: Generic Type Parameters on Traits vs. Associated Types
// =================================================================================================

/*
## Generic Type Parameters on Traits vs. Associated Types

- A trait with a **generic type parameter** allows a single type to implement the trait **multiple
  times** with different type arguments:
  ```
  trait Combine<Rhs> { fn combine(self, rhs: Rhs) -> ...; }
  impl Combine<Vector2D> for Vector2D { ... }  // vector + vector
  impl Combine<f64> for Vector2D { ... }       // vector * scalar
  ```
- A trait with an **associated type** allows only **one implementation per type** — the output type
  is fixed by the implementor (as shown in section 10).
- **Rule of thumb**:
  - Use **associated types** when there is a single natural relationship (e.g., `Iterator::Item`,
    `Deref::Target`).
  - Use **generic parameters** when multiple relationships are valid (e.g., `Add<Rhs>`, `From<T>`).
- The standard `Add` trait combines both approaches: `trait Add<Rhs = Self> { type Output; fn
  add(self, rhs: Rhs) -> Self::Output; }` `Rhs` is a generic parameter (you can add different
  right-hand types), but `Output` is an associated type (each `Rhs` produces exactly one result
  type).
*/

// Generic type parameter — allows multiple impls per type
trait Combine<Rhs> {
    type Output;
    fn combine(self, rhs: Rhs) -> Self::Output;
}

struct Vector2D {
    x: f64,
    y: f64,
}

// Combine<Vector2D> — vector addition
impl Combine<Vector2D> for Vector2D {
    type Output = Vector2D;
    fn combine(self, rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Combine<f64> — scalar multiplication (same type, different Rhs)
impl Combine<f64> for Vector2D {
    type Output = Vector2D;
    fn combine(self, scalar: f64) -> Vector2D {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// Using std::ops::Add — Rhs defaults to Self
struct Offset {
    dx: i32,
    dy: i32,
}

impl Add for Offset {
    type Output = Offset;
    fn add(self, rhs: Offset) -> Offset {
        Offset {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
        }
    }
}

fn generic_vs_associated_types() {
    // Multiple impls: Combine<Vector2D> and Combine<f64>
    let v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = Vector2D { x: 3.0, y: 4.0 };
    let sum = v1.combine(v2); // Combine<Vector2D>
    println!("sum: ({}, {})", sum.x, sum.y);

    let v3 = Vector2D { x: 1.0, y: 2.0 };
    let scaled = v3.combine(3.0); // Combine<f64>
    println!("scaled: ({}, {})", scaled.x, scaled.y);

    // std::ops::Add with default Rhs = Self
    let total = Offset { dx: 1, dy: 2 } + Offset { dx: 3, dy: 4 };
    println!("offset: ({}, {})", total.dx, total.dy);

    println!("generic_vs_associated_types section executed");
}

// =================================================================================================
// Section 12: Marker Traits and Trait Composition
// =================================================================================================

/*
## Marker Traits and Trait Composition

- **Marker traits** are traits with **no methods**. They signal a capability or property to the
  compiler or downstream code. Standard examples:
  - `Send` — the type can be transferred across thread boundaries.
  - `Sync` — the type can be shared between threads via references.
  - `Sized` — the type has a known size at compile time (most types are `Sized` by default).
- You can define **custom marker traits** to group requirements: `trait Storable: Clone + Debug +
  PartialEq {}`. Any type implementing all three supertraits can then implement `Storable`.
- **Blanket implementations** can automatically implement a marker trait for every type that
  satisfies the bounds: `impl<T: Clone + Debug + PartialEq> Storable for T {}` Now every type
  meeting the bounds is automatically `Storable`, with no per-type `impl` needed.
- **Trait composition** in bounds: `T: Clone + Debug + Display` requires `T` to implement all three
  traits. This works in generic functions, struct definitions, and `where` clauses.
- Note: `Copy`, `Clone`, `Debug`, `PartialEq` derive macros were covered in module 007 section 2.
  Here the focus is on using traits as **bounds** and composing **custom marker traits**.
*/

// Custom marker trait — groups multiple requirements
trait Storable: Clone + fmt::Debug + PartialEq {}

// Blanket implementation — every type satisfying the bounds is
// automatically Storable
impl<T: Clone + fmt::Debug + PartialEq> Storable for T {}

// Now Storable can be used as a single, concise bound
fn store_item<T: Storable>(item: &T) {
    println!("storing: {:?}", item);
    let backup = item.clone();
    assert_eq!(item, &backup);
    println!("  verified backup matches original");
}

// Marker trait with no supertraits — a pure tag
trait Auditable {}

#[derive(Debug)]
#[allow(dead_code)]
struct Transaction {
    id: u32,
    amount: f64,
}

impl Auditable for Transaction {}

fn audit<T: Auditable + fmt::Debug>(item: &T) {
    println!("audit record: {:?}", item);
}

fn marker_traits_and_composition() {
    // Storable works for any Clone + Debug + PartialEq type
    store_item(&42);
    store_item(&String::from("data"));
    store_item(&vec![1, 2, 3]);

    // Auditable is a pure marker — no methods, just signals intent
    let tx = Transaction {
        id: 1,
        amount: 99.95,
    };
    audit(&tx);

    // Types without Auditable cannot be audited:
    // audit(&42); // ERROR: the trait `Auditable` is not implemented for `i32`

    // --- Dynamically sized types (DSTs) ---
    // `str` and `[T]` are themselves DSTs — their size is NOT known
    // at compile time. You cannot store a DST directly in a variable:
    //     let s: str = "hello";
    //     // ERROR: the size for values of type `str` cannot be
    //     //        known at compilation time
    //
    // DSTs must always live behind a pointer:
    //   - &str                  (fat pointer: address + length)
    //   - Box<str>               (owned, on the heap)
    //   - Rc<str> / Arc<str>
    // A &str is 2×usize: it stores both the address of the string
    // bytes AND the byte length, which is how the compiler knows
    // *its* size even though `str` itself has none. Trait objects
    // (`&dyn Trait`, `Box<dyn Trait>`) are DSTs for the same reason
    // (section 8 above) — the pointer carries a vtable alongside
    // the data pointer.

    // --- ?Sized bound ---
    // By default, type parameters have an implicit `Sized` bound.
    // `?Sized` relaxes this, allowing dynamically sized types (DSTs)
    // like `str` and `[T]`. This is useful for functions accepting
    // both owned types and their unsized counterparts via reference.
    fn print_ref<T: ?Sized + Display>(t: &T) {
        println!("?Sized value: {t}");
    }
    print_ref(&42); // &i32 (Sized)
    print_ref("hello"); // &str — pointer to DST `str`
    print_ref(&String::from("world")); // &String (Sized — works like &i32 above)

    println!("marker_traits_and_composition section executed");
}

// =================================================================================================
// Section 13: Const Generics
// =================================================================================================

/*
## Const Generics

- **Const generics** allow types and functions to be parameterized over **constant values** (not
  just types): `struct Array<T, const N: usize>`.
- The const parameter must be a primitive type (`usize`, `bool`, `char`, integer types).
- Stable since Rust 1.51. Commonly used for fixed-size arrays, buffers, and matrices where the size
  is part of the type.
- `[T; N]` in the standard library uses const generics internally.
- **Default type parameters**: generic type params can have defaults: `struct Foo<T = i32>`. If the
  caller omits the parameter, the default is used. The standard library uses this (e.g., `HashMap<K,
  V, S = RandomState>`).
*/

#[derive(Debug)]
struct FixedBuffer<T, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> FixedBuffer<T, N> {
    fn new() -> Self {
        Self {
            data: [T::default(); N],
        }
    }

    fn len(&self) -> usize {
        N
    }
}

// Const generic in a function — works with any array size
fn sum_array<const N: usize>(arr: &[i32; N]) -> i32 {
    arr.iter().sum()
}

// Default type parameter — T defaults to String if not specified
struct Container<T = String> {
    value: T,
}

fn const_generics() {
    // Const generic struct — size is part of the type
    let buf: FixedBuffer<i32, 5> = FixedBuffer::new();
    println!("buffer: {:?}, len: {}", buf.data, buf.len());

    let small: FixedBuffer<u8, 3> = FixedBuffer::new();
    println!("small: {:?}, len: {}", small.data, small.len());

    // Const generic function — works with any array size
    let a = [1, 2, 3];
    let b = [10, 20, 30, 40, 50];
    println!("sum of 3: {}", sum_array(&a));
    println!("sum of 5: {}", sum_array(&b));

    // Default type parameter — omit to use the default
    let default_container = Container {
        value: String::from("hello"),
    };
    println!("default type: {}", default_container.value);

    let int_container = Container { value: 42i32 };
    println!("explicit type: {}", int_container.value);

    println!("const_generics section executed");
}

// =================================================================================================
// Section 14: Closure Trait Bounds
// =================================================================================================

/*
## Closure Trait Bounds

- Closures implement one or more of three traits based on how they capture variables:
  - **`Fn`** — borrows captured state immutably (call takes `&self`). Can be called repeatedly. Used
    for callbacks, event handlers.
  - **`FnMut`** — borrows captured state mutably (call takes `&mut self`). Can be called repeatedly
    but may mutate state. Used for accumulators.
  - **`FnOnce`** — consumes captured state (call takes `self`). Can be called only once. Every
    closure implements this.
- Hierarchy: `Fn` ⊂ `FnMut` ⊂ `FnOnce`. A `Fn` closure can be used wherever `FnMut` or `FnOnce` is
  required.
- Generic functions accepting closures use these traits as bounds: `fn apply<F: Fn(i32) -> i32>(f:
  F, x: i32) -> i32`.
- Standard library methods use these bounds:
  - `.map()` takes `FnMut`
  - `.filter()` takes `FnMut` (returning bool)
  - `thread::spawn()` takes `FnOnce`
*/

// Accepts Fn — the closure can be called multiple times
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

// Accepts FnMut — the closure may mutate state
fn call_n_times<F: FnMut()>(mut f: F, n: u32) {
    for _ in 0..n {
        f();
    }
}

// Accepts FnOnce — the closure is consumed after one call
fn consume_and_run<F: FnOnce() -> String>(f: F) -> String {
    f()
}

fn closure_trait_bounds() {
    // Fn — pure transformation, no mutation
    let double = |x: i32| x * 2;
    println!("apply_twice(double, 3) = {}", apply_twice(double, 3)); // 12
    println!(
        "apply_twice(|x| x + 10, 5) = {}",
        apply_twice(|x| x + 10, 5)
    ); // 25

    // FnMut — closure mutates captured state
    let mut count = 0;
    call_n_times(
        || {
            count += 1;
        },
        5,
    );
    println!("count after 5 calls: {count}");

    // FnOnce — closure consumes a captured value
    let name = String::from("Rust");
    let result = consume_and_run(move || {
        format!("Hello, {name}!") // name is moved into the closure
    });
    println!("{result}");

    // Returning closures — use impl Fn (covered in section 7)
    fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
        move |x| x * factor
    }
    let triple = make_multiplier(3);
    println!("triple(7) = {}", triple(7));

    println!("closure_trait_bounds section executed");
}

// =================================================================================================
// Section 15: Lifetimes with Generic Type Parameters
// =================================================================================================

/*
## Lifetimes with Generic Type Parameters

- Lifetime parameters (`'a`) and type parameters (`T`) can be combined in the same generic
  signature: `fn foo<'a, T>(...)`.
- Convention: lifetime parameters come before type parameters.
- Common in structs that hold both owned and borrowed data.
*/

// A struct with both a lifetime and a type parameter
#[derive(Debug)]
struct Annotated<'a, T> {
    value: T,
    label: &'a str,
}

impl<'a, T: Display> Annotated<'a, T> {
    fn describe(&self) -> String {
        format!("{}: {}", self.label, self.value)
    }
}

// Function with lifetime + type parameter
fn longest_display<'a, T: Display>(items: &'a [T]) -> &'a T {
    let mut longest_idx = 0;
    let mut longest_len = 0;
    for (i, item) in items.iter().enumerate() {
        let len = format!("{item}").len();
        if len > longest_len {
            longest_len = len;
            longest_idx = i;
        }
    }
    &items[longest_idx]
}

fn lifetimes_with_generics() {
    // Struct with lifetime + type parameter
    let label = String::from("count");
    let annotated = Annotated {
        value: 42,
        label: &label,
    };
    println!("{}", annotated.describe());

    let annotated_float = Annotated {
        value: 3.14,
        label: "pi",
    };
    println!("{:?}", annotated_float);

    // Function with lifetime + type parameter
    let words = ["hi", "hello", "greetings"];
    let longest = longest_display(&words);
    println!("longest display: {longest}");

    println!("lifetimes_with_generics section executed");
}

// =================================================================================================
// Section 16: Additional Operator Overloading
// =================================================================================================

/*
## Additional Operator Overloading

- The `std::ops` module provides traits for overloading operators beyond `Add` (shown in section
  11):
  - `Sub` (`-`), `Mul` (`*`), `Div` (`/`), `Rem` (`%`)
  - `Neg` (unary `-`), `Not` (unary `!`)
  - `Index` / `IndexMut` (the `[]` operator)
  - `BitAnd`, `BitOr`, `BitXor`, `Shl`, `Shr`
- Each trait follows the same pattern: define the `Output` type and implement the corresponding
  method.
- **Compound assignment traits**: `AddAssign` (`+=`), `SubAssign` (`-=`), `MulAssign` (`*=`), etc.
  These modify the left operand in place — they take `&mut self` and have no `Output` type.
- **`Index` / `IndexMut` desugaring**: writing `a[i]` calls `*Index::index(&a, i)`, and `a[i] =
  value` calls `*IndexMut::index_mut(&mut a, i) = value`. The `Output` associated type declares what
  a single lookup returns — often `&T` for element access, but also `&[T]` to allow chained indexing
  like `matrix[row][col]` where `matrix[row]` is a slice and `slice[col]` is the element.
- **`PartialOrd::partial_cmp` and `Option<Ordering>`.** Unlike `Ord::cmp`, which must always produce
  a total order (every pair is comparable), `PartialOrd::partial_cmp` returns `Option<Ordering>` and
  is allowed to answer `None` when two values are *incomparable*. This is why `f64` implements
  `PartialOrd` but not `Ord`: `NaN < x`, `NaN > x`, and `NaN == x` are all `false` for every `x`.
  The same design lets you implement a *partial* order over your own types (e.g., overlapping
  intervals, DAG ancestry), where the `<` / `>` operators still work but some pairs have no defined
  ordering.
*/

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f64) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// AddAssign — implements the += operator
impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// Reverse operator: scalar * vector (requires impl on the scalar type).
// This is allowed by the orphan rule because Vec2 (a local type) appears
// as a generic parameter of Mul — the rule in section 4 is the common
// case; the full rule also accepts a local type in a covered generic-
// parameter position.
impl Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

fn operator_overloading() {
    let a = Vec2 { x: 1.0, y: 2.0 };
    let b = Vec2 { x: 3.0, y: 4.0 };

    println!("a + b = {:?}", a + b);
    println!("-a = {:?}", -a);
    println!("a * 3.0 = {:?}", a * 3.0);

    // Reverse: scalar * vector
    println!("3.0 * a = {:?}", 3.0 * a);

    // Compound assignment: +=
    let mut c = Vec2 { x: 1.0, y: 1.0 };
    c += Vec2 { x: 2.0, y: 3.0 };
    println!("after += : {:?}", c);

    // Note: overloading operators does NOT change their precedence or
    // associativity — those are fixed by the language. `a + b * c` always
    // evaluates `b * c` first, regardless of the types involved.

    // --- Not and BitOr: unary ! and binary | ---
    // Same shape as Add/Neg: each trait declares an `Output` and a
    // single method. Here is a tiny permission bitset.
    use std::ops::{BitOr, Not};

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Perm(u8);

    const READ: Perm = Perm(0b001);
    const WRITE: Perm = Perm(0b010);
    const EXEC: Perm = Perm(0b100);

    impl BitOr for Perm {
        type Output = Perm;
        fn bitor(self, rhs: Perm) -> Perm {
            Perm(self.0 | rhs.0)
        }
    }

    impl Not for Perm {
        type Output = Perm;
        fn not(self) -> Perm {
            // Mask to the 3 valid bits so `!READ` doesn't light up
            // the unused upper bits of the u8.
            Perm(!self.0 & 0b111)
        }
    }

    let rw = READ | WRITE;
    println!("READ | WRITE = {rw:?} (bits={:03b})", rw.0);
    let rwx = rw | EXEC;
    println!("rw | EXEC    = {rwx:?} (bits={:03b})", rwx.0);
    let no_read = !READ;
    println!("!READ        = {no_read:?} (bits={:03b})", no_read.0);

    // --- Index and IndexMut: the `[]` operator ---
    use std::ops::{Index, IndexMut};

    struct Grid<T> {
        rows: usize,
        cols: usize,
        data: Vec<T>,
    }

    // Tuple indexing: grid[(row, col)] returns &T.
    impl<T> Index<(usize, usize)> for Grid<T> {
        type Output = T;
        fn index(&self, (r, c): (usize, usize)) -> &T {
            &self.data[r * self.cols + c]
        }
    }

    impl<T> IndexMut<(usize, usize)> for Grid<T> {
        fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut T {
            &mut self.data[r * self.cols + c]
        }
    }

    // Row indexing: grid[row] returns &[T] (a slice = one row).
    // This enables the chained form `grid[row][col]`, where the second
    // `[col]` is the slice's own `Index` impl.
    impl<T> Index<usize> for Grid<T> {
        type Output = [T];
        fn index(&self, row: usize) -> &[T] {
            let start = row * self.cols;
            &self.data[start..start + self.cols]
        }
    }

    let mut grid: Grid<i32> = Grid {
        rows: 2,
        cols: 3,
        data: vec![0; 6],
    };
    // `grid[(0, 0)] = 1` desugars to
    //   *IndexMut::index_mut(&mut grid, (0, 0)) = 1
    grid[(0, 0)] = 1;
    grid[(0, 1)] = 2;
    grid[(1, 2)] = 9;
    // `grid[(1, 2)]` desugars to *Index::index(&grid, (1, 2))
    println!("grid[(0,0)]={}, grid[(1,2)]={}", grid[(0, 0)], grid[(1, 2)]);
    // Chained `grid[1][2]` works because `Index<usize>` returns &[T],
    // and the slice has its own `Index<usize>` -> &T.
    println!("grid[1][2] via row indexing: {}", grid[1][2]);
    println!("grid has {} rows and {} cols", grid.rows, grid.cols);

    // --- PartialOrd with partial_cmp returning None ---
    // Overlapping intervals have no defined ordering: neither is
    // strictly before, after, nor equal to the other.
    use std::cmp::Ordering;

    #[derive(Debug, PartialEq)]
    struct Interval {
        lo: i32,
        hi: i32,
    }

    impl PartialOrd for Interval {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.hi < other.lo {
                Some(Ordering::Less) // `self` ends before `other` starts
            } else if self.lo > other.hi {
                Some(Ordering::Greater) // `self` starts after `other` ends
            } else if self == other {
                Some(Ordering::Equal)
            } else {
                None // overlapping — no defined order
            }
        }
    }

    let a = Interval { lo: 0, hi: 5 };
    let b = Interval { lo: 10, hi: 20 };
    let c = Interval { lo: 3, hi: 7 }; // overlaps with `a`

    println!("a={a:?}, b={b:?}, c={c:?}");
    println!("  a < b              = {}", a < b); // true
    println!("  a.partial_cmp(&b)  = {:?}", a.partial_cmp(&b)); // Some(Less)
    println!("  a.partial_cmp(&c)  = {:?}", a.partial_cmp(&c)); // None
    println!("  a <  c             = {}", a < c); // false
    println!("  a >= c             = {}", a >= c); // false — both sides are false when incomparable
    // Note: we deliberately did NOT derive `Ord` here. A total order
    // is the wrong model for overlapping intervals, and `Ord` would
    // let `sort()` silently produce a meaningless result.

    println!("operator_overloading section executed");
}

// =================================================================================================
// Section 17: AsRef and AsMut — Reference Conversion
// =================================================================================================

/*
## AsRef and AsMut

- **`AsRef<T>`** is the trait behind "give me a cheap reference view of this value as `&T`". The
  trait is tiny:
  ```
  trait AsRef<T: ?Sized> {
      fn as_ref(&self) -> &T;
  }
  ```
  `T` is usually an unsized type like `str`, `[u8]`, or `Path`.
- **Why it exists**: a function that logically "takes a filename" really wants `&Path`, but callers
  have `&str`, `String`, `PathBuf`, `&OsStr`, etc. Rather than overloading with many variants, the
  standard library writes one generic:
  ```
  pub fn open<P: AsRef<Path>>(path: P) -> io::Result<File>
  ```
  and every caller type implements `AsRef<Path>`.
- **Blanket impl for references** (roughly): `impl<T: ?Sized, U: ?Sized> AsRef<U> for &T where T:
  AsRef<U>`. This is why you can pass `&String` *and* `String` to the same generic `AsRef<Path>`
  parameter — the blanket adds one level of indirection for free. That is also why
  `File::open("foo.txt")` works even though it is `str` (not `&str`) that implements `AsRef<Path>`:
  the blanket lifts the impl from `str` to `&str`.
- Key implementations worth remembering:
  - `String: AsRef<str>` and `String: AsRef<[u8]>`.
  - `Vec<T>: AsRef<[T]>`.
  - `PathBuf: AsRef<Path>`; `str: AsRef<Path>`; `OsStr: AsRef<Path>`.
- **`AsMut<T>`** is the mutable counterpart: `fn as_mut(&mut self) -> &mut T`. Less common. Notable
  absence: `String` does **not** implement `AsMut<[u8]>` — exposing the bytes mutably would let
  callers break the UTF-8 invariant. The intentional gap is a good reminder that `AsRef` / `AsMut`
  impls encode design decisions, not just mechanical conversions.
- **`AsRef` vs `Borrow`** (next section): `AsRef` is about cheap reference conversion. `Borrow`
  additionally guarantees that hash, equality, and ordering agree between the owned and borrowed
  forms. Use `AsRef<T>` for flexible argument passing; use `Borrow<T>` for keyed lookups in
  `HashMap` / `BTreeMap`.
*/

fn as_ref_and_as_mut() {
    // A generic function accepting anything that can be viewed as &str.
    fn show<S: AsRef<str>>(label: &str, s: S) {
        println!("  {label:12} -> {:?}", s.as_ref());
    }

    // All of these call sites compile: each argument type either
    // implements `AsRef<str>` directly or picks it up via the
    // blanket `impl<T: AsRef<U>> AsRef<U> for &T`.
    let owned = String::from("hello");
    let boxed: Box<str> = Box::from("boxed");
    show("&'static str", "string literal");
    show("String", owned.clone());
    show("&String", &owned); // via blanket impl on &T
    show("&Box<str>", &boxed); // via blanket impl on &T

    // The same pattern with `AsRef<Path>` — this is how `File::open`,
    // `fs::read_to_string`, etc. accept such a wide range of inputs.
    fn file_stem<P: AsRef<std::path::Path>>(p: P) -> Option<String> {
        p.as_ref()
            .file_stem()
            .map(|os| os.to_string_lossy().into_owned())
    }
    println!("  stem &str   : {:?}", file_stem("report.txt"));
    println!(
        "  stem PathBuf: {:?}",
        file_stem(std::path::PathBuf::from("/tmp/a.log"))
    );

    // --- AsMut: mutable viewing ---
    // Accept anything we can obtain a `&mut [i32]` from and zero it.
    fn zero_all<T: AsMut<[i32]>>(mut buf: T) {
        for x in buf.as_mut() {
            *x = 0;
        }
    }
    let mut v = vec![1, 2, 3];
    zero_all(&mut v); // Vec<i32>: AsMut<[i32]>
    println!("  after zero_all on Vec : {v:?}");

    let mut arr = [7, 8, 9];
    zero_all(&mut arr); // [i32; N]: AsMut<[i32]>
    println!("  after zero_all on arr: {arr:?}");

    // `String` deliberately does NOT implement `AsMut<[u8]>` — letting
    // callers edit the bytes would risk breaking the UTF-8 invariant.
    // `String::as_mut_str` hands back `&mut str` instead, which still
    // constrains modifications to valid UTF-8.

    println!("as_ref_and_as_mut section executed");
}

// =================================================================================================
// Section 18: Borrow and BorrowMut — Hash-Compatible Reference Conversion
// =================================================================================================

/*
## Borrow and BorrowMut

- **`Borrow<T>`** looks nearly identical to `AsRef<T>`:
  ```
  trait Borrow<Borrowed: ?Sized> {
      fn borrow(&self) -> &Borrowed;
  }
  ```
  The *signature* is the same; the difference is a **contract**. `Borrow<T>` requires that the
  returned `&T` hash, compare, and order **exactly** the same as the original value. Without that
  guarantee, keyed lookups in hash maps and trees would be unsound.
- **Why it exists**: consider `HashMap<String, V>`. You want to look up entries with a `&str`
  instead of constructing a throw- away `String` every time. `HashMap::get` is generic:
  ```
  pub fn get<Q>(&self, k: &Q) -> Option<&V>
  where
      K: Borrow<Q>,
      Q: Hash + Eq + ?Sized,
  ```
  The `K: Borrow<Q>` bound says "I can view my keys as `&Q`, and the hash/equality of the `&Q` view
  matches the hash/equality of the `K` key". `String` implements `Borrow<str>`, so `map.get("foo")`
  works even though the key type is `String`.
- **`String: Borrow<str>`** — ✅. Good: the hash of a `String` equals the hash of the `&str` it
  dereferences to.
- **`String: Borrow<[u8]>`** — ❌ intentionally absent. The raw conversion is trivial, but `str` and
  `[u8]` have different `Hash` implementations, so a map keyed by `String` could not correctly look
  up entries with `&[u8]`. This is the concrete place where `Borrow` differs from `AsRef`: `String`
  *does* implement `AsRef<[u8]>`, because conversion-without-contract is harmless, but
  `Borrow<[u8]>` would be a landmine.
- **`BorrowMut<T>`** adds a `fn borrow_mut(&mut self) -> &mut T` method with the same hash/eq/ord
  contract. Rarely needed in everyday code.
- **When to reach for which**:
  - Accepting flexible input → `AsRef<T>`.
  - Keying or indexing a collection → `Borrow<T>`.
  - If in doubt, `AsRef<T>` is the safer default — `Borrow<T>` is specifically for "I am used as the
    key/lookup type here".
*/

fn borrow_and_borrow_mut() {
    use std::borrow::Borrow;
    use std::collections::HashMap;

    // Build a HashMap<String, i32> and then look up with `&str`.
    // `HashMap::get<Q>` accepts any `Q` such that `K: Borrow<Q>`;
    // since `String: Borrow<str>`, `&str` is a valid query type.
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("alice"), 10);
    scores.insert(String::from("bob"), 20);

    let by_literal: Option<&i32> = scores.get("alice");
    println!("  scores.get(\"alice\") = {by_literal:?}");
    // Note: a `&"alice"` literal has type `&&str`. Auto-deref on the
    // receiver and the `K: Borrow<Q>` bound together make this work
    // without writing any explicit conversion.

    // Direct call to `.borrow()` to see what `String: Borrow<str>`
    // actually produces.
    let owned = String::from("rust");
    let s: &str = owned.borrow();
    println!("  owned.borrow()       = {s:?}");

    // A generic helper that accepts anything borrowable as `&str`.
    fn first_char<S: Borrow<str>>(s: S) -> Option<char> {
        s.borrow().chars().next()
    }
    println!(
        "  first_char(String)   = {:?}",
        first_char(String::from("hello"))
    );
    println!("  first_char(&str)     = {:?}", first_char("world"));

    // Contrast: `AsRef<str>` would work just as well in this helper,
    // because `first_char` never uses the value as a hash map key.
    // `Borrow` and `AsRef` are interchangeable here; the difference
    // only matters when the caller relies on hash/eq equivalence.

    println!("borrow_and_borrow_mut section executed");
}

pub fn run() {
    generic_functions();
    generic_structs_and_enums();
    generic_impl_blocks();
    trait_definitions_and_implementations();
    implementing_std_traits();
    trait_bounds();
    impl_trait_return_position();
    trait_objects_and_dynamic_dispatch();
    supertraits();
    associated_types();
    generic_vs_associated_types();
    marker_traits_and_composition();
    const_generics();
    closure_trait_bounds();
    lifetimes_with_generics();
    operator_overloading();
    as_ref_and_as_mut();
    borrow_and_borrow_mut();
}
