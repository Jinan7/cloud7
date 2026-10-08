use anyhow::Context;
use aws_sdk_s3::operation::{create_multipart_upload::CreateMultipartUploadOutput, get_object::GetObjectError::NoSuchKey};

pub async fn create_multipart_upload(
    client: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
) -> Result<CreateMultipartUploadOutput, anyhow::Error> {

    let response = client.create_multipart_upload()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .context("Failed to create multipart upload")?;

    Ok(response)
}