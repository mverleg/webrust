#![feature(lazy_cell)]

use ::askama::Template;
use ::axum::http::HeaderValue;
use ::axum::response::Html;
use ::axum::Router;
use ::axum::routing;
use ::clap::Parser;
use ::hyper::header;
use ::minify_html::Cfg;
use ::time;
use ::tower::ServiceBuilder;
use ::tower::ServiceExt;
use ::tower_http::compression::CompressionLayer;
use ::tower_http::services::ServeDir;
use ::tower_http::trace::TraceLayer;
use ::tracing::info;
use ::tracing::Level;
use ::tracing::span;
use ::tracing_subscriber;
use ::time::OffsetDateTime;
use ::time::format_description;

use crate::args::Args;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod args;
mod resources;

// static FAR_FUTURE: &'static str = &OffsetDateTime::now_utc().format("%a, %d %b %Y %H:%M:%S %Z").unwrap();

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

    let datetime_format = format_description::parse("%a, %d %b %Y %H:%M:%S %Z").unwrap();
    info!("expires: {}", OffsetDateTime::now_utc().format(&datetime_format).unwrap());  //TODO @mark: TEMPORARY! REMOVE THIS!
    let app = Router::new()
        .route("/api", routing::get(|| async { "{\"error\": \"not yet implemented\"}" }))
        //.nest_service("/s", ServeDir::new("static").map_response(|mut resp| resp.headers_mut().insert(header::EXPIRES, HeaderValue::from_static(FAR_FUTURE))))
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
