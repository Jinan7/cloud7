use anyhow::Context;
use aws_sdk_s3::operation::create_multipart_upload::CreateMultipartUploadOutput;


pub async fn create_multipart_upload(
    client: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
) -> anyhow::Result<CreateMultipartUploadOutput> {

    let res = client.create_multipart_upload()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .context("Upload failed")?;

    Ok(res)
}