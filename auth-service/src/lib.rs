// This struct encapsulates our application-related logic.
use axum::{response::IntoResponse, routing::post, serve::Serve, Router, http::StatusCode};
use tower_http::services::ServeDir;
use std::error::Error;

pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup_handler))
            .route("/login", post(login_handler))
            .route("/logout", post(logout_handler))
            .route("/verify-2fa", post(verify_2fa_handler))
            .route("/verify-token", post(verify_token_handler));

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Application {
            server,
            address,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

async fn signup_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn login_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn logout_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn verify_2fa_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn verify_token_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
