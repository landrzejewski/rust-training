use std::cell::Cell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Barrier, Condvar, Mutex, RwLock, mpsc};
use std::thread;
use std::time::{Duration, Instant};

// ============================================================
// Section 1: Launching and Coordinating Threads
// ============================================================

/*
## Launching and Coordinating Threads

- Rust uses **OS threads** (1:1 model) — each `thread::spawn` call
  creates a real operating system thread with its own stack.
- `thread::spawn` takes a closure with three bounds:
  - `FnOnce` — the closure is called exactly once (when the thread
    starts). It may consume captured variables.
  - `Send` — everything the closure captures must be safe to transfer
    to another thread. This is checked at compile time.
  - `'static` — the closure and its captured data must live for the
    entire duration of the thread (which could outlive the spawning
    scope). This is why you cannot borrow local variables directly —
    use `move` closures to transfer ownership.
- `thread::spawn` returns a `JoinHandle<T>` where `T` is the
  closure's return type.
  - `handle.join()` blocks the calling thread until the spawned
    thread finishes. It returns `Result<T, Box<dyn Any + Send>>`.
  - `Ok(value)` — the thread completed normally, returning `value`.
  - `Err(payload)` — the thread panicked. The panic payload is
    returned, and the calling thread can handle it gracefully
    instead of propagating the panic.
- Dropping a `JoinHandle` without calling `join()` **detaches** the
  thread — it keeps running in the background but can no longer be
  joined. When the main thread exits, all detached threads are
  terminated immediately.
- `thread::Builder` provides fine-grained control:
  - `.name("...")` — sets a thread name visible in debuggers and
    `thread::current().name()`.
  - `.stack_size(bytes)` — overrides the default stack size.
  - `.spawn(closure)` returns `io::Result<JoinHandle<T>>` (may fail
    if the OS refuses to create the thread).
- `thread::current()` returns a handle to the calling thread.
  `.id()` returns a unique `ThreadId`, `.name()` returns
  `Option<&str>`.
- `thread::sleep(duration)` puts the current thread to sleep for at
  least the specified duration. It is a real OS-level sleep and
  blocks the thread entirely.

### Arc — Atomic Reference Counting

- When multiple threads need shared access to the same data, you
  need `Arc<T>` (Atomic Reference Counted). It is the thread-safe
  counterpart of `Rc<T>`.
- `Rc<T>` is `!Send` — its reference count uses non-atomic
  operations, so it cannot be safely shared across threads. The
  compiler will reject any attempt to send an `Rc` to another
  thread.
- `Arc::new(value)` creates a new reference-counted pointer.
  `Arc::clone(&arc)` increments the reference count atomically.
  When the last `Arc` is dropped, the value is deallocated.
- `Arc<T>` is `Send + Sync` when `T: Send + Sync`. This means you
  can both send it to other threads and share references to it.
- `Arc` provides **shared read-only access**. For shared mutable
  access, combine it with interior mutability: `Arc<Mutex<T>>` or
  `Arc<RwLock<T>>` (see Section 3).
- Prefer `Arc::clone(&arc)` over `arc.clone()` — the explicit form
  makes it clear you are cloning the pointer (cheap), not the
  underlying data (potentially expensive).

### Output Locking

- `println!` uses `std::io::Stdout::lock()` internally to ensure its
  output is not interrupted by another thread's `println!`. Each
  `println!` waits for any concurrent `println!` to finish before
  writing. Without this, output from multiple threads could be
  interleaved mid-line in garbled fashion.

### Arc Clone Shadowing Idiom

- When spawning multiple threads that each need their own `Arc`
  clone, naming each clone (`arc_clone1`, `arc_clone2`, ...) is
  cluttered. Instead, open a new scope before each `move` closure
  and shadow the variable:
  ```text
  thread::spawn({
      let data = Arc::clone(&data);  // shadow `data` in this scope
      move || { /* use data */ }
  });
  ```
  The outer `data` remains available for the next thread's clone.
  This is the idiomatic pattern in production Rust code.
*/

fn launching_and_coordinating_threads() {
    // --- Basic spawn and join ---
    // Spawn a thread that computes a value and returns it
    let handle = thread::spawn(|| 21 * 2);
    // join() blocks until the thread completes and returns the value
    let value = handle.join().unwrap();
    println!("thread returned: {value}");

    // --- Returning ownership from a thread ---
    // The move closure takes ownership of `data`; the thread returns
    // the processed result
    let data = [1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        let sum: i32 = data.iter().sum();
        sum
    });
    // `data` is no longer accessible here — it was moved into the thread
    let sum = handle.join().unwrap();
    println!("sum computed by thread: {sum}");

    // --- Thread Builder with a name ---
    let handle = thread::Builder::new()
        .name("worker-1".to_owned())
        .spawn(|| {
            let name = thread::current().name().unwrap().to_owned();
            let id = thread::current().id();
            format!("hello from '{name}' (id: {id:?})")
        })
        .expect("failed to spawn thread");
    println!("{}", handle.join().unwrap());

    // --- Multiple threads with JoinHandle collection ---
    let mut handles = Vec::new();
    for i in 0..5 {
        handles.push(thread::spawn(move || i * i));
    }
    let squares: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("squares from 5 threads: {squares:?}");

    // --- Sharing read-only data with Arc ---
    // Two threads read from the same shared Vec without any mutex
    let shared_data = Arc::new(vec![10, 20, 30]);
    let p1 = shared_data.clone();
    let p2 = shared_data.clone();

    let h1 = thread::spawn(move || p1.iter().sum::<i32>());
    let h2 = thread::spawn(move || p2.iter().max().copied());
    println!(
        "Arc shared: sum={}, max={}",
        h1.join().unwrap(),
        h2.join().unwrap().unwrap()
    );
    // The original Arc is still valid — the spawned threads got clones
    println!("original Arc still accessible: {shared_data:?}");

    // --- Arc clone shadowing idiom ---
    // Instead of arc_clone1, arc_clone2, ... shadow the name in a
    // new scope before each move closure
    let data = Arc::new(vec![1, 2, 3]);
    let h1 = thread::spawn({
        let data = Arc::clone(&data); // shadow in a new scope
        move || data.iter().sum::<i32>()
    });
    let h2 = thread::spawn({
        let data = Arc::clone(&data); // same name, different clone
        move || data.len()
    });
    println!(
        "Arc shadowing idiom: sum={}, len={}",
        h1.join().unwrap(),
        h2.join().unwrap()
    );
    // `data` (the original Arc) is still usable here
    println!("original Arc after shadowing: {data:?}");

    // --- Handling a panicking thread ---
    let handle = thread::spawn(|| {
        panic!("something went wrong in the thread");
    });
    match handle.join() {
        Ok(_) => println!("thread completed normally"),
        Err(payload) => {
            // The panic payload is Box<dyn Any + Send>
            if let Some(msg) = payload.downcast_ref::<&str>() {
                println!("thread panicked with: {msg}");
            }
        }
    }
}

// ============================================================
// Section 2: Send and Sync — Thread Safety at Compile Time
// ============================================================

/*
## Send and Sync — Thread Safety at Compile Time

Rust's type system prevents data races at compile time through two
marker traits: `Send` and `Sync`. These are **auto-traits** — the
compiler automatically implements them for types whose fields are
all `Send`/`Sync`. You never need to explicitly opt in for standard
composed types. For manually implementing `Send`/`Sync` on types
with raw pointers (using `unsafe impl`), see the unsafe module.

### `Send`

- A type is `Send` if it can be safely **transferred** (moved) to
  another thread.
- `thread::spawn` requires the closure to be `Send` — this is the
  compiler's way of ensuring everything you move into a new thread
  is thread-safe to transfer.
- **Most types are `Send`**: `i32`, `String`, `Vec<T>` (when T:
  Send), `Arc<T>` (when T: Send + Sync), `Mutex<T>` (when T: Send).
- **Notable `!Send` types**:
  - `Rc<T>` — its reference count is non-atomic; sending it to
    another thread could cause a data race on the refcount.
  - Raw pointers (`*const T`, `*mut T`) — no safety guarantees.
  - `MutexGuard<'_, T>` — `!Send` on **all** platforms (not just
    some). POSIX requires a mutex to be unlocked by the same thread
    that locked it, and Rust enforces this unconditionally by making
    `MutexGuard` `!Send`.

### `Sync`

- A type is `Sync` if a `&T` (shared reference) can be safely
  **shared** between threads.
- Formally: `T` is `Sync` if and only if `&T` is `Send`. If you can
  safely send a reference to another thread, the type is `Sync`.
- **`!Sync` types** — types with unsynchronized interior mutability:
  - `Cell<T>` — allows mutation through `&self` without any locking.
  - `RefCell<T>` — runtime borrow checking, not thread-safe.
  - `UnsafeCell<T>` — the primitive building block for interior
    mutability, has no synchronization.

### Key Relationships

- `Arc<T>` requires `T: Send + Sync` to be `Send + Sync` itself.
  This makes sense: `Arc` is shared across threads (Sync) and the
  last dropping thread deallocates (Send).
- `Mutex<T>` is `Sync` when `T: Send` — the mutex adds the
  synchronization that `T` lacks. This is the **key insight**: you
  can put a `!Sync` type like `Cell<i32>` inside a `Mutex` and
  safely share it across threads via `Arc<Mutex<Cell<i32>>>`.
- `RwLock<T>` is `Sync` when `T: Send + Sync`.

### Common Types — Send and Sync Status

- `i32`, `String`, `Vec<T>` — `Send + Sync`
- `Rc<T>` — `!Send`, `!Sync`
- `Arc<T>` — `Send + Sync` (when `T: Send + Sync`)
- `Cell<T>`, `RefCell<T>` — `Send`, `!Sync`
- `Mutex<T>` — `Send + Sync` (when `T: Send`)
- `MutexGuard<'_, T>` — `!Send`, `Sync` (when `T: Sync`)
*/

fn send_and_sync() {
    // Compile-time assertion helpers — these functions exist only
    // to verify trait bounds. If a type does not satisfy the bound,
    // the code won't compile.
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    // --- Standard types are Send + Sync ---
    assert_send::<i32>();
    assert_sync::<i32>();
    assert_send::<String>();
    assert_sync::<String>();
    assert_send::<Vec<i32>>();
    assert_sync::<Vec<i32>>();
    println!("i32, String, Vec<i32>: Send + Sync");

    // --- Arc<T> is Send + Sync when T: Send + Sync ---
    assert_send::<Arc<String>>();
    assert_sync::<Arc<String>>();
    println!("Arc<String>: Send + Sync");

    // --- Rc<T> is neither Send nor Sync ---
    // Uncommenting these would cause a compile error:
    // assert_send::<std::rc::Rc<i32>>();  // ERROR: Rc<i32> is not Send
    // assert_sync::<std::rc::Rc<i32>>();  // ERROR: Rc<i32> is not Sync
    println!("Rc<i32>: !Send, !Sync (compile error if asserted)");

    // --- Cell<T> is Send but not Sync ---
    assert_send::<Cell<i32>>();
    // assert_sync::<std::cell::Cell<i32>>();  // ERROR: Cell is !Sync
    println!("Cell<i32>: Send, !Sync");

    // --- Mutex<T> adds synchronization → makes T effectively Sync ---
    // Cell<i32> is !Sync, but Mutex<Cell<i32>> IS Sync
    assert_send::<Mutex<Cell<i32>>>();
    assert_sync::<Mutex<Cell<i32>>>();
    println!("Mutex<Cell<i32>>: Send + Sync (Mutex adds synchronization)");

    // --- Practical demonstration: Rc fails in thread::spawn ---
    // This would not compile because Rc is !Send:
    // let rc = std::rc::Rc::new(42);
    // thread::spawn(move || {
    //     println!("{}", rc);  // ERROR: Rc<i32> cannot be sent between threads
    // });

    // But Arc works perfectly:
    let arc = Arc::new(42);
    let arc_clone = Arc::clone(&arc);
    let handle = thread::spawn(move || {
        println!("value from Arc in another thread: {arc_clone}");
    });
    handle.join().unwrap();

    // --- Arc<Mutex<T>> pattern for shared mutable non-Sync data ---
    // Even though Cell<i32> is !Sync, wrapping it in Mutex makes it
    // safe to share across threads
    let shared = Arc::new(Mutex::new(Cell::new(0)));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let guard = shared_clone.lock().unwrap();
        guard.set(42);
    });
    handle.join().unwrap();
    let guard = shared.lock().unwrap();
    println!(
        "Cell inside Mutex, modified by another thread: {}",
        guard.get()
    );
}

// ============================================================
// Section 3: Safe State Sharing — Atomics, Mutex, and RwLock
// ============================================================

/*
## Safe State Sharing — Atomics, Mutex, and RwLock

When multiple threads need to read and write shared data, Rust
provides three mechanisms: atomic types, `Mutex`, and `RwLock`.

### Atomic Types

Atomic types (`AtomicBool`, `AtomicI32`, `AtomicU64`, `AtomicUsize`,
etc.) provide lock-free, thread-safe operations on individual values
using hardware-level atomic instructions.

Atomic operations are the fundamental building block for all
concurrency primitives. `Mutex`, `Condvar`, `Arc`, channels — they
are all implemented using atomic operations under the hood.

**Operations:**
- `load(ordering)` — reads the current value.
- `store(value, ordering)` — writes a new value.
- `swap(val, ordering)` — stores a new value and returns the
  previous value. Conceptually "fetch_store".
- `fetch_add(val, ordering)` — adds and returns the previous value.
  Note: `fetch_add` and `fetch_sub` implement **wrapping** on
  overflow — they silently wrap around instead of panicking, unlike
  the `+` and `-` operators on regular integers in debug mode.
- `fetch_sub`, `fetch_or`, `fetch_and`, `fetch_nand`, `fetch_xor`
  — analogous read-modify-write operations.
- `fetch_max(val, ordering)`, `fetch_min(val, ordering)` — keeps a
  running maximum or minimum. Useful for tracking peak statistics
  across threads.
- `compare_exchange(current, new, success_ord, failure_ord)` —
  atomically sets the value to `new` if it currently equals
  `current`. Returns `Ok(previous)` on success, `Err(actual)` on
  failure. This is the basis for lock-free algorithms (CAS loops).
- `compare_exchange_weak` — same semantics but may spuriously fail
  on some architectures (ARM). Use in a loop for better performance
  on those platforms.
- `fetch_update(success_ord, failure_ord, f)` — convenience wrapper
  for the CAS-retry-loop pattern. Applies closure `f` to the current
  value, then uses `compare_exchange_weak` in a loop to store the
  result. Returns `Ok(previous)` or `Err(current)` if `f` returns
  `None`.

**ABA problem:** `compare_exchange` succeeds if the value matches
`expected`, even if it was changed A→B→A in between. This is
irrelevant for simple counters but can be a problem in pointer-based
lock-free data structures where the meaning of an address may change
even though the address itself is reused.

### Memory Ordering

Every atomic operation requires an `Ordering` parameter that
controls how the operation is visible relative to other memory
operations across threads.

- **`Relaxed`** — guarantees atomicity but provides no ordering
  guarantees relative to other operations. Perfect for simple
  counters or statistics where you only care that the increment
  itself is atomic, not about the order of surrounding reads/writes.
- **`Acquire`** — used on **load** operations. Guarantees that all
  reads and writes *after* this load see the effects of operations
  that happened *before* the corresponding `Release` store. Think
  of it as "acquire the data published by a Release store".
- **`Release`** — used on **store** operations. Guarantees that all
  reads and writes *before* this store are visible to any thread
  that does an `Acquire` load of this atomic. Think of it as
  "release/publish my preceding work".
- **`AcqRel`** — combines `Acquire` and `Release`. Used on
  read-modify-write operations (`fetch_add`, `compare_exchange`)
  that both read and write. The read part has Acquire semantics,
  the write part has Release semantics.
- **`SeqCst`** (Sequentially Consistent) — the strongest ordering.
  All `SeqCst` operations across all threads appear to happen in a
  single, globally agreed-upon order. Highest overhead but simplest
  to reason about. Use when multiple atomics must be observed
  consistently or when in doubt.

**Practical guidance:**

**Relaxed — “I don’t care about timing, just don’t break the number”**
  * Only guarantees the operation itself is atomic (no torn reads/writes)
  * No guarantees about when other threads see the result
  * Example: counting page views — timing doesn’t matter, only correctness
**Release — “I’m done writing, others can now see my work”**
  * Used for write (store) operations
  * All operations before this become visible to other threads that synchronize
  * Think: “I’ve finished preparing the data — now I publish it”
**Acquire — “I’m reading, give me the latest published data”**
  * Used for read (load) operations
  * Ensures you see everything that was written before a corresponding Release
  * Think: “I’m grabbing the data — make sure I see everything the writer finished”
**AcqRel — “I’m doing both (read + write) safely”**
  * Used for read-modify-write operations (e.g. `fetch_add`, `compare_exchange`)
  * Acts as Acquire on the read part
  * Acts as Release on the write part
  * Think: “I read the current value and update it safely”
**SeqCst — “Everyone must agree on one global order”**
  * Strongest and simplest ordering
  * All threads observe operations in the same global order
  * Easiest to reason about
  * Slightly slower than other orderings


### `Mutex<T>`

- `Mutex::new(value)` creates a mutex wrapping a value.
- `mutex.lock()` acquires the lock, blocking if another thread holds
  it. Returns `LockResult<MutexGuard<T>>`.
- `MutexGuard<T>` implements `Deref`/`DerefMut`, giving access to
  the inner value. The lock is released when the guard is dropped
  (RAII pattern). Keep the guard scope as small as possible.
- **Poisoning**: if a thread panics while holding the lock, the
  mutex becomes "poisoned". Subsequent `lock()` calls return
  `Err(PoisonError)`. You can recover with `.unwrap()` (panic if
  poisoned) or `.into_inner()` (access the data despite poisoning).
- Common pattern: `Arc<Mutex<T>>` — `Arc` provides shared ownership
  across threads, `Mutex` provides exclusive access.
- **Deadlock risk**: if thread A holds lock X and waits for lock Y,
  while thread B holds lock Y and waits for lock X, both threads
  block forever. Avoid by always acquiring locks in a consistent
  order.

### `RwLock<T>`

- Allows **multiple concurrent readers** OR **one exclusive writer**
  at a time.
- `rwlock.read()` — acquires a shared read lock. Multiple threads
  can hold read locks simultaneously.
- `rwlock.write()` — acquires an exclusive write lock. Blocks until
  all readers and other writers release their locks.
- Prefer `RwLock` over `Mutex` when reads are much more frequent
  than writes. If writes are common, `Mutex` is simpler and avoids
  the overhead of tracking reader counts.
- Also susceptible to poisoning, like `Mutex`.
- **Writer starvation**: `std::sync::RwLock` makes no fairness
  guarantees — the docs state that the priority policy is unspecified,
  so a waiting writer may or may not block new readers. Since Rust 1.62
  (Linux) and 1.78 (other Unixes) std implements the lock itself
  (futex-based on Linux, a queue-based implementation elsewhere,
  SRWLOCK on Windows) rather than delegating to a platform rwlock.
  Either way, do not rely on any particular reader/writer priority. This generally prevents a scenario
  where a steady stream of readers never allows a writer to acquire
  the lock, but no fairness guarantees are made.

### MutexGuard Lifetime Pitfalls

- **One-liner pattern**: `list.lock().unwrap().push(1);` — the guard
  is a temporary that is dropped at the end of the statement. The
  lock is held only for the duration of that single expression.
- **`if let` pitfall (every edition)**: temporaries in the scrutinee
  of `if let` live until the end of the `if let` **body**, so the
  MutexGuard is still held while the body runs:
  ```text
  if let Some(item) = list.lock().unwrap().pop() {
      process(item);  // guard is STILL held here — in EVERY edition
  }
  ```
  **What Rust 2024 changes** ("if let rescoping", RFC 3606) is narrower
  than it is often described: scrutinee temporaries are now dropped
  *before the `else` block* instead of after the whole `if let`. The
  body is unaffected. So this deadlocked in edition 2021 and is fine
  in edition 2024:
  ```text
  if let Some(item) = list.lock().unwrap().pop() {
      process(item);
  } else {
      list.lock().unwrap().push(0);  // 2021: DEADLOCK; 2024: fine
  }
  ```
  **Portable workaround for the body** (works in all editions): extract
  to a separate `let` so the guard drops first:
  ```text
  let item = list.lock().unwrap().pop();
  if let Some(item) = item {
      process(item);  // lock is NOT held here (any edition)
  }
  ```
- A plain `if` does NOT have this problem — the condition is its own
  temporary scope, so any temporaries it creates (e.g. the `MutexGuard`
  in `if list.lock().unwrap().is_empty()`) are dropped before the body
  executes.

### Lock Hold Duration

- Keep the duration a mutex is locked as short as possible. Holding
  a lock during slow operations (I/O, sleep, computation) forces
  other threads to wait, effectively serializing their work and
  eliminating the benefits of concurrency.
- Use `drop(guard)` explicitly when you need to release the lock
  before the end of the scope.
*/

fn safe_state_sharing() {
    // --- Atomic counter with Relaxed ordering ---
    // 5 threads each increment the counter 100 times.
    // Relaxed is sufficient because we only need the final count to
    // be correct — we don't care about ordering of surrounding ops.

    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::new();
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let total = counter.load(Ordering::Relaxed);
    println!("atomic counter (5 threads x 100): {total}");
    assert_eq!(total, 500);

    // --- Acquire/Release flag signaling ---
    // The producer writes a payload, then sets a flag with Release.
    // The consumer loads the flag with Acquire, then reads the payload.
    // The Acquire/Release pair guarantees the consumer sees the payload.
    let payload = Arc::new(AtomicU64::new(0));
    let ready = Arc::new(AtomicBool::new(false));

    let payload_w = Arc::clone(&payload);
    let ready_w = Arc::clone(&ready);
    let producer = thread::spawn(move || {
        // Write the payload first
        payload_w.store(42, Ordering::Relaxed);
        // Then signal readiness — Release ensures the payload write
        // is visible to anyone who does an Acquire load of `ready`
        ready_w.store(true, Ordering::Release);
    });

    let payload_r = Arc::clone(&payload);
    let ready_r = Arc::clone(&ready);
    let consumer = thread::spawn(move || {
        // Spin until the flag is set — Acquire ensures we see the
        // payload write that happened before the Release store
        while !ready_r.load(Ordering::Acquire) {
            thread::yield_now(); // hint to the OS to schedule others
        }
        // Safe to read the payload — guaranteed to see 42
        payload_r.load(Ordering::Relaxed)
    });

    producer.join().unwrap();
    let received = consumer.join().unwrap();
    println!("Acquire/Release signaling — consumer received: {received}");

    // --- Mutex poisoning recovery ---
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data_clone = Arc::clone(&data);
    let _ = thread::spawn(move || {
        let _guard = data_clone.lock().unwrap();
        panic!("thread panics while holding the lock");
    })
    .join(); // join returns Err because the thread panicked

    // The mutex is now poisoned — lock() returns Err(PoisonError)
    match data.lock() {
        Ok(guard) => println!("mutex ok: {guard:?}"),
        Err(poisoned) => {
            // Recover the data despite poisoning using into_inner()
            let recovered = poisoned.into_inner();
            println!("mutex was poisoned, recovered data: {recovered:?}");
        }
    }

    // --- Mutex: shared mutable Vec ---
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for i in 0..5 {
        let log = Arc::clone(&log);
        handles.push(thread::spawn(move || {
            // lock() acquires the mutex; the guard auto-releases on drop
            {
                let mut guard = log.lock().unwrap();
                guard.push(format!("entry from thread {i}"));
            }

            // guard is dropped here → lock is released
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let entries = log.lock().unwrap();
    println!("mutex-protected log ({} entries):", entries.len());
    for entry in entries.iter() {
        println!("  {entry}");
    }

    // --- MutexGuard lifetime: one-liner vs if-let pitfall ---
    let list = Mutex::new(vec![1, 2, 3]);

    // One-liner: guard is a temporary, dropped at end of statement.
    // Lock is held only during this line.
    list.lock().unwrap().push(4);

    // if-let PITFALL (every edition): the guard lives until the end of
    // the if-let BODY, so the lock is held during the body even though
    // we only needed it for the pop. Rust 2024's "if let rescoping" only
    // moves the drop to *before the else block* — the body is unchanged:
    //
    //   // Every edition: lock STILL held during process_item!
    //   if let Some(item) = list.lock().unwrap().pop() {
    //       process_item(item);
    //   } else {
    //       // Edition 2021: DEADLOCK here. Edition 2024: fine.
    //       list.lock().unwrap().push(0);
    //   }
    //
    // PORTABLE FIX: extract to a separate let — guard drops at end of
    // that statement, before we enter the if-let body (works in all editions).
    let item = list.lock().unwrap().pop();
    if let Some(item) = item {
        println!("MutexGuard pitfall fix: popped {item} (lock released before this line)");
    }

    // --- Lock hold duration: keep it minimal ---
    // Holding the guard during sleep serializes all threads.
    // Dropping the guard before sleep allows parallelism.
    let counter = Mutex::new(0u32);
    let start = Instant::now();
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                let mut guard = counter.lock().unwrap();
                *guard += 1;
                // Release the lock BEFORE sleeping — other threads can proceed
                drop(guard);
                thread::sleep(Duration::from_millis(25));
            });
        }
    });
    let parallel_time = start.elapsed();
    println!(
        "lock hold duration: 4 threads × 25ms sleep (lock dropped before sleep) = {:?}, counter = {}",
        parallel_time,
        counter.into_inner().unwrap()
    );
    // If we had NOT dropped the guard before sleep, total time would
    // be ~100ms (4 × 25ms serialized) instead of ~25ms.

    // --- RwLock: multiple readers, one writer ---
    let config = Arc::new(RwLock::new(HashMap::from([
        ("host".to_string(), "localhost".to_string()),
        ("port".to_string(), "8080".to_string()),
    ])));

    // Spawn 3 reader threads — they can read concurrently
    let mut handles = Vec::new();
    for i in 0..3 {
        let config = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            let guard = config.read().unwrap();
            let host = guard.get("host").unwrap().clone();
            format!("reader {i} saw host={host}")
        }));
    }

    // Spawn 1 writer thread
    let config_w = Arc::clone(&config);
    handles.push(thread::spawn(move || {
        let mut guard = config_w.write().unwrap();
        guard.insert("host".to_string(), "0.0.0.0".to_string());
        "writer updated host".to_string()
    }));

    for h in handles {
        println!("  {}", h.join().unwrap());
    }
}


// ============================================================
// Section 4: Communication between Threads Using Channels
// ============================================================

/*
## Communication between Threads Using Channels

Channels provide a message-passing approach to concurrency: instead
of sharing memory (and synchronizing access), threads communicate
by sending values through a channel.

### `std::sync::mpsc` — Multi-Producer, Single-Consumer

- `mpsc::channel::<T>()` — creates an **unbounded** channel.
  Returns `(Sender<T>, Receiver<T>)`. The sender can send
  indefinitely without blocking (messages are buffered in memory).
- `mpsc::sync_channel::<T>(bound)` — creates a **bounded** channel
  with a capacity of `bound` messages. `send()` blocks when the
  buffer is full, applying backpressure to the producer. Returns
  `(SyncSender<T>, Receiver<T>)`.
- `sender.send(value)` — sends a value through the channel. This
  **moves** the value — ownership is transferred. Returns
  `Result<(), SendError<T>>`; fails if the receiver has been
  dropped.
- `sender.clone()` — creates another sender for the same channel.
  This is how you get **multiple producers**.
- `receiver.recv()` — blocks until a message arrives. Returns
  `Result<T, RecvError>`; returns `Err` when all senders have been
  dropped (channel is closed).
- `receiver.try_recv()` — non-blocking. Returns
  `Err(TryRecvError::Empty)` if no message is available, or
  `Err(TryRecvError::Disconnected)` if all senders are dropped.
- `receiver.recv_timeout(duration)` — blocks for at most `duration`.
- `Receiver<T>` implements `IntoIterator` (by value and by reference),
  so `for msg in receiver { ... }` and `receiver.iter()` both work. It
  is *not* itself an `Iterator`, so there is no `receiver.next()` or
  `receiver.map()` — call `.iter()` / `.try_iter()` to get one. The loop runs until all senders are
  dropped. This is the idiomatic way to drain a channel.
- When all `Sender`s are dropped, the channel closes. Pending
  messages can still be received, and `recv()` returns `Err` once
  the buffer is empty.
*/

fn channels() {
    // --- Basic channel: one producer, one consumer ---
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();
        }
        // tx is dropped here -> the channel closes once messages are consumed
    });
    // Iterating the receiver blocks until the channel closes.
    let received: Vec<i32> = rx.iter().collect();
    println!("channel received: {received:?}");

    // --- Multiple producers ---
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone(); // second sender

    thread::spawn(move || {
        tx.send("hello from producer 1").unwrap();
    });
    thread::spawn(move || {
        tx2.send("hello from producer 2").unwrap();
    });
    // Both senders are moved into threads and dropped when those threads end.
    // Arrival ORDER is non-deterministic, so sort before printing — asserting
    // a fixed order here would be a lie about how threads interleave.
    let mut messages: Vec<&str> = rx.iter().collect();
    messages.sort();
    println!("multiple producers: {messages:?}");

    // --- Bounded channel (sync_channel) with backpressure ---
    // Capacity of 2: the 3rd send blocks until the receiver reads.
    let (tx, rx) = mpsc::sync_channel(2);
    let producer = thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap(); // blocks when the buffer is full
        }
    });
    // Consuming lets the producer unblock.
    let received: Vec<i32> = rx.iter().collect();
    producer.join().unwrap();
    println!("bounded channel received: {received:?}");

    // --- Ownership transfer through channels ---
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let data = vec![String::from("a"), String::from("b"), String::from("c")];
        tx.send(data).unwrap();
        // `data` has been moved into the channel — it cannot be used here
    });
    let mut received = rx.recv().unwrap();
    received.push(String::from("d")); // the receiver now owns the Vec
    println!("ownership transferred: {received:?}");
}

// ============================================================
// Section 5: Scoped Threads, Barrier, Condvar
// ============================================================

/*
## Scoped Threads, Barrier, Condvar

### Scoped Threads — `thread::scope`

- `thread::scope(|s| { ... })` creates a scope in which threads can
  **borrow local data** without the `'static` bound.
- Inside the scope closure, `s.spawn(|| ...)` spawns a scoped
  thread. The closure can borrow from the enclosing function because
  the scope guarantees all spawned threads finish before `scope`
  returns.
- If any scoped thread panics, `scope` joins all remaining threads
  and then propagates the panic.
- **Key benefit**: eliminates the need for `Arc` when threads only
  need temporary access to data. Compare with Section 1 where `Arc`
  was necessary because `thread::spawn` requires `'static`.

### Barrier

- `Barrier::new(n)` creates a barrier that blocks each thread
  calling `.wait()` until `n` threads have all called `.wait()`.
- Once all `n` threads arrive, they are all released simultaneously.
- `BarrierWaitResult::is_leader()` — exactly one thread is
  designated the "leader" and returns `true`. Useful when one
  thread should perform cleanup or logging after the barrier.
- Use case: phased computation where all threads must finish phase N
  before any starts phase N+1.

### Condvar — Condition Variable

- A `Condvar` is always paired with a `Mutex`. It allows threads to
  **wait** for a condition to become true, without busy-spinning.
- `condvar.wait(guard)` atomically releases the mutex and suspends
  the thread. When the thread is woken, it reacquires the mutex and
  returns the new guard.
- `condvar.notify_one()` wakes one waiting thread.
  `condvar.notify_all()` wakes all waiting threads.
- **Always wait in a loop**: `while !condition { guard =
  cvar.wait(guard).unwrap(); }`. Spurious wakeups (where `wait`
  returns without `notify` being called) can happen on some
  platforms, so you must re-check the condition after each wakeup.
- Use case: producer-consumer patterns where the consumer sleeps
  until new data is available, avoiding busy-wait loops.
*/

fn scoped_threads_and_synchronization() {
    // --- Scoped threads borrowing local data ---
    // No Arc needed — scoped threads can borrow directly
    let data = [1, 2, 3, 4, 5];
    let results = Mutex::new(Vec::new());

    thread::scope(|s| {
        for chunk in data.chunks(2) {
            s.spawn(|| {
                // Borrow `chunk` from the enclosing scope (no move, no Arc)
                let sum: i32 = chunk.iter().sum();
                results.lock().unwrap().push(sum);
            });
        }
        // All scoped threads are automatically joined here
    });

    let mut results = results.into_inner().unwrap();
    results.sort();
    println!("scoped threads (chunk sums): {results:?}");

    // --- Barrier: synchronizing phases ---
    let barrier = Barrier::new(4);
    let phase_log = Mutex::new(Vec::new());

    thread::scope(|s| {
        for i in 0..4 {
            let barrier = &barrier;
            let phase_log = &phase_log;
            s.spawn(move || {
                // Phase 1 work
                phase_log
                    .lock()
                    .unwrap()
                    .push(format!("thread {i}: phase 1 done"));

                // Wait for all threads to finish phase 1
                let wait_result = barrier.wait();
                if wait_result.is_leader() {
                    phase_log
                        .lock()
                        .unwrap()
                        .push("--- barrier released (leader) ---".to_string());
                }

                // Phase 2 work — all threads guaranteed to start together
                phase_log
                    .lock()
                    .unwrap()
                    .push(format!("thread {i}: phase 2 started"));
            });
        }
    });

    println!("barrier phases:");
    for entry in phase_log.into_inner().unwrap() {
        println!("  {entry}");
    }

    // --- Condvar: producer-consumer pattern ---
    // The producer pushes items into a shared queue and notifies the
    // consumer. The consumer waits on the Condvar until data appears.
    //
    // RULE: every value the wait predicate reads must live under the SAME
    // mutex the Condvar is paired with, and must be changed while holding
    // that mutex. Signalling completion through a separate atomic would
    // create a lost wakeup: the consumer can evaluate the predicate as
    // "keep waiting" and then be preempted just before wait(); if the
    // producer flips the flag and notifies in that window, the notification
    // is lost (a Condvar stores no permit) and the consumer blocks forever.
    //
    // So the queue AND the `done` flag share one Mutex.
    let state = Arc::new((Mutex::new((Vec::<i32>::new(), false)), Condvar::new()));

    let producer_state = Arc::clone(&state);
    let producer = thread::spawn(move || {
        let (lock, cvar) = &*producer_state;
        for i in 1..=5 {
            lock.lock().unwrap().0.push(i);
            cvar.notify_one(); // wake the consumer
            thread::sleep(Duration::from_millis(5));
        }
        lock.lock().unwrap().1 = true; // set `done` WHILE holding the lock
        cvar.notify_one(); // wake the consumer so it observes `done`
    });

    let consumer_state = Arc::clone(&state);
    let consumer = thread::spawn(move || {
        let (lock, cvar) = &*consumer_state;
        let mut consumed = Vec::new();
        let mut guard = lock.lock().unwrap();
        loop {
            // Wait in a loop — handles spurious wakeups
            while guard.0.is_empty() && !guard.1 {
                guard = cvar.wait(guard).unwrap();
            }
            // Drain whatever is available
            consumed.append(&mut guard.0);
            if guard.1 {
                break; // producer finished and we have drained the queue
            }
        }
        consumed
    });

    producer.join().unwrap();
    let consumed = consumer.join().unwrap();
    println!("condvar producer-consumer: consumed {consumed:?}");

    // --- Thread Parking: wake a specific thread ---
    // The worker parks itself until the main thread unparks it.
    let flag = Arc::new(AtomicBool::new(false));
    let flag_clone = Arc::clone(&flag);

    let worker = thread::spawn(move || {
        // Park in a loop — park() may return spuriously.
        while !flag_clone.load(Ordering::Acquire) {
            thread::park();
        }
        "worker woke up and saw the flag"
    });

    // Give the worker time to park.
    thread::sleep(Duration::from_millis(10));
    // Set the flag BEFORE unparking, so the worker cannot miss it.
    flag.store(true, Ordering::Release);
    worker.thread().unpark();

    let result = worker.join().unwrap();
    println!("thread parking: {result}");
}

pub fn run() {
    launching_and_coordinating_threads();
    send_and_sync();
    safe_state_sharing();
    channels();
    scoped_threads_and_synchronization();
}
