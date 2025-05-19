use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn simple_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Main thread: {i} ");
        thread::sleep(Duration::from_millis(1));
    }

    // join is necessary so that spawned threads finish working
    // even when the main thread is done with its work
    // otherwise they will finish printing as soon as main thread
    // is done

    // experiment where you call `join` or comment it out
    // also change the numbers of loops and sleep duration
    // to get a feel for how threads work
    handle.join().unwrap();
}

pub fn enter_arc() {
    // Arc is a thread-safe reference-counting pointer
    // Here, we use Arc to share ownership of an immutable value between threads

    let counter = Arc::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Since counter is immutable, we can only read its value here
            println!("Counter value in thread: {}", counter);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", counter);
}

pub fn enter_mutex() {
    // Mutex is a mutual exclusion primitive that allows only one thread to access the data at a time
    // it is used to protect shared data from being accessed by multiple threads at the same time
    use std::thread;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

pub fn enter_mutex_vec() {
    // Demonstrates using a Mutex to safely push numbers into a Vec from multiple threads

    let numbers = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..10 {
        let numbers = Arc::clone(&numbers);
        let handle = thread::spawn(move || {
            let mut vec = numbers.lock().unwrap();
            vec.push(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let vec = numbers.lock().unwrap();
    println!("Resulting Vec: {:?}", *vec);
}
