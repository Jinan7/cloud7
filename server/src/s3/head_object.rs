use aws_sdk_s3::operation::head_object::HeadObjectOutput;


pub async fn head_object (
    client: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str
) -> anyhow::Result<HeadObjectOutput> {

    let response = client.head_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;

    Ok(response)
}