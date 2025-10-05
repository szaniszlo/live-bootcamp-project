use crate::helpers::TestApp;

#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn non_existent_route_returns_404() {
    let app = TestApp::new().await;

    let response = app.http_client
        .get(format!("{}/non-existent", app.address))
        .send()
        .await
        .expect("Failed to execute GET request.");

    assert_eq!(response.status().as_u16(), 404);
}
