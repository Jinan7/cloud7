use anyhow::Context;
use crate::s3::{create_multipart_upload, upload_parts};


pub async fn multipart_upload(
    client: aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
    file: &[u8],
    file_size: u64,
) -> anyhow::Result<()> {

    let create_multipart_upload_res = create_multipart_upload(&client, bucket, key)
        .await
        .context("Upload failed")?;

    let upload_id = create_multipart_upload_res
        .upload_id()
        .context("Upload failed")?;

    upload_parts(&client, bucket, key, upload_id, file, file_size)
        .await
        .context("Upload failed")?;

    Ok(())
}