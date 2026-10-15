use actix_web::{HttpResponse, web};
use serde::Serialize;

use crate::{key::Key, s3, startup::S3Bucket, utils::e500};

#[derive(Serialize)]
struct HeadObjectResponse {
    content_length: i64
}
pub async fn head_object(
    client: web::Data<aws_sdk_s3::Client>,
    bucket: web::Data<S3Bucket>,
    key: web::Path<Key>
) -> Result<HttpResponse, actix_web::Error> {

    let response = s3::head_object(
        &client, 
        &bucket.0, 
        &key.key
    )
    .await
    .map_err(e500)?;

    let content_length = response
        .content_length()
        .ok_or(anyhow::anyhow!("invalid content length"))
        .map_err(e500)?;

    let payload = HeadObjectResponse { content_length };

    Ok(
        HttpResponse::Ok()
        .json(&payload)
    )
}