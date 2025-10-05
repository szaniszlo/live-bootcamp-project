use auth_service::Application;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address);

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new();

        Self {
            address,
            http_client
        }
    }

    fn to_url(&self, path: Option<&str>) -> String {
        match path {
            Some(path) => format!("{}/{}", self.address, path),
            None => format!("{}", self.address)        
        }
    }

    async fn get(&self, path: Option<&str>) -> reqwest::Response {
        self.http_client
            .get(self.to_url(path))
            .send()
            .await
            .expect("Failed to execute GET request.")
    }

    async fn post(&self, path: &str) -> reqwest::Response {
        self.http_client
            .post(self.to_url(Some(path)))
            .send()
            .await
            .expect("Failed to execute POST request.")
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.get(None).await
    }

    pub async fn signup(&self) -> reqwest::Response {
        self.post("signup").await
    }

    pub async fn login(&self) -> reqwest::Response {
        self.post("login").await
    }

    pub async fn logout(&self) -> reqwest::Response {
        self.post("logout").await
    }

    pub async fn verify_2fa(&self) -> reqwest::Response {
        self.post("verify-2fa").await
    }

    pub async fn verify_token(&self) -> reqwest::Response {
        self.post("verify-token").await
    }

}
