use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// 1. BASIC THREAD
//    thread::spawn() starts a new thread.
//    Think of a thread like a separate worker doing a job
//    at the same time as your main program.

fn basic_thread() {

    thread::spawn(|| {
        println!("Hello!");
    });

    thread::sleep(Duration::from_millis(50));
 
    println!("Hello from the main thread!");
}

// 2. JOINING THREADS
//    thread::spawn() returns a JoinHandle.
//    Calling .join() on it makes the current thread WAIT
//    until that spawned thread finishes. This is much better
//    than using sleep() to guess when it's done.

fn joining_threads() {
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  Spawned thread: step {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });
 
    // Main thread does its own work while the other runs
    println!("  Main thread: doing other stuff...");
 
    // Wait for the spawned thread to finish before moving on
    handle.join().expect("The thread panicked!");
    println!("  Main thread: spawned thread is done, continuing.");
}


// 3. MOVE CLOSURES
//    Normally closures borrow values. But threads can outlive
//    the scope where values were created, so Rust forces you
//    to MOVE values into the thread with the `move` keyword.
//    This transfers ownership into the thread.

fn move_into_thread() {
    let message = String::from("I was created outside the thread!");
 
    // Without `move` this would NOT compile – Rust won't let
    // the thread borrow `message` because it could dangle.
    let handle = thread::spawn(move || {
        // `message` is now OWNED by this thread
        println!("  Thread says: {}", message);
    });
 
    // Note: we can no longer use `message` here – it was moved.
 
    handle.join().expect("Thread panicked");
}

// 4. MUTEX  (Mutual Exclusion)
//    A Mutex wraps data and ensures only ONE thread can access
//    it at a time. You call .lock() to get access; Rust
//    automatically unlocks it when the lock goes out of scope.
//
//    Think of it like a bathroom with one key:
//      - you grab the key (.lock())
//      - do your business (read/write the data)
//      - the key is returned automatically when you're done

fn mutex_example() {
    // Wrap an integer in a Mutex
    let counter = Mutex::new(0);
 
    {
        // Lock the mutex – we now have exclusive access
        let mut num = counter.lock().expect("Mutex was poisoned");
        *num += 1; // dereference and modify the value inside
        println!("  Counter inside lock: {}", *num);
        // `num` drops here → mutex is automatically unlocked
    }
 
    // Lock again to read the final value
    println!("  Counter after lock released: {}", *counter.lock().unwrap());
}

// 5. ARC  (Atomically Reference Counted)
//    Arc<T> lets MULTIPLE owners share the same value safely
//    across threads. It keeps a count of how many owners exist
//    and frees the data when the count hits zero.
//
//    It's like a shared library book with a sign-out sheet –
//    anyone can hold a reference, and the book is put away
//    only when everyone returns it.


fn arc_example() {
    // Put a read-only message behind an Arc
    let shared_message = Arc::new(String::from("Hello from the Arc!"));
 
    let mut handles = vec![];
 
    for id in 0..3 {
        // Arc::clone() increases the reference count cheaply –
        // it does NOT copy the String, just the pointer.
        let message_clone = Arc::clone(&shared_message);
 
        let handle = thread::spawn(move || {
            println!("  Thread {}: {}", id, message_clone);
        });
        handles.push(handle);
    }
 
    for h in handles {
        h.join().unwrap();
    }
}


// 6. ARC + MUTEX  – sharing MUTABLE data across threads
//    This is the most common pattern in Rust concurrency:
//
//      Arc  = shared ownership across threads
//      Mutex = safe mutable access (one thread at a time)
//
//    Together: Arc<Mutex<T>>
//    Multiple threads can own the Arc, and each thread locks

fn arc_mutex_example() {
    // A counter shared across several threads
    let counter = Arc::new(Mutex::new(0));
 
    let mut handles = vec![];
 
    for _ in 0..5 {
        // Clone the Arc so each thread gets its own "ticket"
        // pointing to the SAME Mutex<i32>
        let counter_clone = Arc::clone(&counter);
 
        let handle = thread::spawn(move || {
            // Lock the mutex to get mutable access
            let mut num = counter_clone.lock().expect("Mutex poisoned");
            *num += 1;
            println!("  Thread incremented counter to {}", *num);
            // Lock released automatically here
        });
 
        handles.push(handle);
    }
 
    // Wait for every thread to finish
    for h in handles {
        h.join().unwrap();
    }
 
    // Read the final value
    println!(
        "\n  Final counter value: {}",
        *counter.lock().unwrap()
    );
    // Expected: 5  (one increment per thread)
}