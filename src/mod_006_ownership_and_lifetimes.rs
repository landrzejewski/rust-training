// =================================================================================================
// Section 1: Ownership Rules
// =================================================================================================

/*
## Ownership Rules

- Rust manages heap memory without a garbage collector. Instead, it enforces a set of **ownership
  rules** at compile time — zero runtime cost.
- **Rule 1**: Every value has exactly one variable that is its **owner**.
- **Rule 2**: There can only be **one owner at a time**. Assigning a heap-allocated value to another
  variable **moves** ownership — the previous owner becomes invalid.
- **Rule 3**: When the owner goes **out of scope**, Rust automatically calls `drop` and frees the
  associated memory. This is powered by the **`Drop` trait** — types that own resources (memory,
  files, locks) implement `Drop` to define their cleanup logic. This is Rust's form of RAII
  (Resource Acquisition Is Initialization). You can also drop a value **early** by calling
  `drop(value)` explicitly (e.g., to release a lock or free memory sooner).
- Types that implement the `Copy` trait are **copied** on assignment instead of moved. (Most `Copy`
  types happen to be stack-allocated, but `Copy` is about semantics — bitwise duplication — not
  storage location.) Common `Copy` types:
  - All integer types (`i8`–`i128`, `u8`–`u128`, `isize`, `usize`)
  - All floating-point types (`f32`, `f64`)
  - `bool`, `char`
  - Tuples, if all elements are `Copy` (e.g., `(i32, bool)`)
  - Fixed-size arrays, if the element is `Copy` (e.g., `[i32; 5]`)
  - Shared references (`&T`) — but **not** `&mut T`
  - Function pointers (`fn(i32) -> i32`)
- Heap-allocated types (`String`, `Vec<T>`, `HashMap<K,V>`, etc.) do **not** implement `Copy` —
  assignment transfers ownership.

### Stack vs heap (short refresher)

- **Stack** — LIFO memory where each value has a fixed, compile-time-known size. Push/pop are
  extremely fast because the allocator never searches for free space: it always goes "on top".
  Function local variables are pushed on entry and popped on exit.
- **Heap** — less organized. The allocator is asked for `alloc(size)`, finds free space, marks it as
  taken, and returns a pointer. The pointer itself has a known size and can live on the stack, but
  reaching the data requires following it (indirection = possible cache miss).
- **Why does this matter for ownership?** Ownership exists mostly to manage **heap memory** — who
  returns it to the allocator and when. Stack values (`Copy` types) are cheap to duplicate, so Rust
  just copies them. Heap values (`String`, `Vec<T>`, `Box<T>`) have a single owner responsible for
  `drop`, preventing double-free and leaks without a GC.
- Consequence: `let y = x;` behaves differently depending on whether `x`'s data lives on the stack
  (copy) or on the heap (move). This is not arbitrary — it follows directly from who is responsible
  for freeing the heap allocation.
- **Reassignment drops the old value immediately.** Assigning a new value to an existing `mut`
  variable runs `drop` on the old value right away — not at end-of-scope. This is how `mut` works
  for owned heap data: `let mut s = String::from("hello"); s = String::from("ahoy");` frees "hello"
  before "ahoy" is stored in `s`. The old allocation is gone; there is no way to refer to it.
*/

#[allow(unused_variables)]
fn ownership_rules() {
    // Copy: stack types are copied, not moved.
    // Both x and y are independent and valid.
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y} (both valid — i32 is Copy)");

    // Move: assigning a String transfers ownership to s2.
    // s1 is no longer valid after this line.
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{s1}"); // ERROR: value used after move
    println!("s2 owns the string: {s2}");

    // Drop: when a scope ends, owned values are freed
    {
        let temporary = String::from("I live in this block");
        println!("{temporary}");
    }
    // `temporary` is dropped here — its heap memory is freed
    // println!("{temporary}"); // ERROR: not found in this scope

    // Move with a custom struct — any struct containing a non-Copy
    // field (like String) is itself non-Copy
    struct Config {
        name: String,
        debug: bool,
    }
    let c1 = Config {
        name: String::from("prod"),
        debug: false,
    };
    let c2 = c1; // c1 is moved into c2
    // println!("{}", c1.name); // ERROR: value used after move
    println!("config moved: {} (debug={})", c2.name, c2.debug);

    // --- Partial moves ---
    // When destructuring a struct, non-Copy fields are moved out
    // individually. The struct as a whole becomes unusable, but
    // Copy fields that were NOT moved remain accessible.
    let c3 = Config {
        name: String::from("staging"),
        debug: true,
    };
    let Config { name, debug } = c3; // `name` (String) is moved out of c3
    println!("moved name: {name}");
    // println!("{}", c3.name); // ERROR: value used after partial move
    println!("debug still accessible: {}", c3.debug); // bool is Copy — OK

    // Struct update syntax can also cause partial moves:
    let c4 = Config {
        name: String::from("dev"),
        debug: false,
    };
    let c5 = Config {
        debug: true,
        ..c4 // moves c4.name into c5; c4.debug (Copy) stays accessible
    };
    println!("c5: {} (debug={})", c5.name, c5.debug);
    // println!("{}", c4.name); // ERROR: partially moved
    println!("c4.debug still works: {}", c4.debug);

    // Explicit early drop with drop() — frees the value before
    // the scope ends
    let data = String::from("release me early");
    println!("{data}");
    drop(data);
    // println!("{data}"); // ERROR: value used after move (drop consumes it)

    // --- Reassignment drops the old value immediately ---
    // Assigning a new value to an existing `mut` variable runs
    // drop on the previous value *right away*, not at end-of-scope.
    // We observe this with a tiny type whose Drop impl prints.
    #[allow(unused_assignments)] // the final value of `slot` is never read by design
    {
        struct Announcer(&'static str);
        impl Drop for Announcer {
            fn drop(&mut self) {
                println!("  dropping Announcer({})", self.0);
            }
        }
        let mut slot = Announcer("first");
        println!("before reassignment");
        slot = Announcer("second"); // "first" is dropped here, immediately
        println!("after reassignment");
        // "second" is dropped when `slot` leaves this block.
    }

    // --- Custom Drop implementation ---
    // Implement the Drop trait to run cleanup code when a value
    // goes out of scope. Typically only needed for types managing
    // external resources (file handles, network connections, raw pointers).
    struct Droppable {
        label: &'static str,
    }
    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("  dropping: {}", self.label);
        }
    }

    println!("--- drop order demo ---");
    let _a = Droppable { label: "first" };
    let _b = Droppable { label: "second" };
    let _c = Droppable { label: "third" };
    // Drop order: variables are dropped in REVERSE declaration order.
    // Output will be: third, second, first.
    // (Struct fields are dropped in declaration order.)
    println!("--- end of scope, drops happen next ---");

    println!("ownership_rules section executed");
}

// =================================================================================================
// Section 2: Clone
// =================================================================================================

/*
## Clone

- When you need an independent copy of a heap-allocated value, call `.clone()` explicitly.
- `.clone()` performs a **deep copy** — it duplicates both the stack metadata and the heap data,
  creating a new owner.
- After cloning, both the original and the clone are independent; modifying one does not affect the
  other.
- Cloning can be expensive for large data structures, so Rust makes it explicit — you must opt in by
  calling `.clone()`.
- The `Clone` trait can be derived for custom types with `#[derive(Clone)]`.
*/

fn clone_semantics() {
    // Clone a String — both the original and the clone are valid
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2} (independent copies)");

    // Clone a Vec — modifications to one do not affect the other
    let v1 = vec![1, 2, 3];
    let mut v2 = v1.clone();
    v2.push(4);
    println!("v1 = {v1:?} (unchanged)");
    println!("v2 = {v2:?} (modified independently)");

    println!("clone_semantics section executed");
}

// =================================================================================================
// Section 3: Ownership and Functions
// =================================================================================================

/*
## Ownership and Functions

- Passing a heap-allocated value to a function **moves** ownership into the function's parameter.
  The caller's variable becomes invalid after the call.
- A function can **return** an owned value, transferring ownership to the caller.
- A function can **take and return** ownership, allowing the caller to regain it after the function
  operates on the value.
- Passing stack-allocated (`Copy`) types to functions creates a copy — the caller's variable remains
  valid.
- Moving values in and out of functions is cumbersome. References (covered in the next section)
  provide a better approach.
*/

// Takes ownership — the caller's variable becomes invalid
fn take_ownership(vec: Vec<i32>) {
    println!("took ownership: {vec:?}");
    // vec is dropped when this function returns
}

// Returns an owned value — ownership transfers to the caller
fn give_ownership() -> Vec<i32> {
    let a = vec![10, 20, 30];
    a
}

// Takes ownership, modifies, and returns — round-trip transfer
fn take_and_give_back(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(40);
    vec
}

// Stack types are copied — the caller's variable is unaffected
fn stack_function(mut val: i32) {
    val += 100;
    println!("inside stack_function: val = {val}");
}

#[allow(unused_variables)]
fn ownership_and_functions() {
    // Move into function — v1 is invalid after this call
    let v1 = vec![1, 2, 3];
    take_ownership(v1);
    // println!("{v1:?}"); // ERROR: value used after move

    // Receive ownership from a function
    let v2 = give_ownership();
    println!("received ownership: {v2:?}");

    // Round-trip: give ownership, get it back
    let v3 = take_and_give_back(v2);
    // v2 is now invalid; v3 owns the data
    println!("after take_and_give_back: {v3:?}");

    // Stack types are copied — x remains valid after the call
    let x = 42;
    stack_function(x);
    println!("x is still valid: {x}");

    println!("ownership_and_functions section executed");
}

// =================================================================================================
// Section 4: References and Borrowing
// =================================================================================================

/*
## References and Borrowing

- A **reference** (`&T`) lets you access a value without taking ownership. Creating a reference is
  called **borrowing**.
- **Why borrow?** Borrowing avoids unnecessary clones and ownership transfers. When a function only
  needs to read or temporarily modify data, a reference is more efficient than moving or copying.
- An **immutable reference** `&T` allows read-only access. You can have **any number** of immutable
  references to the same value simultaneously.
- A **mutable reference** `&mut T` allows read-write access. You can have **at most one** mutable
  reference to a value at a time.
- **Borrowing rules** (enforced at compile time):
  1. You may have **either** multiple `&T` **or** one `&mut T` — never both at the same time.
  2. References must always be **valid** (no dangling references).
- These rules prevent **data races** (rule 1) and **use-after-free** bugs (rule 2) at compile time.
  A **data race** has three ingredients:
  1. Two or more pointers access the same data simultaneously.
  2. At least one of them writes.
  3. There is no synchronization mechanism.
  Rust's borrow checker refuses programs that satisfy (1) + (2) without (3): you cannot have `&mut
  T` alongside another `&T` or `&mut T` to the same data. The result is that entire classes of
  concurrency bugs are caught before the program even runs.
- A reference's scope extends from where it is created to its **last use** (Non-Lexical Lifetimes —
  NLL), not necessarily to the end of the enclosing block. NLL has been the default borrow checker
  behavior since Rust 2018 edition.
- **Reborrowing**: when you pass a `&mut T` to a function, Rust implicitly creates a temporary
  reborrow (`&mut *ref`) instead of moving the mutable reference. This lets you continue using the
  original `&mut` after the function returns.
- Some methods on owned types **return references** to their data (e.g., `String::as_str()` →
  `&str`, `Vec::first()` → `Option<&T>`, `Vec::get(i)` → `Option<&T>`). The returned reference
  borrows from the owner — its lifetime is tied to the owner's scope.
- Functions can accept **mixed borrows** — e.g., `&T` for read-only context and `&mut T` for mutable
  state — as long as they don't alias the same data.
- **Casting between references**: an immutable `&T` **cannot** be cast to a mutable `&mut T`. A
  mutable reference can be reborrowed as immutable — the `&mut` is temporarily "frozen" and cannot
  be used for mutation until the immutable reborrow ends.
- **Assigning references**: immutable references `&T` implement `Copy` — assigning to another
  variable creates a copy. This is safe because the borrowing rules already allow multiple `&T`.
  Mutable references `&mut T` are **moved** on assignment, not copied — if `&mut T` were Copy, you
  could create two `&mut T` to the same data, violating the exclusivity guarantee.
*/

// Reads a slice without taking ownership (idiomatic: accept &[T], not &Vec<T>)
fn borrow_immutably(vec: &[i32]) {
    println!("borrowed (read-only): {vec:?}");
}

// Modifies a Vec without taking ownership
fn borrow_mutably(vec: &mut Vec<i32>) {
    vec.push(99);
}

fn references_and_borrowing() {
    // --- Multiple immutable references ---
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("r1={r1}, r2={r2}, r3={r3} (multiple immutable refs)");

    // --- Mutable reference ---
    let mut s = String::from("hello");
    let r = &mut s;
    r.push_str(" world");
    println!("modified through &mut: {r}");

    // --- Violation: mutable + immutable cannot coexist ---
    // let mut val = String::from("data");
    // let r1 = &val;          // immutable borrow
    // let r2 = &mut val;      // ERROR: cannot borrow as mutable
    // println!("{r1}");        // because immutable borrow is still in use

    // --- Non-Lexical Lifetimes (NLL) ---
    // Immutable refs end at their last use, so a mutable ref can
    // be created afterward — even within the same scope
    let mut data = vec![1, 2, 3];
    let r1 = &data;
    let r2 = &data;
    println!("immutable: {r1:?}, {r2:?}");
    // r1 and r2 are no longer used — their borrow ends here (NLL)
    let r3 = &mut data;
    r3.push(4);
    println!("mutable (after immutable refs ended): {r3:?}");

    // --- Borrowing in functions ---
    let v = vec![10, 20, 30];
    borrow_immutably(&v);
    println!("still valid after immutable borrow: {v:?}");

    let mut v = vec![10, 20, 30];
    borrow_mutably(&mut v);
    println!("modified by function: {v:?}");

    // --- Reborrowing ---
    // Passing &mut to a function creates a temporary reborrow.
    // The original &mut is usable again after the function returns.
    let mut nums = vec![1, 2, 3];
    let r = &mut nums;
    borrow_mutably(r); // implicit reborrow of r
    r.push(100); // r is still usable after the call
    println!("after reborrow: {r:?}");

    // --- Methods returning references from owned data ---
    let words = vec!["alpha", "beta", "gamma"];
    let first = words.first(); // Option<&&str> — borrows from words
    println!("first element: {first:?}");

    let greeting = String::from("hello world");
    let slice: &str = greeting.as_str(); // borrows the String's data
    println!("as_str: {slice}");

    // --- Mixed borrowing: &T (read) + &mut T (write) in one call ---
    fn log_and_push(label: &str, data: &mut Vec<String>) {
        let entry = format!("[{label}] item #{}", data.len() + 1);
        data.push(entry);
    }
    let mut log = Vec::new();
    log_and_push("info", &mut log);
    log_and_push("warn", &mut log);
    println!("log: {log:?}");

    // --- Casting between references ---
    // A mutable ref can be reborrowed as immutable (frozen temporarily)
    let mut val = 42;
    let r_mut = &mut val;
    let r_imm: &i32 = r_mut; // reborrow as immutable — r_mut is frozen
    println!("reborrowed as immutable: {r_imm}");
    // *r_mut += 1; // ERROR: cannot use r_mut while r_imm exists
    // r_imm's scope ends here (NLL), so r_mut is unfrozen
    *r_mut += 1;
    println!("after unfreeze: {r_mut}");

    // --- Assigning references ---
    // Immutable references are Copy — both r1 and r2 are valid
    let x = 10;
    let r1 = &x;
    let r2 = r1; // Copy — both r1 and r2 point to x
    println!("r1={r1}, r2={r2} (both valid — &T is Copy)");

    // Mutable references are moved, not copied
    let mut y = 20;
    let m1 = &mut y;
    let m2 = m1; // Move — m1 is now invalid
    // println!("{m1}"); // ERROR: m1 was moved
    *m2 += 1;
    println!("m2={m2} (m1 was moved, only m2 is valid)");

    // --- Cannot move out of a reference ---
    // A reference borrows data — it does not own it. Moving the
    // value out would leave the original owner with invalid data.
    let s = String::from("owned");
    let r = &s;
    // let taken = *r; // ERROR: cannot move out of `*r` which is behind a shared reference
    // Solution: clone if you need an independent owned copy
    let cloned = r.clone(); // or (*r).clone() — equivalent via auto-deref
    println!("cloned from ref: {cloned}");

    let x1 = (1, 2);
    let x2 = x1; // copy

    let t1 = (1, String::from("hello"));
    let t2 = t1; // move

    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = p1; // move

    // p1.y; // compilation error
    p2.y;

    #[derive(Copy, Clone)]
    struct Point1 {
        x: i32,
        y: i32,
    }

    let p1 = Point1 { x: 1, y: 2 };
    let p2 = p1; // copy

    p1.y;
    p2.y;

    // #[derive(Copy, Clone)]
    struct Point3 {
        x: i32,
        y: i32,
        name: String
    }

    println!("references_and_borrowing section executed");
}

// =================================================================================================
// Section 5: Dereferencing
// =================================================================================================

/*
## Dereferencing

- The **dereference operator** `*` follows a reference to access the underlying value. It is the
  inverse of `&` (referencing).
- Conceptually: **borrowing** (`&`) is about sharing access, **dereferencing** (`*`) is about using
  the value behind a reference.
- Types starting with `&` (like `&i32`, `&str`) are **borrowed**; types without `&` (like `i32`,
  `String`) are **owned**. Applying `*` to a reference of type `&T` yields a place of type `T` that
  you can read from (and, for `&mut T`, write to). Whether the value can be *moved out* depends on
  whether `T` is `Copy` — see the next two bullets.
- For references to **stack-allocated** (`Copy`) types: `*ref` produces a copy of the value.
  Assigning through `*mut_ref = val` modifies the original variable.
- For references to **heap-allocated** types: you cannot move the value out via `*ref` because that
  would leave the owner with invalid data. Use methods directly on the reference instead.
- **Auto-dereferencing**: Rust automatically inserts `*` as needed when calling methods.
  `ref_to_vec.push(1)` works without writing `(*ref_to_vec).push(1)`.
- **Parentheses matter** when mixing `*` with method calls: `.` binds more tightly than `*`, so
  `*r.method()` dereferences the method's **return value**, not `r`. Write `(*r).method()` to
  dereference first. In practice, auto-deref handles this for you.
*/

fn dereferencing() {
    // Read a value through an immutable reference (copies for Copy types)
    let num = 42;
    let r = &num;
    let copy = *r;
    println!("original: {num}, dereferenced copy: {copy}");

    // Modify through a mutable reference using *
    let mut value = 10;
    let r = &mut value;
    *r = 50;
    println!("modified through *r: {value}");

    // Auto-deref: methods can be called directly on references
    let mut vec = vec![1, 2, 3];
    let r = &mut vec;
    r.push(4); // auto-deref — no need to write (*r).push(4)
    println!("auto-deref push: {r:?}");

    // Explicit (*ref).method() — equivalent to the auto-deref form
    let mut vec = vec![10, 20];
    let r = &mut vec;
    (*r).push(30);
    println!("explicit deref push: {r:?}");

    println!("dereferencing section executed");
}

// =================================================================================================
// Section 6: Lifetimes
// =================================================================================================

/*
## Lifetimes

- A **lifetime** is the scope for which a reference is valid. The compiler tracks lifetimes to
  ensure every reference points to valid data.
- Most lifetimes are **inferred** automatically. Explicit lifetime annotations are needed when the
  compiler cannot determine how the lifetimes of inputs and outputs relate.
- Annotations use the syntax `'a` and are placed after `&`: `&'a str`. They **describe
  relationships** between references — they do not change how long values actually live.
- **Lifetime elision rules** (where the compiler infers lifetimes):
  1. Each reference parameter gets its own lifetime parameter.
  2. If there is exactly one input lifetime, it is assigned to all output lifetimes.
  3. If one of the parameters is `&self` or `&mut self`, its lifetime is assigned to all output
     lifetimes.
- Structs that hold references must annotate the lifetime: `struct Foo<'a> { field: &'a str }`. This
  guarantees the struct cannot outlive the data it references.
- **`'static` lifetime**: the longest possible lifetime — valid for the entire program duration. All
  string literals (`"hello"`) have type `&'static str` because they are embedded in the binary. A
  function can also return `&'static` references to constant values — the compiler promotes literals
  to static storage, so the returned reference is valid regardless of input lifetimes.
- **`'static` is not a band-aid for lifetime errors.** The compiler will occasionally suggest
  `consider using a 'static lifetime` when borrow checking fails. This is almost always the
  **wrong** fix. `'static` means "valid for the entire program run" — very few references genuinely
  meet that bar. When a suggestion to use `'static` appears, the underlying problem is usually a
  dangling reference, a mismatched borrow, or a struct that should own its data instead of borrowing
  it. Fix the real problem (tighten the scope, return an owned value, clone, or restructure the
  caller) rather than papering over it with `'static` — otherwise the constraint propagates to every
  caller and you end up fighting the borrow checker at each new use site.
- **Multiple lifetime parameters**: when a function takes references with independent lifetimes,
  each can have its own parameter (e.g., `'a`, `'b`). The return type is tied to only the relevant
  input lifetime.
- **Returning a reference to a locally-created value is always invalid.** A common mistake: you
  write `fn make_greeting<'a>() -> &'a str { let s = String::from("hi"); s.as_str() }` and assume
  the `'a` annotation gives the compiler enough information. It does not. Lifetime annotations can
  only connect a return reference to **one of the input parameters**; they cannot extend the life of
  a value created inside the function body. The local `String` is dropped at the end of the
  function, leaving the returned slice dangling. The fix is structural, not syntactic: **return an
  owned type** (`String` instead of `&str`) and let the caller decide when to drop it. Returning
  references is only appropriate when the data already lives somewhere the caller owns.
- **Method-level lifetime parameters**: impl methods can introduce their own lifetime parameters
  (e.g., `'b`) separate from the struct's lifetime (`'a`). This lets the return type be tied to the
  borrow of `&self` rather than the stored data's lifetime.
- A **dangling reference** — a reference to data that no longer exists — is rejected by the
  compiler.
*/

// Both parameters share lifetime 'a — the returned reference is
// valid only as long as *both* inputs are valid
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// Two independent lifetimes — the return is tied to 'a only,
// so `label` can have a shorter or longer lifetime independently.
// (Clippy notes that 'b could be elided here; it is written
// explicitly for pedagogical clarity.)
#[allow(clippy::needless_lifetimes)]
fn first_element<'a, 'b>(slice: &'a [i32], _label: &'b str) -> &'a i32 {
    &slice[0]
}

// A struct holding a borrowed string slice — the struct cannot
// outlive the data that `text` refers to
struct Excerpt<'a> {
    text: &'a str,
}

struct B<'b> {
    text: &'b Excerpt<'b>,
}

// Lifetime elision (rule 3): methods with &self — the compiler
// assigns self's lifetime to the output automatically
impl<'a> Excerpt<'a> {
    fn text(&self) -> &str {
        self.text
    }
}

// Returns a 'static reference — the literal 6 is promoted to static
// storage by the compiler, so the result outlives any input lifetime
fn static_value(_a: &i32, _b: &i32) -> &'static i32 {
    &6
}

// Lifetime elision (rule 2): one input reference, so the compiler
// automatically assigns its lifetime to the output — no annotation
// needed
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

#[allow(unused_variables, unused_assignments)]
fn lifetimes() {
    // --- Dangling reference — rejected by the compiler ---
    // The inner variable is dropped when the block ends, leaving
    // the reference pointing to freed memory.
    // NOTE: under NLL the assignment `r = &x` alone compiles — the
    // borrow of `x` only needs to live until `r` is next used. The
    // error appears when you actually *use* `r` after `x` is dropped.
    let r;
    {
        let x = 5;
        r = &x;
    }
    // println!("{r}"); // ERROR: `x` does not live long enough

    // --- Explicit lifetime: longest() ---
    // The result's lifetime is the shorter of the two inputs
    let s1 = String::from("long string");
    let result;
    {
        let s2: &str = "aaa";
        result = longest(s1.as_str(), &s2);
        println!("longest: {result}");
        // result cannot be used after this block because s2
        // (the shorter lifetime) is dropped here
    }
    println!("result: {result}");

    // --- Multiple lifetime parameters ---
    let numbers = vec![10, 20, 30];
    let first;
    {
        let label = String::from("my list");
        first = first_element(&numbers, &label);
        println!("first element: {first} (label: {label})");
        // label is dropped here, but first is still valid because
        // its lifetime is tied to `numbers`, not `label`
    }
    println!("first element still valid: {first}");

    // --- 'static lifetime ---
    // String literals are embedded in the binary and live for the
    // entire program — their type is &'static str
    let s: &'static str = "I live forever";
    println!("'static str: {s}");

    // --- 'static function return ---
    // A function can return &'static by referencing a constant/literal
    // value promoted to static storage — the result is valid regardless
    // of the input lifetimes
    let val;
    {
        let a = 10;
        let b = 20;
        val = static_value(&a, &b);
        // a and b are dropped here, but val is &'static so it survives
    }
    println!("'static return: {val} (inputs already dropped)");

    // --- Struct with lifetime ---
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = Excerpt {
        text: first_word(&novel),
    };
    println!("excerpt: {}", excerpt.text);

    // Struct outliving its data — the compiler prevents this:
    // let excerpt;
    // {
    //     let short_lived = String::from("temporary");
    //     excerpt = Excerpt { text: &short_lived };
    // }
    // println!("{}", excerpt.text);
    // ERROR: `short_lived` does not live long enough

    // --- Lifetime elision (rule 2) ---
    // first_word() has one input reference, so the compiler infers
    // the output lifetime automatically — no 'a annotation needed
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("first word: {word}");

    // --- Lifetime elision (rule 3) ---
    // Methods with &self: the compiler assigns self's lifetime to
    // the output — no explicit annotation needed on text()
    let novel = String::from("It was a bright cold day in April...");
    let excerpt = Excerpt {
        text: first_word(&novel),
    };
    println!("excerpt via method: {}", excerpt.text());

    println!("lifetimes section executed");
}

pub fn run() {
    ownership_rules();
    clone_semantics();
    ownership_and_functions();
    references_and_borrowing();
    dereferencing();
    lifetimes();
}
