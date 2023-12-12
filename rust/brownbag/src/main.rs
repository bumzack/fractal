use crate::multithreaded::multi_threaded;
use crate::multithreaded_mpsc::multi_threaded_mpsc;
use crate::singlethreaded::single_threaded;

mod complex;
mod fractal;
mod multithreaded;
mod multithreaded_mpsc;
mod palette;
mod singlethreaded;
mod tile;
mod utils;

fn main() {
    let width = 4096;
    let height = 3072;
    let max_iterations = 100_000;

    // let width = 1024;
    // let height = 768;
    // let max_iterations = 100;

    single_threaded(width, height, max_iterations);
    multi_threaded(width, height, max_iterations);
    multi_threaded_mpsc(width, height, max_iterations);
}
