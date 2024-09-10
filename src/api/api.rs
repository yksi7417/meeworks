use axum::{response::{Response, IntoResponse}, Json, http::StatusCode};
use serde::Serialize;

// here we show a type that implements Serialize + Send
#[derive(Serialize)]
pub struct Message {
    pub message: String
}

impl Default for Message {
    fn default() -> Self {
        Message {
            message: String::new(),
        }
    }
}

pub enum ApiResponse {
    // OK,
    // Created,
    JsonData(Vec<Message>),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            // Self::OK => (StatusCode::OK).into_response(),
            // Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response()
        }
    }
}

pub enum ApiError {
    BadRequest,
    // Forbidden,
    // Unauthorised,
    // InternalServerError
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::BadRequest => StatusCode::BAD_REQUEST,
            // Self::Forbidden => StatusCode::FORBIDDEN,
            // Self::Unauthorised => StatusCode::UNAUTHORIZED,
            // Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };
        status.into_response()
    }
}

