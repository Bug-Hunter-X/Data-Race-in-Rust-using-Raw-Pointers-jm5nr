use std::sync::{Arc, Mutex};

fn main() {
    let v = Arc::new(Mutex::new(vec![1, 2, 3]));
    let v_clone = v.clone();

    let thread1 = std::thread::spawn(move || {
        let mut v_locked = v_clone.lock().unwrap();
        *v_locked.get_mut(0).unwrap() = 4; 
    });

    thread1.join().unwrap();
    println!("The first element is: {}", v.lock().unwrap()[0]);
} 