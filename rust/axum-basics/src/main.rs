use axum::{
    Router, extract::Path, http::StatusCode, response::{IntoResponse, Json}, routing::get
};
use serde_json::{Value, json};

#[derive(Debug)]
enum ApiError {
    NotFound,
    InvalidInput(String),
    InternalError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Data not found".to_string()),
            ApiError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Server is running",
    }))
}

async fn list_users() -> Result<Json<Value>, ApiError> {
    Err(ApiError::InternalError)
}

async fn get_user(Path(id): Path<u32>) -> Result<Json<Value>, ApiError> {
    if id > 100 {
        return Err(ApiError::NotFound);
    }

    Ok(Json(json!({
        "id": id,
        "name": "user",
    })))
}

fn create_app() -> Router {
    Router::new()
    .route("/health", get(health_check))
    .route("/users", get(list_users))
    .route("/users/{id}", get(get_user))
}

#[tokio::main]
async fn main() {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind tcp listener");

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("failed to start server");
}
