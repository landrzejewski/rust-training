use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

// =================================================================================================
// Section 1: Smart Pointers Overview & Box<T>
// =================================================================================================

/*
## Smart Pointers Overview & `Box<T>`

- A **pointer** is a value that contains an address referring to some other data. The most common
  pointer in Rust is the reference (`&T`) — it borrows a value without taking ownership and has no
  runtime cost beyond the address itself.
- **Smart pointers**, in contrast, are data structures that act like pointers but also carry
  additional metadata and capabilities. Unlike `&T`, most smart pointers **own** the data they point
  to.
- `String` and `Vec<T>` are already smart pointers you have used — they own heap memory, track
  capacity and length, and free their data on drop.
- Two traits make a type feel like a pointer:
  - **`Deref`** — allows an instance to be used with `*` and enables *deref coercion*, so smart
    pointers can be treated like references in most contexts.
  - **`Drop`** — defines what happens when the smart pointer goes out of scope (releasing heap
    memory, closing a file, unlocking a mutex, etc.).
- **`Box<T>`** is the simplest smart pointer: it stores data on the **heap** while the `Box` value
  itself (just a pointer) lives on the stack. Use `Box<T>` when:
  1. The size of a type cannot be known at compile time (e.g., recursive types — see Section 2).
  2. You have a large value and want to transfer ownership without copying the data.
  3. You want to own a value through a trait object (`Box<dyn Trait>`) — covered in module 008.
- Creating a box is straightforward: `Box::new(value)`. Reading the inner value uses the dereference
  operator: `*boxed`.
*/

fn smart_pointers_overview() {
    // Allocate an i32 on the heap — the Box itself lives on the stack
    // and holds a pointer to the heap data.
    let boxed = Box::new(5);
    println!("boxed value: {boxed}");

    // Dereference to read the inner i32. Box<T> implements Deref,
    // so `*boxed` yields the underlying i32.
    let sum = *boxed + 10;
    println!("sum through deref: {sum}");

    // Transfer ownership of a large value without copying the payload.
    // Only the Box (pointer + metadata) moves; the heap data stays put.
    let large = Box::new([0_i32; 1000]);
    let moved = large;
    println!("moved Box with len {}", moved.len());

    // The Box is freed automatically when it goes out of scope —
    // Drop is implemented for Box<T> to deallocate the heap memory.

    println!("smart_pointers_overview section executed");
}

// =================================================================================================
// Section 2: Recursive Types with Box<T>
// =================================================================================================

/*
## Recursive Types with `Box<T>`

- Rust must know the **size** of every type at compile time. For an `enum`, the size is determined
  by its **largest variant** plus a discriminant tag.
- A naive recursive enum like `enum List { Cons(i32, List), Nil }` has no fixed size — each `Cons`
  would contain another `List`, which contains another `Cons`, and so on forever. The compiler
  rejects this with "recursive type has infinite size".
- The fix is **indirection**: store the recursive field behind a pointer. The pointer has a fixed,
  known size (one machine word), regardless of what it points to.
- `Box<T>` is the canonical way to add this indirection for an owned recursive value. The enum
  becomes `enum List { Cons(i32, Box<List>), Nil }` — now each variant has a known size: either a
  tag alone (`Nil`) or a tag, an `i32`, and a pointer (`Cons`).
- This classic example is a **cons list**, borrowed from Lisp: each node holds a value and a pointer
  to the rest of the list.
*/

#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn recursive_types() {
    use List::{Cons, Nil};

    // Build 1 -> 2 -> 3 -> Nil. Each Cons owns its tail through a Box.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("cons list: {list:?}");

    // The following would not compile because the size of List
    // would be infinite without indirection:
    // enum BadList { Cons(i32, BadList), Nil } // ERROR: recursive type

    println!("recursive_types section executed");
}

// =================================================================================================
// Section 3: Rc<T> — Reference Counted Smart Pointer
// =================================================================================================

/*
## `Rc<T>` — Reference Counted Smart Pointer

- Ownership in Rust is usually **single**: one value, one owner. Sometimes, though, a value needs to
  be shared — for example, multiple parts of a graph may all "own" the same node, and none of them
  should drop it while another is still using it.
- **`Rc<T>`** (Reference Counted) is a smart pointer that enables **multiple owners** of the same
  heap-allocated value. It keeps a count of the number of references to the data and frees the value
  only when the count drops to zero.
- Use `Rc::new(value)` to create the first owner, and `Rc::clone(&rc)` to create another owner.
  `Rc::clone` does **not** deep-copy the inner data — it just increments the reference count, which
  is cheap.
- The idiomatic form is `Rc::clone(&a)` rather than `a.clone()`. Both do the same thing, but using
  the associated function makes it visually obvious that the operation is an O(1) refcount bump, not
  a potentially expensive deep clone.
- `Rc::strong_count(&rc)` returns the current refcount. Useful for demonstrations and debugging.
- `Rc<T>` is only for **single-threaded** code. Incrementing the count is not thread-safe. For
  multi-threaded sharing, use the atomic variant `Arc<T>` (covered in the advanced concurrency
  module).
*/

// A cons list variant whose tail is shared via Rc.
#[derive(Debug)]
#[allow(dead_code)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

fn rc_smart_pointer() {
    use RcList::{Cons, Nil};

    // Build a shared tail: 5 -> 10 -> Nil
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    // b and c both prepend a new head and share `a` as their tail.
    // With Box<T> this would fail: constructing `c` would try to
    // move `a` a second time after it was already moved into `b`.
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        // _c goes out of scope here — refcount decreases.
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    println!("rc_smart_pointer section executed");
}

// =================================================================================================
// Public entry point
// =================================================================================================

pub fn run() {
    smart_pointers_overview();
    recursive_types();
    rc_smart_pointer();
}
