use crate::helpers::TestApp;
use reqwest::Body;

#[tokio::test]
async fn login_returns_200() {
    let app = TestApp::new().await;

    let body = serde_json::json!({});
    let response = app.post_login(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}