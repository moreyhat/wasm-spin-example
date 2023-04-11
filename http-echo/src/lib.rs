use anyhow::Result;
use bytes::Bytes;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[http_component]
fn handle_echo(req: Request) -> Result<Response> {
    let body = match req.body() {
        Some(d) => d.clone(),
        None => Bytes::from(""),
    };

    Ok(http::Response::builder()
        .status(200)
        .body(Some(body.into()))?)
}
