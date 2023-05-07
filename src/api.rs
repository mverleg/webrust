use ::std::path::PathBuf;
use std::sync::Arc;

use ::axum::extract::State;
use ::axum::http::Method;
use ::axum::http::StatusCode;
use ::axum::Json;
use ::clap::Arg;
use ::serde::Serialize;
use ::axum;
use crate::AppState;

use crate::args::Args;
use crate::conf::Conf;

#[derive(Debug, Serialize)]
pub struct Status<'a, D: Serialize> {
    is_ok: bool,
    msg: String,
    data: Option<&'a D>,
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

pub async fn api_conf_get(State(state): State<AppState>) -> (StatusCode, Json<Vec<u8>>) {
    let conf_ref = state.conf();
    let conf = conf_ref;
    //TODO @mark: cannot borrow conf (because lifetimes) and cannot use Arc (because Serde), so clone
    let mut json = Vec::new();
    serde_json::to_writer(&mut json, &Status {
        is_ok: true,
        msg: "latest config".to_string(),
        data: Some(&*conf),
        //TODO @mark: get this as argument
    }).unwrap();
    (
        StatusCode::OK,
        Json(json)
    )
}

pub async fn api_conf_put(State(args): State<Arc<Args>>) -> (StatusCode, Json<Vec<u8>>) {
    //TODO @mark: I didn't find an automatic way to do this...
    unimplemented!()
}

pub async fn api_conf_patch(State(args): State<Arc<Args>>) -> (StatusCode, Json<Vec<u8>>) {
    unimplemented!()
}