use ::std::path::PathBuf;
use std::sync::Arc;

use ::axum;
use ::axum::extract;
use ::axum::http::Method;
use ::axum::http::StatusCode;
use ::axum::Json;
use ::clap::Arg;
use ::serde::Serialize;
use ::tracing::info;

use crate::AppState;
use crate::args::Args;
use crate::conf::Conf;

#[derive(Debug, Serialize)]
pub struct Status<D: Serialize> {
    is_ok: bool,
    msg: String,
    data: Option<D>,
}

#[derive(Debug, Serialize)]
pub struct ApiIndex {
    /// url -> method -> description
    endpoints: Vec<(&'static str, Vec<(&'static str, &'static str)>)>,
}

pub async fn api_index() -> Json<ApiIndex> {
    //TODO @mark: I didn't find an automatic way to do this...
    Json(ApiIndex {
        endpoints: vec![
            ("/conf", vec![
                ("GET", "get the current configuration"),
                ("PUT", "overwrite the current configuration"),
                ("PATCH", "partial configuration updates"),
            ]),
        ]
    })
}

pub async fn api_conf_get(extract::State(state): extract::State<AppState>) -> (StatusCode, Json<Status<Conf>>) {
    (
        StatusCode::OK,
        Json(Status {
            is_ok: true,
            msg: "latest config".to_string(),
            data: Some((*state.conf()).clone()),
        })
    )
}

// curl -XPUT -H "Content-Type: application/json" http://localhost:8080/api/conf -d '{"name":"Hello world!","score":37}'
// curl -XGET curl http://localhost:8080/api/conf
pub async fn api_conf_put(extract::State(state): extract::State<AppState>, extract::Json(conf): extract::Json<Conf>) -> (StatusCode, Json<Status<Conf>>) {
    info!("updating conf to {:?}", &conf);
    state.conf_container.set(&state.args.conf_state_path, conf.clone());
    (
        StatusCode::OK,
        Json(Status {
            is_ok: true,
            msg: "latest config".to_string(),
            data: Some(conf),
        })
    )
}

pub async fn api_conf_patch(extract::State(args): extract::State<AppState>) -> (StatusCode, Json<Status<Conf>>) {
    unimplemented!()
}
