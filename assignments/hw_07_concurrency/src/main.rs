use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut threads = vec![];
    let counter = Arc::new(Mutex::new(0));

    for _i in 0..5 {
        // Clone the ARC reference to counter mutex, lol
        let c_mut_arc_ref = Arc::clone(&counter);
        threads.push(thread::spawn(move || {
            for _j in 0..10 {
                let mut num = c_mut_arc_ref.lock().unwrap();
                *num += 1;
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("Counter: {}", *counter.lock().unwrap());
}
