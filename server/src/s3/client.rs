pub fn s3client(
    config: aws_config::SdkConfig
) -> aws_sdk_s3::Client {
    aws_sdk_s3::Client::new(&config)
}