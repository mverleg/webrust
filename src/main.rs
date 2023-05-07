#![forbid(unsafe_code)]
#![feature(lazy_cell)]
#![feature(async_closure)]

use std::sync::Arc;
use std::time::Duration;

use ::askama::Template;
use ::askama_axum::IntoResponse;
use ::askama_axum::Response;
use ::axum::error_handling::{HandleError, HandleErrorLayer};
use ::axum::extract::FromRef;
use ::axum::http::{header, HeaderValue, Request, StatusCode};
use ::axum::http::Method;
use ::axum::middleware::map_response;
use ::axum::response::Html;
use ::axum::Router;
use ::axum::routing;
use ::axum::ServiceExt;
use ::clap::Parser;
use ::minify_html::Cfg;
use ::tower::Service;
use ::tower::ServiceBuilder;
use ::tower_http::compression::CompressionLayer;
use ::tower_http::cors;
use ::tower_http::cors::CorsLayer;
use ::tower_http::limit::ResponseBody;
use ::tower_http::normalize_path::NormalizePath;
use ::tower_http::services::fs::ServeFileSystemResponseBody;
use ::tower_http::services::ServeDir;
use ::tower_http::trace::TraceLayer;
use ::tracing::info;
use ::tracing::Level;
use ::tracing::span;
use ::tracing_subscriber;

use crate::api::{api_conf_get, api_conf_patch, api_conf_put, api_index};
use crate::args::Args;
use crate::conf::{Conf, ConfContainer};

#[cfg(feature = "jemalloc")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod args;
mod conf;
mod api;
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

#[derive(Template)]
#[template(path = "notification.html")]
struct NotificationTemplate<'a> {
    shared: SharedContext,
    title: &'a str,
    message: &'a str,
    is_err: bool,
}

async fn index() -> Html<Vec<u8>> {
    let templ = IndexTemplate { shared: SharedContext::default(), name: "world" };
    Html(minify_html::minify(templ.render().unwrap().as_bytes(), &Cfg::default()))
}

async fn not_found() -> impl IntoResponse {
    let templ = NotificationTemplate { shared: SharedContext::default(), title: "Not found", is_err: true,
        message: "The page you were looking for could not be found. There might be a typo in the address, or the page might have moved or disappeared." };
    (StatusCode::NOT_FOUND, Html(minify_html::minify(templ.render().unwrap().as_bytes(), &Cfg::default())))
}

async fn add_cache_control_header<R>(mut response: Response<R>) -> Response<R> {
    if response.status() == StatusCode::OK || response.status() == StatusCode::NOT_MODIFIED {
        response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("public, max-age=604800"));
    }
    response
}

#[derive(Clone)]
pub struct AppState {
    args: Arc<Args>,
    conf_container: ConfContainer,
}

impl AppState {
    pub fn conf(&self) -> Arc<Conf> {
        self.conf_container.get(&self.args.conf_state_path)
    }
}

impl FromRef<AppState> for Arc<Args> {
    fn from_ref(state: &AppState) -> Self {
        state.args.clone()
    }
}
//TODO @mark: probably not that useful since we seem unable to get two states per handler

// impl FromRef<AppState> for ConfContainer {
//     fn from_ref(state: &AppState) -> Self {
//         state.conf_container.clone()
//     }
// }
//TODO @mark: ^

impl FromRef<AppState> for Arc<Conf> {
    fn from_ref(state: &AppState) -> Self {
        state.conf_container.get(&state.args.conf_state_path)
    }
}
//TODO @mark: probably not that useful since we seem unable to get two states per handler

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Arc::new(Args::parse());
    // initialize this to detect problems on startup instead of first request
    SharedContext::default();

    let conf_container = ConfContainer::empty();
    //TODO @mark: since we do this in main instead of static, we may get rid op optional ^
    let state = AppState { args: args.clone(), conf_container };

    let app = Router::new()
        .nest("/api", Router::new()
            .route("/", routing::get(api_index))
            .route("/conf", routing::get(api_conf_get))
            .route("/conf", routing::put(api_conf_put))
            .route("/conf", routing::patch(api_conf_patch)))
            .with_state(state)
            //TODO @mark: pass args in more elegant way
        .route("/", routing::get(index))
        // .nest_service("/s", ServeDir::new("static"));
        .nest("/s", Router::new()
            .nest_service("/", ServeDir::new("static"))
            .layer(map_response(add_cache_control_header)))
        .fallback(not_found)
            //TODO @mark: is this weird double-route structure needed? I tried quite a while to simplify but didn't succeed
        // .layer(map_response(add_cache_control_header)))
        // .nest_service("/s", <ServeDir as tower::ServiceExt<Request<ReqBody>>>::map_response::<fn(Response<ServeFileSystemResponseBody>) -> Response<ServeFileSystemResponseBody> {add_cache_control_header::<ServeFileSystemResponseBody>}, Response<ServeFileSystemResponseBody>>(ServeDir::new("static"), add_cache_control_header))
        .layer(ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())  // first because needs other layers to be Default
            .layer(CorsLayer::new().allow_methods([Method::HEAD, Method::GET]).allow_origin(cors::Any))
            .layer(CompressionLayer::new())
        );
    let app = NormalizePath::trim_trailing_slash(app);

    let span = span!(Level::INFO, "running_server");
    let _guard = span.enter();
    info!("host = {}", &args.host);
    axum::Server::bind(&args.host.parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
