use actix_web::{HttpResponse, web};
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use serde::Deserialize;

use crate::{s3, startup::S3Bucket, utils::e500};

#[derive(Debug, Deserialize)]
pub struct Part {
    pub e_tag: String,
    pub part_number: i32
}

#[derive(Debug, Deserialize)]
pub struct CompleteFormData {
    pub key: String,
    pub upload_id: String,
    pub parts: Vec<Part>
}

pub async fn complete_multipart_upload(
    client: web::Data<aws_sdk_s3::Client>,
    bucket: web::Data<S3Bucket>,
    form: web::Json<CompleteFormData>
) -> Result<HttpResponse, actix_web::Error> {

    let parts = form.0.parts.iter()
        .map(|part| {

            CompletedPart::builder()
                .e_tag(part.e_tag.to_string())
                .part_number(part.part_number)
                .build()
        })
        .collect::<Vec<_>>();

    let parts = CompletedMultipartUpload::builder()
        .set_parts(Some(parts))
        .build();
        

    s3::complete_multipart_upload(
        &client, 
        &bucket.0, 
        &form.0.key, 
        &form.0.upload_id, 
        parts
    )
    .await
    .map_err(e500)?;

    Ok(HttpResponse::Ok().finish())
}