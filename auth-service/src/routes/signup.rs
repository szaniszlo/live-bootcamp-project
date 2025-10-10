use axum::{response::IntoResponse, http::StatusCode, Json};

pub async fn signup_handler(Json(request): Json<SignupRequest>) -> impl IntoResponse {
    println!("signup request for email: {}", request.email);
    StatusCode::OK.into_response()
}

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct SignupRequest {
    email: String,
    password: String,
    #[serde(rename = "requires2FA")]
    requires_2fa: bool,
}
