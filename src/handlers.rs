use axum::{response::IntoResponse, Json};

pub async fn user_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Users";

    let json_response = serde_json::json!({
        "message": MESSAGE,
        "status": "OK"
    });

    return Json(json_response);
}

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API";

    let json_response = serde_json::json!({
        "message": MESSAGE,
        "status": "OK"
    });

    return Json(json_response);
}
