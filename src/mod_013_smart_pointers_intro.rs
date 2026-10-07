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

- Rust must know the **size** of every type at compile time. For an `enum`, the size is at least
  that of its **largest variant**, usually plus a discriminant tag and rounded up to the alignment.
  The compiler can elide that tag via *niche optimization* when a variant has an unused bit
  pattern — which is why `Option<Box<T>>` is exactly the size of `Box<T>`, with no extra tag.
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
// Section 3: Deref Trait & Custom Smart Pointer
// =================================================================================================

/*
## `Deref` Trait & Custom Smart Pointer

- The dereference operator `*` was introduced in module 006. For plain references (`&T`), `*r`
  simply follows the address and yields the pointed-to value.
- For smart pointers, `*` is powered by the **`Deref` trait**. Implementing it tells the compiler
  how to turn a smart pointer into a reference to its inner value.
- The trait has one associated type and one method:
  ```rust
  trait Deref {
      type Target: ?Sized;
      fn deref(&self) -> &Self::Target;
  }
  ```
- When you write `*y`, the compiler actually expands this to `*(y.deref())` — `deref()` returns a
  `&Target`, and then the regular `*` on the resulting reference produces the inner value.
- `deref()` returns `&T` (not `T`) specifically so it does **not** move the value out of the smart
  pointer. The smart pointer retains ownership; the caller only gets a borrow.
- A mutable counterpart, **`DerefMut`**, provides `deref_mut(&mut self) -> &mut Target` and powers
  `*y = ...` on a mutable smart pointer. `DerefMut` requires `Deref` as a supertrait.
*/

// A tiny smart pointer around a single value. It is a tuple
// struct so we can reach the inner value with `self.0`.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn deref_trait() {
    let x = 5;
    let y = MyBox::new(x);

    // *y expands to *(y.deref()) — deref() returns &i32, then * yields i32.
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("*y = {}", *y);

    // Works for any inner type, including heap-allocated values.
    let message = MyBox::new(String::from("inside MyBox"));
    println!("inner string length: {}", message.len()); // auto-deref to &String, then String::len

    println!("deref_trait section executed");
}

// =================================================================================================
// Section 4: Deref Coercion
// =================================================================================================

/*
## Deref Coercion

- **Deref coercion** converts a reference to one type into a reference to another type by chaining
  `Deref` implementations. For example, `&String` can become `&str` because `String` implements
  `Deref<Target = str>`.
- This happens **automatically** at function-call and method-call sites. It lets the same function
  signature accept references to owned values, smart pointers, and custom wrappers alike — with no
  manual conversion from the caller.
- Deref coercion chains as deeply as needed. A call with `&MyBox<String>` can coerce
  `&MyBox<String>` → `&String` → `&str`, so a function taking `&str` accepts the `MyBox` directly.
- The compiler performs three reference coercions through `Deref`:
  1. `&T` → `&U` when `T: Deref<Target = U>`
  2. `&mut T` → `&mut U` when `T: DerefMut<Target = U>`
  3. `&mut T` → `&U` when `T: Deref<Target = U>`
- Notice the asymmetry: a **mutable** reference can coerce to an **immutable** one, but not the
  other way around. Converting `&T` into `&mut T` would break Rust's borrowing guarantees — there
  could be other `&T` around, and promoting one of them to `&mut T` would create aliasing mutable
  access.
- Deref coercion is resolved entirely at **compile time**; there is no runtime cost.
*/

fn hello(name: &str) {
    println!("hello, {name}!");
}

fn deref_coercion() {
    let m = MyBox::new(String::from("Rust"));

    // Without deref coercion you would have to write:
    //   hello(&(*m)[..]);
    // Deref coercion handles the conversion chain for you:
    //   &MyBox<String>  --Deref-->  &String  --Deref-->  &str
    hello(&m);

    // Works just as well for a plain reference:
    let s = String::from("World");
    hello(&s);

    println!("deref_coercion section executed");
}

// =================================================================================================
// Section 5: The Drop Trait & Early Drop
// =================================================================================================

/*
## The `Drop` Trait & Early Drop

- `Drop` was introduced in module 006 as the mechanism that runs cleanup code when a value goes out
  of scope. Smart pointers rely on `Drop` to release their backing resource — for example, `Box<T>`
  frees its heap allocation inside its `drop` method.
- The trait is:
  ```rust
  trait Drop {
      fn drop(&mut self);
  }
  ```
  You do not call `drop` yourself — the compiler inserts a call at the end of the owning scope.
- You are **not allowed** to call `value.drop()` explicitly. The compiler rejects it with error
  `E0040` ("explicit use of destructor method"). If you could call `drop` manually, the compiler
  would still insert the automatic call at the end of scope, resulting in a **double free**.
- To drop a value **early** — for instance, to release a lock before a long computation — use the
  free function `std::mem::drop` (re-exported in the prelude simply as `drop`). It takes the value
  by value (`fn drop<T>(_: T)`), so ownership moves into it and the destructor runs immediately.
*/

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn drop_and_early_drop() {
    let _c = CustomSmartPointer {
        data: String::from("first"),
    };
    let d = CustomSmartPointer {
        data: String::from("second"),
    };
    println!("CustomSmartPointers created");

    // Calling d.drop() directly is a compile error:
    //   d.drop(); // ERROR[E0040]: explicit use of destructor method
    // Use std::mem::drop (in scope via the prelude as `drop`) to
    // drop `d` early — useful when a resource must be released
    // before the natural end of scope.
    drop(d);
    println!("d was explicitly dropped before the end of scope");

    // _c is dropped automatically when this function returns.
    println!("drop_and_early_drop section executed");
}

// =================================================================================================
// Section 6: Rc<T> — Reference Counted Smart Pointer
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
// Section 7: RefCell<T> & Interior Mutability
// =================================================================================================

/*
## `RefCell<T>` & Interior Mutability

- **Interior mutability** is a pattern that lets you mutate data even through an immutable reference
  to its container. It is implemented with types that wrap the data and enforce borrow rules at a
  different time than normal.
- Recall the borrow rules from module 006:
  1. Either one `&mut T` or any number of `&T` at a time.
  2. References must always be valid.
  The Rust compiler normally enforces these rules **at compile time**. That is strict but sometimes
  too conservative — some programs are memory-safe yet the compiler cannot prove it.
- **`RefCell<T>`** enforces the same rules, but **at runtime**. If you violate them, the program
  panics instead of failing to compile.
  - `borrow()` returns a `Ref<T>` (an immutable borrow guard).
  - `borrow_mut()` returns a `RefMut<T>` (a mutable borrow guard).
  - While any `RefMut` exists, another `borrow()` or `borrow_mut()` will panic. While any `Ref`
    exists, `borrow_mut()` will panic.
- `RefCell<T>` is single-threaded only (use `Mutex<T>` / `RwLock<T>` for multi-threaded interior
  mutability).
- A classic use case is a **mock object**. A testing fake may need to record messages it received,
  but the trait it implements only gives it `&self`. Wrapping the log in a `RefCell` lets the fake
  push to the log from `&self` methods.

### When to pick which smart pointer

| Type          | Owners    | Mutation           | Checking      |
| ------------- | --------- | ------------------ | ------------- |
| `Box<T>`      | single    | via `&mut` only    | compile time  |
| `Rc<T>`       | multiple  | immutable only     | compile time  |
| `RefCell<T>`  | single    | through `&self`    | runtime       |

`Rc<T>` and `RefCell<T>` are both single-threaded.
*/

trait Messenger {
    fn send(&self, msg: &str);
}

// A testing double that records every message it "sent".
// `send` only has `&self`, so we need interior mutability to
// push into the log.
struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        self.sent_messages.borrow_mut().push(String::from(msg));
    }
}

// Demonstrates the runtime panic from two simultaneous
// `borrow_mut()` calls. NOT called from `run()` to keep the
// demo output clean — left here as a reference example.
#[allow(dead_code)]
fn refcell_runtime_panic_demo() {
    let cell = RefCell::new(vec![1, 2, 3]);
    let _first = cell.borrow_mut();
    let _second = cell.borrow_mut(); // panics: already borrowed: BorrowMutError
}

fn refcell_and_interior_mutability() {
    // --- Mock messenger ---
    let mock = MockMessenger::new();
    mock.send("warning: 75% of quota used");
    mock.send("alert: 90% of quota used");

    // borrow() for read access — returns a Ref<Vec<String>>
    let log = mock.sent_messages.borrow();
    println!("messages recorded: {}", log.len());
    for (i, msg) in log.iter().enumerate() {
        println!("  {i}: {msg}");
    }
    // `log` is a Ref — the RefCell is still considered immutably
    // borrowed until `log` goes out of scope.
    drop(log);

    // --- Direct mutation through &RefCell ---
    let cell = RefCell::new(vec![10, 20]);
    cell.borrow_mut().push(30); // temporary RefMut released at end of expression
    println!("cell contents: {:?}", cell.borrow());

    println!("refcell_and_interior_mutability section executed");
}

// =================================================================================================
// Section 8: Combining Rc<T> with RefCell<T>
// =================================================================================================

/*
## Combining `Rc<T>` with `RefCell<T>`

- `Rc<T>` alone gives you **shared ownership** but only of **immutable** data. `RefCell<T>` alone
  gives you **interior mutability** but only a **single owner**.
- Wrapping a `RefCell<T>` inside an `Rc<T>` combines the two: **multiple owners of a value that can
  still be mutated**. This is a very common pattern for building graphs and other shared-state data
  structures in single-threaded code.
- Mutation goes through `borrow_mut` on the inner cell: `*shared.borrow_mut() += 10;`. Every owner
  sees the change, because they all share the same `RefCell`.
*/

#[derive(Debug)]
#[allow(dead_code)]
enum SharedList {
    Cons(Rc<RefCell<i32>>, Rc<SharedList>),
    Nil,
}

fn rc_with_refcell() {
    use SharedList::{Cons, Nil};

    // A mutable head value shared between several lists.
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("before: a = {a:?}");
    println!("before: b = {b:?}");
    println!("before: c = {c:?}");

    // Mutate the shared value once. Both b and c (which both
    // point at a, which holds `value`) observe the update.
    *value.borrow_mut() += 10;

    println!("after:  a = {a:?}");
    println!("after:  b = {b:?}");
    println!("after:  c = {c:?}");

    println!("rc_with_refcell section executed");
}

// =================================================================================================
// Section 9: Reference Cycles & Weak<T>
// =================================================================================================

/*
## Reference Cycles & `Weak<T>`

- Rust's ownership model prevents dangling references, but it does **not** prevent memory leaks. If
  two `Rc`s end up pointing at each other — for example, through a `RefCell<Rc<T>>` field — their
  strong counts will never drop to zero, and the data they own will never be freed. This is a
  **reference cycle**.
- Visually:
  ```text
  A ──Rc──▶ B
  ▲         │
  └───Rc────┘
  ```
  Even when all external references to `A` and `B` are gone, each still holds one `Rc` to the other,
  so both counts stay at 1 and neither is dropped.
- The fix is **`Weak<T>`**: a non-owning reference counted pointer. Creating a `Weak` via
  `Rc::downgrade(&rc)` does **not** increment the strong count — it increments a separate **weak
  count** instead.
- `Weak<T>` does not let you access the value directly. Call `.upgrade()`, which returns
  `Option<Rc<T>>`: `Some` if the value is still alive (strong count > 0), `None` if it has been
  dropped. This makes `Weak<T>` safe — it can never cause a dangling reference.
- The canonical use case is a **parent → child** graph. Parents own their children strongly via
  `Rc`; children refer back to their parent via `Weak`. Dropping the parent frees
  the tree cleanly; children simply see their parent reference upgrade to `None`.
*/

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn reference_cycles_and_weak() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), // no parent yet
        children: RefCell::new(vec![]),
    });

    // Before a parent is assigned, upgrade() returns None.
    println!(
        "leaf parent before link: {:?}",
        leaf.parent.borrow().upgrade()
    );
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]), // strong edge
        });

        // leaf --weak--> branch: does NOT increase branch's strong count.
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        println!(
            "leaf   strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        // Upgrading yields an Rc<Node> we can read through.
        if let Some(parent) = leaf.parent.borrow().upgrade() {
            println!("leaf's parent value = {}", parent.value);
        }
        // `branch` drops here — its strong count hits 0 and the
        // Node is freed. `leaf.parent` still holds a Weak, but
        // upgrading it will now return None.
    }

    println!(
        "after branch drop, leaf parent = {:?}",
        leaf.parent.borrow().upgrade()
    );
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!("reference_cycles_and_weak section executed");
}

// =================================================================================================
// Public entry point
// =================================================================================================

pub fn run() {
    smart_pointers_overview();
    recursive_types();
    deref_trait();
    deref_coercion();
    drop_and_early_drop();
    rc_smart_pointer();
    refcell_and_interior_mutability();
    rc_with_refcell();
    reference_cycles_and_weak();
}
