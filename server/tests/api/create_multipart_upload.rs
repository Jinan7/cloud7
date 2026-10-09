use crate::helpers::spawn_app;

#[tokio::test]
async fn create_multipart_upload_returns_200() {

    let app = spawn_app().await;
    let key = "example_file.txt";
    let response = app.client.get(format!("{}/files/create_multipart/{}", app.addr, key))
        .send()
        .await
        .expect("failed to create multipart upload");

    assert_eq!(response.status().as_u16(), 200);
    let response = String::from_utf8(response.bytes().await.unwrap().to_vec()).expect("Failed to deserialize response");

    dbg!(response);
}