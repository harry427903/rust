use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hello from thread 1: {}", i);
            thread::sleep(Duration::from_millis(1));
        } 
    });

    for i in 1..5 {
        println!("Hello from thread 2: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}