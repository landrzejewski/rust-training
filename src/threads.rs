use std::sync::{mpsc, Arc, Barrier, Mutex, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn run() {
    threads();
}

fn channels() {
    let (tx, rx) = mpsc::channel();
    let txl = tx.clone();
    thread::spawn(move || {
        let numbers = vec!["1", "2", "3"];
        for number in numbers {
            _ = &txl.send(number);
            thread::sleep(Duration::from_secs(1));
        }
    });

    _ = tx.send("abc");

    for number in rx {
        // infinite loop due to use/existence of tx references
        println!("New message: {number}");
    }

    // rx.iter()
    //    .map(fn)
}

fn rw_lock() {
    let lock = RwLock::new(0);
    if let Ok(mut write_lock_guard) = lock.write() {
        *write_lock_guard += 1;
    };
    if let Ok(read_lock_guard) = lock.read() {
        let value = *read_lock_guard;
        println!("Value {}", value);
    };
}

fn barrier() {
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(10);
    let atomic_barrier = Arc::new(Barrier::new(10));
    for _ in 1..=10 {
        let barrier = Arc::clone(&atomic_barrier);
        handles.push(thread::spawn(move || {
            println!("Before wait ({:?})", thread::current());
            barrier.wait();
            println!("After wait ({:?})", thread::current());
        }));
    }
    for handle in handles {
        _ = handle.join();
    }
}

fn mutex() {
    let atomic_counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for _ in 1..50 {
        let counter = Arc::clone(&atomic_counter);
        let handle = thread::spawn(move || {
            if let Ok(mut guard) = counter.lock() {
                *guard += 1;
                println!("Counter value: {}, ({:?})", *guard, thread::current());
                thread::sleep(Duration::from_secs(1));
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        _ = handle.join();
    }
    println!(
        "Final counter value {}, ({:?})",
        *atomic_counter.lock().unwrap(),
        thread::current()
    );
}

fn threads() {
    let numbers = vec![1, 2, 3];
    let result = thread::Builder::new()
        .name("other".to_string())
        /*.spawn(move || {
            println!("Numbers: {:?}", numbers);
            for index in 1..10 {
                println!("Index: {}, ({:?})", index, thread::current());
                thread::sleep(Duration::from_secs(1));
            }
        });*/
        .spawn(move || task(numbers));
        //.spawn(move || task(&numbers));
    println!("Before join ({:?})", thread::current());
    let values = match result {
        Ok(handle) => handle.join(),
        Err(error) => {
            println!("Error: {}", error);
            Ok(vec![0])
        }
    }
    .expect("Failed");
    println!("After join {:?} ({:?})", values, thread::current());
}

fn task(numbers: Vec<i32>) -> Vec<i32> {
    //fn task(numbers: Vec<i32>) -> Vec<i32> {
    println!("Numbers: {:?}", numbers);
    for index in 1..10 {
        println!("Index: {}, ({:?})", index, thread::current());
        thread::sleep(Duration::from_secs(1));
    }
    numbers.clone()
}
