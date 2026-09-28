use aws_config::{BehaviorVersion, retry::RetryConfig};

const MAX_RETRIES: u32 = 10;
pub async fn get_s3_client() -> aws_sdk_s3::Client {
    let retry_config = RetryConfig::standard()
        .with_max_attempts(MAX_RETRIES);
    let config = aws_config::defaults(BehaviorVersion::latest())
        .retry_config(retry_config)
        .load()
        .await;
    
    let client = aws_sdk_s3::Client::new(&config);
    client
}