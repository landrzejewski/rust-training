use std::cell::Cell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Barrier, Condvar, Mutex, RwLock, mpsc};
use std::thread;
use std::time::{Duration, Instant};

// =================================================================================================
// Section 1: Launching and Coordinating Threads
// =================================================================================================

/*
## Launching and Coordinating Threads

- Rust uses **OS threads** (1:1 model) — each `thread::spawn` call creates a real operating system
  thread with its own stack.
- `thread::spawn` takes a closure with three bounds:
  - `FnOnce` — the closure is called exactly once (when the thread starts). It may consume captured
    variables.
  - `Send` — everything the closure captures must be safe to transfer to another thread. This is
    checked at compile time.
  - `'static` — the closure and its captured data must live for the entire duration of the thread
    (which could outlive the spawning scope). This is why you cannot borrow local variables directly
    — use `move` closures to transfer ownership.
- `thread::spawn` returns a `JoinHandle<T>` where `T` is the closure's return type.
  - `handle.join()` blocks the calling thread until the spawned thread finishes. It returns
    `Result<T, Box<dyn Any + Send>>`.
  - `Ok(value)` — the thread completed normally, returning `value`.
  - `Err(payload)` — the thread panicked. The panic payload is returned, and the calling thread can
    handle it gracefully instead of propagating the panic.
  - **Where** you call `join()` matters: calling it *before* the main thread's own work forces the
    spawned thread to finish first (sequential, no interleaving), while calling it *after* lets both
    run concurrently and then synchronizes at the end. Small details like the position of `join` in
    the source code determine whether threads actually run in parallel.
- Dropping a `JoinHandle` without calling `join()` **detaches** the thread — it keeps running in the
  background but can no longer be joined. When the main thread exits, all detached threads are
  terminated immediately.
- `thread::Builder` provides fine-grained control:
  - `.name("...")` — sets a thread name visible in debuggers and `thread::current().name()`.
  - `.stack_size(bytes)` — overrides the default stack size.
  - `.spawn(closure)` returns `io::Result<JoinHandle<T>>` (may fail if the OS refuses to create the
    thread).
- `thread::current()` returns a handle to the calling thread. `.id()` returns a unique `ThreadId`,
  `.name()` returns `Option<&str>`.
- `thread::sleep(duration)` puts the current thread to sleep for at least the specified duration. It
  is a real OS-level sleep and blocks the thread entirely.

### Arc — Atomic Reference Counting

- When multiple threads need shared access to the same data, you need `Arc<T>` (Atomic Reference
  Counted). It is the thread-safe counterpart of `Rc<T>`.
- `Rc<T>` is `!Send` — its reference count uses non-atomic operations, so it cannot be safely shared
  across threads. The compiler will reject any attempt to send an `Rc` to another thread.
- `Arc::new(value)` creates a new reference-counted pointer. `Arc::clone(&arc)` increments the
  reference count atomically. When the last `Arc` is dropped, the value is deallocated.
- `Arc<T>` is `Send + Sync` when `T: Send + Sync`. This means you can both send it to other threads
  and share references to it.
- `Arc` provides **shared read-only access**. For shared mutable access, combine it with interior
  mutability: `Arc<Mutex<T>>` or `Arc<RwLock<T>>` (see Section 3).
- Prefer `Arc::clone(&arc)` over `arc.clone()` — the explicit form makes it clear you are cloning
  the pointer (cheap), not the underlying data (potentially expensive).

### Output Locking

- `println!` uses `std::io::Stdout::lock()` internally to ensure its output is not interrupted by
  another thread's `println!`. Each `println!` waits for any concurrent `println!` to finish before
  writing. Without this, output from multiple threads could be interleaved mid-line in garbled
  fashion.

### Arc Clone Shadowing Idiom

- When spawning multiple threads that each need their own `Arc` clone, naming each clone
  (`arc_clone1`, `arc_clone2`, ...) is cluttered. Instead, open a new scope before each `move`
  closure and shadow the variable:
  ```text
  thread::spawn({
      let data = data.clone();  // shadow `data` in this scope
      move || { /* use data */ }
  });
  ```
  The outer `data` remains available for the next thread's clone. This is the idiomatic pattern in
  production Rust code.
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
    let data = vec![1, 2, 3, 4, 5];
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
    let data_for_t1 = Arc::clone(&shared_data);
    let data_for_t2 = Arc::clone(&shared_data);

    let h1 = thread::spawn(move || data_for_t1.iter().sum::<i32>());
    let h2 = thread::spawn(move || data_for_t2.iter().max().copied());
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

/*
### `Mutex<T>`

- `Mutex::new(value)` creates a mutex wrapping a value.
- `mutex.lock()` acquires the lock, blocking if another thread holds it. Returns
  `LockResult<MutexGuard<T>>`.
- `MutexGuard<T>` implements `Deref`/`DerefMut`, giving access to the inner value. The lock is
  released when the guard is dropped (RAII pattern). Keep the guard scope as small as possible.
- **Poisoning**: if a thread panics while holding the lock, the mutex becomes "poisoned". Subsequent
  `lock()` calls return `Err(PoisonError<MutexGuard<T>>)`. Two recovery options:
  - Call `.unwrap()` on the `LockResult` — panics if the mutex is poisoned, otherwise yields the
    guard.
  - Match on `Err(poisoned)` and call `poisoned.into_inner()` on the `PoisonError` to extract the
    guard despite the poison (this is the pattern the demo below uses).
- Common pattern: `Arc<Mutex<T>>` — `Arc` provides shared ownership across threads, `Mutex` provides
  exclusive access.
- **Deadlock risk**: if thread A holds lock X and waits for lock Y, while thread B holds lock Y and
  waits for lock X, both threads block forever. Avoid by always acquiring locks in a consistent
  order.

### `RwLock<T>`

- Allows **multiple concurrent readers** OR **one exclusive writer** at a time.
- `rwlock.read()` — acquires a shared read lock. Multiple threads can hold read locks
  simultaneously.
- `rwlock.write()` — acquires an exclusive write lock. Blocks until all readers and other writers
  release their locks.
- Prefer `RwLock` over `Mutex` when reads are much more frequent than writes. If writes are common,
  `Mutex` is simpler and avoids the overhead of tracking reader counts.
- Also susceptible to poisoning, like `Mutex`.
- **Writer starvation**: many OS-level RwLock implementations block new readers when a writer is
  waiting, even if the lock is currently read-locked. Rust's `RwLock` delegates to the OS, so the
  exact behavior is platform-dependent. This generally prevents a scenario where a steady stream of
  readers never allows a writer to acquire the lock, but no fairness guarantees are made.

### MutexGuard Lifetime Pitfalls

- **One-liner pattern**: `list.lock().unwrap().push(1);` — the guard is a temporary that is dropped
  at the end of the statement. The lock is held only for the duration of that single expression.
- **`if let` pitfall (Rust 2021 and earlier)**: in editions before 2024, temporaries in the
  scrutinee of `if let` lived until the end of the entire `if let` block:
  ```text
  if let Some(item) = list.lock().unwrap().pop() {
      process(item);  // guard was STILL held here in edition 2021!
  }
  ```
  **Rust 2024 edition fixes this**: temporaries not captured by the pattern are dropped before the
  body executes, so the MutexGuard is released before `process(item)` runs. **Portable workaround**
  (works in all editions): extract to a separate `let` so the guard drops first:
  ```text
  let item = list.lock().unwrap().pop();
  if let Some(item) = item {
      process(item);  // lock is NOT held here (any edition)
  }
  ```
- A plain `if` does NOT have this problem — a boolean condition cannot borrow from temporaries, so
  the guard drops before the body executes.

### Lock Hold Duration

- Keep the duration a mutex is locked as short as possible. Holding a lock during slow operations
  (I/O, sleep, computation) forces other threads to wait, effectively serializing their work and
  eliminating the benefits of concurrency.
- Use `drop(guard)` explicitly when you need to release the lock before the end of the scope.
*/

fn safe_state_sharing() {
    // --- Mutex: shared mutable Vec ---
    let log = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for i in 0..5 {
        let log = Arc::clone(&log);
        handles.push(thread::spawn(move || {
            // lock() acquires the mutex; the guard auto-releases on drop
            let mut guard = log.lock().unwrap();
            guard.push(format!("entry from thread {i}"));
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

pub fn run() {
    launching_and_coordinating_threads();
    safe_state_sharing();
}
