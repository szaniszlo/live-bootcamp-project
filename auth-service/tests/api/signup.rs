use crate::helpers::{TestApp, get_random_email};


#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases= [
        serde_json::json!(""),
        serde_json::json!({
            "email": "some@example.com",
            "password": "short",
            "requires2FA": "blablah"
        }),
        serde_json::json!({
            "password": "short",
            "requires2FA": true
        }),
    ];

    for test_case in test_cases {
        let response = app.post_signup(&test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }        

}

#[tokio::test]
async fn should_return_200_for_valid_input() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
        "email": get_random_email(),
        "password": "validpassword",
        "requires2FA": true
    });
    let response = app.post_signup(&body).await;

    assert_eq!(response.status().as_u16(), 200);
}
