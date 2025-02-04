use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned Thread {}", i);
        }
    });

    handle.join().unwrap();
    for i in 1..5 {
        println!("Main Thread {}", i);
    }
}
