use axum::{routing::get, Router};
use anyhow::Context;

async fn hello_world() -> &'static str {
    "Hello, world 🀅!"
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/hello", get(hello_world));

    Ok(router.into())
}
