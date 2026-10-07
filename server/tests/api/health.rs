use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check() {

    let app = spawn_app().await;

    let response = app.client
        .get(format!("{}/health_check", app.addr))
        .send()
        .await
        .expect("failed to send request");

    assert_eq!(response.status().as_u16(), 200)

}