use std::thread;
use std::time::Duration;

fn main() {
    let cores = 4;
    let mut threads = vec![];

    for _ in 0..cores {
        let t = thread::spawn(|| {
            let id = thread::current().id();
            println!("hi from thread {:?}", id);
            thread::sleep(Duration::from_millis(1500));
            println!("by from thread {:?}", id);
            id
        });

        threads.push(t);
    }
    println!("after starting the threads");

    for t in threads {
        let res = t.join();
        match res {
            Ok(id) => println!("thread {:?} finished", id),
            Err(e) => println!("thread returned an error {:?}", e),
        }
    }
}
