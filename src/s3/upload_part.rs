use anyhow::Context;
use aws_sdk_s3::{operation::upload_part::UploadPartOutput, primitives};


pub async fn upload_part(
    client: aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
    upload_id: &str,
    part_number: i32,
    body: primitives::ByteStream
) -> anyhow::Result<UploadPartOutput> {

    let res = client.upload_part()
        .bucket(bucket)
        .key(key)
        .upload_id(upload_id)
        .part_number(part_number)
        .body(body)
        .send()
        .await
        .context(format!("Error uploading part {}", part_number))?;

    Ok(res)
}