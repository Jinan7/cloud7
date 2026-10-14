use std::time::Duration;
use aws_sdk_s3::{presigning::PresigningConfig};

pub async fn download_file_presigned(
    client: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
    expires_in: u64,
) -> Result<String, anyhow::Error> {

    let expires_in = {
        let expires_in = Duration::from_secs(expires_in);
        PresigningConfig::expires_in(expires_in)?
    };
    

    let response = client.get_object()
        .bucket(bucket)
        .key(key)
        .presigned(expires_in)
        .await?;

    Ok(response.uri().into())
}