use std::sync::{Arc, Mutex};
use std::thread;

/// Arc = Atomic Reference Counted
///
/// It lets multiple threads own the same data safely.
///
/// - `Rc<T>` is for single-threaded shared ownership
/// - `Arc<T>` is for multi-threaded shared ownership
///
/// Arc only gives shared ownership.
/// If you want to MUTATE shared data across threads,
/// you usually combine it with `Mutex<T>`.

fn main() {
    basic_arc_example();
    thread_arc_example();
    arc_mutex_example();
}

fn basic_arc_example() {
    println!("--- Basic Arc Example ---");

    let message = Arc::new(String::from("hello from Arc"));

    let a = Arc::clone(&message);
    let b = Arc::clone(&message);

    println!("original: {}", message);
    println!("clone a: {}", a);
    println!("clone b: {}", b);

    // All 3 variables point to the same heap data.
    println!("strong count = {}", Arc::strong_count(&message));
    println!();
}

fn thread_arc_example() {
    println!("--- Arc Across Threads Example ---");

    let numbers = Arc::new(vec![10, 20, 30, 40]);

    let mut handles = vec![];

    for i in 0..3 {
        let shared_numbers = Arc::clone(&numbers);

        let handle = thread::spawn(move || {
            let sum: i32 = shared_numbers.iter().sum();
            println!("thread {} sees {:?}, sum = {}", i, shared_numbers, sum);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("after threads, strong count = {}", Arc::strong_count(&numbers));
    println!();
}

fn arc_mutex_example() {
    println!("--- Arc<Mutex<T>> Example ---");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let shared_counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = shared_counter.lock().unwrap();
                *num += 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_value = *counter.lock().unwrap();
    println!("final counter value = {}", final_value);
}
