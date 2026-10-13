use actix_web::{HttpResponse, web};
use serde::Serialize;
use sqlx::{PgPool};
use crate::{key::Key, s3, startup::S3Bucket, utils::e500};

#[derive(Serialize)]
struct CreateMultipartUploadResponse {
    upload_id: String,
    file_id: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateMultipartUploadPayload {
    pub mime: String,
}
pub async fn create_multipart_upload(
    pool: web::Data<PgPool>,
    client: web::Data<aws_sdk_s3::Client>,
    bucket: web::Data<S3Bucket>,
    key: web::Path<Key>,
    payload: web::Json<CreateMultipartUploadPayload>
) -> Result<HttpResponse, actix_web::Error> {

    let pool: &PgPool = &pool;
    
    let output = s3::create_multipart_upload(
        &client,
        &bucket.0,
        &key.key
    )
    .await
    .map_err(e500)?;

    let file_id = uuid::Uuid::new_v4();

    sqlx::query!(
        r#"
            INSERT INTO files (id, name, mime, size, status)
            VALUES ($1, $2, $3, 0, 'pending')
        "#,
        file_id,
        &key.key,
        &payload.mime
    )
    .execute(pool)
    .await
    .map_err(e500)?;
    

    let upload_id = output.upload_id
        .ok_or(anyhow::anyhow!("Invalid upload ID"))
        .map_err(e500)?;

    let response_body = CreateMultipartUploadResponse {
        upload_id,
        file_id: file_id.to_string(),
    };

    Ok(
        HttpResponse::Ok()
        .json(response_body)
    )
}