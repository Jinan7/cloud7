use actix_web::{HttpResponse, web};
use serde::Serialize;
use crate::{key::Key, s3::{self, get_config, s3client}, startup::S3Bucket, utils::e500};

#[derive(Serialize)]
struct CreateMultipartUploadResponse {
    upload_id: String,
}

pub async fn create_multipart_upload(
    bucket: web::Data<S3Bucket>,
    key: web::Path<Key>,
) -> Result<HttpResponse, actix_web::Error> {

    let config = get_config().await;
    let client = s3client(config).await;
    let output = s3::create_multipart_upload(
        &client,
        &bucket.0,
        &key.key
    )
    .await
    .map_err(e500)?;

    let upload_id = output.upload_id
        .ok_or(anyhow::anyhow!("Invalid upload ID"))
        .map_err(e500)?;

    let response_body = CreateMultipartUploadResponse {
        upload_id
    };

    Ok(
        HttpResponse::Ok()
        .json(response_body)
    )
}