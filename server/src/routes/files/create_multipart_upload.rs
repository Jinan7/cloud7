use actix_web::{HttpResponse, web};
use crate::{s3::{self, get_config, s3client}, startup::S3Bucket, utils::e500};

#[derive(serde::Deserialize)]
pub struct Key {
    key: String
}
pub async fn create_multipart_upload(
    bucket: web::Data<S3Bucket>,
    key: web::Path<Key>,
) -> Result<HttpResponse, actix_web::Error> {

    let config = get_config().await;
    let client = s3client(config).await;
    s3::create_multipart_upload(
        &client,
        &bucket.0,
        &key.key
    )
    .await
    .map_err(e500)?;

    Ok(HttpResponse::Ok().finish())
}