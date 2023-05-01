use ::clap::Parser;
use ::tracing_subscriber;
use ::axum::Router;
use ::axum::routing;

use crate::args::Args;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod args;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let app = Router::new()
        .route("/", routing::get(|| async { "Hello, World!" }));

    axum::Server::bind(&args.host.parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
