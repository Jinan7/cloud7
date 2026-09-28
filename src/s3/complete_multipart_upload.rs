use anyhow::Context;
use aws_sdk_s3::{operation::complete_multipart_upload, types};


pub async fn complete_multipart_upload(
    client: aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
    upload_id: &str,
    completed_multipart_upload: types::CompletedMultipartUpload
) -> anyhow::Result<complete_multipart_upload::CompleteMultipartUploadOutput> {

    let res = client.complete_multipart_upload()
        .bucket(bucket)
        .key(key)
        .upload_id(upload_id)
        .multipart_upload(completed_multipart_upload)
        .send()
        .await
        .context("Upload failed")?;

    Ok(res)
}