use auth_service::Application;
use uuid::Uuid;

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

    
    async fn post<Body>(&self, path: &str, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(self.to_url(Some(path)))
            .json(body)
            .send()
            .await
            .expect("Failed to execute POST request.")
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.get(None).await
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize
    {
        self.post("signup", body).await
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize
    {
        self.post("login", body).await
    }

    pub async fn post_logout<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize
    {
        self.post("logout", body).await
    }

    pub async fn post_verify_2fa<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize
    {
        self.post("verify-2fa", body).await
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize
    {
        self.post("verify-token", body).await
    }

}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
