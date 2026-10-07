use server::startup::Application;

#[tokio::main]
async fn main() {
    
    let app = Application::build().await;
    app.run().await;
}
