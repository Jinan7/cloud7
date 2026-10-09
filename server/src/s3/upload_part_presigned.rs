use std::time::Duration;

use aws_sdk_s3::presigning::{PresigningConfig};

pub async fn upload_part_presigned(
    client: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
    upload_id: &str,
    part_number: i32,
    expires_in: u64,
) -> Result<String, anyhow::Error> {

    let expires_in = {
        let expires_in = Duration::from_secs(expires_in);
        PresigningConfig::expires_in(expires_in)?
    };
    
    let presigned_req = client.upload_part()
        .bucket(bucket)
        .key(key)
        .upload_id(upload_id)
        .part_number(part_number)
        .presigned(expires_in)
        .await
        .or_else(
            |e| {
                dbg!(&e);
                return Err(anyhow::anyhow!(e))
            }
        )
        .unwrap();

    Ok(presigned_req.uri().into())

}