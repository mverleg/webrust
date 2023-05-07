use ::std::path::PathBuf;

use ::axum::http::Method;
use ::axum::Json;
use ::serde::Serialize;
use axum::http::StatusCode;
use clap::Arg;

use crate::args::Args;
use crate::conf::{Conf, CONF};

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

pub async fn api_conf_get(args: &Args) -> (StatusCode, Json<Status<Conf>>) {
    //TODO @mark: I didn't find an automatic way to do this...
    (
        StatusCode::OK,
        Json(Status {
            is_ok: true,
            msg: "latest config".to_string(),
            data: Some(CONF.get(&args.conf_state_path)),
            //TODO @mark: get this as argument
        })
    )
}

pub async fn api_conf_put(args: &Args) -> Json<Status<Conf>> {
    //TODO @mark: I didn't find an automatic way to do this...
    unimplemented!()
}

pub async fn api_conf_patch(args: &Args) -> Json<Status<Conf>> {
    unimplemented!()
}
