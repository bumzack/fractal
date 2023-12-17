use std::sync::Arc;

use crossbeam::deque::{Injector, Steal, Worker};

fn worker(start_row: usize, worker: Arc<Worker<Job>>) {
    let injector = worker.clone();
    let jobs = (start_row..600)
        .step_by(10)
        .map(|y| {
            let injector = worker.clone();
            Arc::new(move || compute_row(&injector, y))
        })
        .collect::<Vec<_>>();
    jobs.into_iter().for_each(|job| loop {
        if let Steal::Success(_) = worker.steal() {
            (job)();
            break;
        }
    });
}

pub fn created_by_ai_assistant_threads_work_stealing() {
    let start = Instant::now();

    let injector = Arc::new(Injector::new());
    let workers: Vec<_> = (0..10)
        .map(|i| {
            let w = Worker::new_fifo();
            injector.push(w.clone());
            w
        })
        .collect::<Vec<_>>();
    let handles: Vec<_> = workers
        .iter()
        .map(|worker| {
            let injector = injector.clone();
            thread::spawn(move || worker(0, injector))
        })
        .collect::<Vec<_>>();
    handles.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });

    let duration = start.elapsed();

    println!("Time elapsed in fractal creation is: {:?}", duration);
}
