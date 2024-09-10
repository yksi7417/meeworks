use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world 🀅!"
}

async fn index_page() -> &'static str {
    "Hello, world 🀅!"
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(index_page));
    let router = Router::new().route("/hello", get(hello_world));
    Ok(router.into())
}
