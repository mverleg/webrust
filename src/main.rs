#![feature(lazy_cell)]

use ::askama::Template;
use ::axum::response::Html;
use ::axum::routing;
use ::clap::Parser;
use ::tracing::info;
use ::tracing::Level;
use ::tracing::span;
use ::tracing_subscriber;

use ::tower_http::services::ServeDir;
use axum::Router;
use minify_html::Cfg;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::args::Args;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod args;
mod resources;

//TODO @mark: brotli

#[derive(Debug)]
struct SharedContext {
    base_url: String,
    logo: String,
    css: Vec<String>,
}

impl Default for SharedContext {
    //TODO @mark: or maybe use Rc instead of default?
    fn default() -> Self {
        SharedContext {
            base_url: resources::DOMAIN.clone(),
            logo: resources::LOGO_PATH.clone(),
            css: resources::CSS_PATHS.clone(),
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    shared: SharedContext,
    name: &'a str,
}

async fn index() -> Html<Vec<u8>> {
    let templ = IndexTemplate { shared: SharedContext::default(), name: "world" };
    Html(minify_html::minify(templ.render().unwrap().as_bytes(), &Cfg::default()))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    // initialize this to detect problems on startup instead of first request
    SharedContext::default();

    let app = Router::new()
        .route("/api", routing::get(|| async { "{\"error\": \"not yet implemented\"}" }))
        .nest_service("/s", ServeDir::new("static"))
        .route("/", routing::get(index))
        .layer(ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new()));

    let span = span!(Level::INFO, "running_server");
    let _guard = span.enter();
    info!("host = {}", &args.host);
    axum::Server::bind(&args.host.parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
