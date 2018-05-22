use std::sync::{ Mutex, Arc };
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut c = counter.lock().unwrap();
            *c += 1;
        });
        handles.push(handle);
    }
    
    for h in handles {
        h.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
}
