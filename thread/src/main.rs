use std::thread;
use std::thread::JoinHandle;

fn main() {
    let handle:JoinHandle<()> = thread::spawn(|| {
        println!("Hello, from thread.");
    });

    handle.join().unwrap();
}
