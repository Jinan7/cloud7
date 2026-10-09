use crate::helpers::spawn_app;

#[tokio::test]
async fn upload_part_presigned_returns_valid_presigned_url() {

    let app = spawn_app().await;

    let body = serde_json::json!(
        {
            "key": "example_file.txt",
            "upload_id": &uuid::Uuid::new_v4().to_string(),
            "part_number": 1,
        }
    );

    let response = app.client
        .post(format!("{}/files/upload_part_presigned", app.addr))
        .json(&body)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status().as_u16(), 200);

    let response = String::from_utf8(response.bytes().await.unwrap().to_vec()).expect("Failed to deserialize response");

    dbg!(response);
}