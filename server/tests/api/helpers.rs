use server::{configuration::get_configuration, startup::Application};

pub struct TestApp {
    pub addr: String,
    pub client: reqwest::Client,
}

pub async fn spawn_app() -> TestApp {

    let config = {
        let mut config = get_configuration();
        config.application.port = 0;
        config
    };

    let app = Application::build(config).await;
    let port = app.port;
    tokio::spawn(app.run());

    let addr = format!("http://127.0.0.1:{}",port);
    let client = reqwest::Client::new();
    TestApp {
        addr,
        client
    }
}