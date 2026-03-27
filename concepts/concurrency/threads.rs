use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i: in 1..10 {
            thread::sleep(time::Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
}