use ::axum::Router;
use ::axum::routing;
use ::clap::Parser;
use ::tracing::info;
use ::tracing::Level;
use ::tracing::span;
use ::tracing_subscriber;

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
        .route("/api", routing::get(|| async { "{\"error\": \"not yet implemented\"}" }))
        .route("/", routing::get(|| async { "Hello, World!" }));

    let span = span!(Level::INFO, "running_server");
    let _guard = span.enter();
    info!("host = {}", &args.host);
    axum::Server::bind(&args.host.parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
