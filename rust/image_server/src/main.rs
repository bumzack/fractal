use log::{info, LevelFilter};
use pretty_env_logger::env_logger::{Builder, Target};
use warp::Filter;

use crate::server::routes;

mod server;
mod utils;

// #[tokio::main(worker_threads = 2)]
#[tokio::main]
async fn main() {
    let mut builder = Builder::new();
    builder.target(Target::Stdout);
    builder.filter_level(LevelFilter::Info);
    builder.init();
    info!("builder={:?}", builder);

    let routes = routes().with(utils::cors());

    warp::serve(routes).run(([127, 0, 0, 1], 3100)).await;
}
