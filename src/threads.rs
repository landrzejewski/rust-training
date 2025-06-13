use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Barrier, Mutex, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn run() {
    channels()
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

fn atomics() {
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_ref = counter.clone();
    thread::spawn(move || {
        for _ in 0..50 {
            counter_ref.fetch_add(1, Ordering::Relaxed);
            println!("Counter value: {:?} (thread: {:?})", counter_ref, thread::current().id());
        }
    });
    thread::sleep(Duration::from_secs(1));
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

fn mutex() {
    let atomic_counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..50 {
        let counter = Arc::clone(&atomic_counter);
        let handle = thread::spawn(move || {
            if let Ok(mut value) = counter.lock() {
                *value += 1;
                println!("Counter value: {} (thread: {:?})", *value, thread::current().id());
                //thread::sleep(Duration::from_secs(1));
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", atomic_counter.lock().unwrap());
}

fn threads() {
    let numbers = Arc::new(vec![1, 2, 3]);
    let numbers_clone = numbers.clone();
    /*thread::spawn(|| {
        loop {
            println!("Hello!");
        }
    });
    thread::sleep(Duration::from_secs(10));*/
    let spawn_result = thread::Builder::new()
        .name("My thread 1".to_owned())
        /*.spawn(move || {
            println!("Numbers: {:?}", numbers);
            for index in 1..10 {
                println!("Index {}, (thread: {:?})", index, thread::current().name().unwrap());
                thread::sleep(Duration::from_secs(1));
            }
            String::from("Success")
        });*/
        //.spawn(move || task(numbers));
        .spawn(|| task(numbers_clone));
    // thread::sleep(Duration::from_secs(10));
    let handle = spawn_result.unwrap();
    let result = handle.join().unwrap();
    println!("Result: {} (thread: {:?})", result, thread::current().name().unwrap());
    println!("Number: {:?} (thread: {:?})", numbers, thread::current().name().unwrap());
}

/*fn task(numbers: &Vec<i32>) -> String {
    println!("Numbers: {:?}", numbers);
    for index in 1..10 {
        println!("Index {}, (thread: {:?})", index, thread::current().name().unwrap());
        thread::sleep(Duration::from_secs(1));
    }
    String::from("Success")
}*/

fn task(numbers: Arc<Vec<i32>>) -> String {
    println!("Numbers: {:?}", numbers);
    for index in 1..10 {
        println!("Index {}, (thread: {:?})", index, thread::current().name().unwrap());
        thread::sleep(Duration::from_secs(1));
    }
    String::from("Success")
}
