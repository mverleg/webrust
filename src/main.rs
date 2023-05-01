use ::axum::Router;
use ::axum::routing;
use ::clap::Parser;
use ::tracing::info;
use ::tracing_subscriber;
use ::tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

use crate::args::Args;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod args;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .with_level(true)
        .init();

    let args = Args::parse();

    let app = Router::new()
        .route("/api", routing::get(|| async { "{\"error\": \"not yet implemented\"}" }))
        .route("/", routing::get(|| async { "Hello, World!" }));

    info!("starting server at {}", &args.host);
    axum::Server::bind(&args.host.parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
