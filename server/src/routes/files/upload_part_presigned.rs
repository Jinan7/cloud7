use actix_web::{HttpResponse, web};
use serde::Deserialize;

use crate::{s3, startup::{PresignedExpiresIn, S3Bucket}, utils::e500};

#[derive(Debug, Deserialize)]
pub struct FormData {
    key: String,
    upload_id: String,
    part_number: i32,
}

#[derive(serde::Serialize)]
pub struct UploadPartResponse {
    presigned_url: String
}
pub async fn upload_part_presigned(
    client: web::Data<aws_sdk_s3::Client>,
    bucket: web::Data<S3Bucket>,
    file: web::Json<FormData>,
    expires_in: web::Data<PresignedExpiresIn>
) -> Result<HttpResponse, actix_web::Error> {

    let presigned_url = s3::upload_part_presigned(
        &client, 
        &bucket.0, 
        &file.0.key, 
        &file.0.upload_id,
        file.0.part_number,
        expires_in.0
    )
    .await
    .map_err(e500)?;

    let response = UploadPartResponse { presigned_url};
    let response = HttpResponse::Ok()
        .json(response);
    Ok(response)
}