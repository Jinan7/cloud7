use client::s3_v2::actions::upload_part_presigned;

#[tokio::test]
async fn upload_part_presigned_returns_valid_presigned_url() {

    let key = "my_example_file.txt";
    let upload_id = uuid::Uuid::new_v4().to_string();
    let part_number = 1;

    let response = upload_part_presigned(key, &upload_id, part_number)
        .await
        .expect("Failed to send request");

    dbg!(response);
}