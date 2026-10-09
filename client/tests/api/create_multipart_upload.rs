#[tokio::test]
async fn create_multipart_upload() {

    let key = "example_file.txt";
    let response = client::s3_v2::actions::create_multipart_upload_v2(key)
        .await
        .expect("failed to send request");

    dbg!(response);
}