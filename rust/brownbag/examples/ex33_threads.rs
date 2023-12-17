use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let cores = 4;
    let mut threads = vec![];

    let y_global = 0;
    let height = 20;

    let y_global = Arc::new(Mutex::new(y_global));

    for _ in 0..cores {
        let y_global = y_global.clone();
        let t = thread::spawn(move || {
            let id = thread::current().id();

            let mut y = 0;
            let mut processed_rows = 0;
            while *y_global.lock().unwrap() < height {
                let mut y_global = y_global.lock().unwrap();
                y = *y_global;
                *y_global += 1;
                processed_rows += 1;
                println!("threadId {:?}, y_global {}, y {}", id, y_global, y);

                // real work
                thread::sleep(Duration::from_millis(25));
            }

            (id, processed_rows)
        });

        threads.push(t);
    }
    println!("after starting the threads");

    for t in threads {
        let res = t.join();
        match res {
            Ok((id, processed_rows)) => {
                println!("thread {:?} processed {} rows", id, processed_rows)
            }
            Err(e) => println!("thread returned an error {:?}", e),
        }
    }
}
