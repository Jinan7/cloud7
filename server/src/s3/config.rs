use aws_config::{BehaviorVersion, SdkConfig};

pub async fn get_config() -> SdkConfig {

    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;

    config
}