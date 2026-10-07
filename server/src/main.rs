use server::{configuration::get_configuration, startup::Application};

#[tokio::main]
async fn main() {
    
    let config = get_configuration();
    let app = Application::build(config).await;
    app.run().await;
}
