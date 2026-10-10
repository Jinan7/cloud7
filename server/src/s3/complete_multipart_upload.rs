use aws_sdk_s3::{operation::complete_multipart_upload::CompleteMultipartUploadOutput, types::{CompletedMultipartUpload}};

pub async fn complete_multipart_upload(
    client: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
    upload_id: &str,
    parts: CompletedMultipartUpload
) -> Result<CompleteMultipartUploadOutput, anyhow::Error> {

    let response = client
    .complete_multipart_upload()
    .bucket(bucket)
    .key(key)
    .upload_id(upload_id)
    .multipart_upload(parts)
    .send()
    .await?;

    Ok(response)
}