use actix_web::{HttpResponse, web};

use crate::{key::Key, s3::download_file_presigned, startup::{PresignedExpiresIn, S3Bucket}, utils::e500};

#[derive(serde::Serialize)]
pub struct GetObjectResponse {
    pub presigned_url: String
}
pub async fn get(
    client: web::Data<aws_sdk_s3::Client>,
    bucket: web::Data<S3Bucket>,
    key: web::Path<Key>,
    expires_in: web::Data<PresignedExpiresIn>
) -> Result<HttpResponse, actix_web::Error> {

    let presigned_url = download_file_presigned(
        &client, 
        &bucket.0, 
        &key.key, 
        expires_in.0
    )
    .await
    .map_err(e500)?;

    let response = GetObjectResponse { presigned_url };
    Ok(
        HttpResponse::Ok()
        .json(response)
    )
}