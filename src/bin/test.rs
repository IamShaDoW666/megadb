use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            for j in 0..11 {
                let mut data = data.lock().unwrap();
                *data += 1;
                println!("Thread: {} {} {}", i, j, *data);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let data = *data.lock().unwrap();
    println!("Final count: {}", data);
}
