mod api;

use crate::api::api::{ApiResponse, ApiError, Message};
use axum::{routing::get, Json, Router, routing::post};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Submission {
    pub message: String
}

async fn hello_world(
    Json(json): Json<Submission>
) -> Result<ApiResponse, ApiError> {
    println!("{}", json.message);
    
    if json.message.is_empty() {
        return Err(ApiError::BadRequest);
    }
    
    Ok(ApiResponse::JsonData(vec![Message {
        message: format!("Received: {}", json.message),
    }]))
}

async fn index_page() -> &'static str {
    "Index Page"
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(index_page))
        .route("/hello", post(hello_world));  
    Ok(router.into())
}
