#![forbid(unsafe_code)]
#![feature(lazy_cell)]

use ::askama::Template;
use ::askama_axum::Response;
use ::axum::response::Html;
use ::axum::Router;
use ::axum::routing;
use ::clap::Parser;
use ::minify_html::Cfg;
use ::tower::ServiceBuilder;
use ::tower::ServiceExt;
use ::tower_http::compression::CompressionLayer;
use ::tower_http::limit::ResponseBody;
use ::tower_http::services::fs::ServeFileSystemResponseBody;
use ::tower_http::services::ServeDir;
use ::tower_http::trace::TraceLayer;
use ::tracing::info;
use ::tracing::Level;
use ::tracing::span;
use ::tracing_subscriber;
use axum::http::Method;
use tower_http::cors;
use tower_http::cors::CorsLayer;

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
        // .nest_service("/s", ServeDir::new("static").map_response(|mut resp: Response<ServeFileSystemResponseBody>| {
        //     // if resp.status() == StatusCode::OK || resp.status() == StatusCode::NOT_MODIFIED {
        //     //     resp.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("public, max-age=604800"));
        //     // }
        //     //TODO @mark: minify css?
        //     resp
        // }))
        .route("/", routing::get(index))
        .layer(ServiceBuilder::new()
            .layer(CorsLayer::new().allow_methods([Method::HEAD, Method::GET]).allow_origin(cors::Any))
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new())
        );

    let span = span!(Level::INFO, "running_server");
    let _guard = span.enter();
    info!("host = {}", &args.host);
    axum::Server::bind(&args.host.parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
