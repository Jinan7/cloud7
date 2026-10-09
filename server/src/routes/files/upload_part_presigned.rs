use actix_web::{HttpResponse, web};

use crate::{key::Key, startup::S3Bucket};


async fn upload_part_presigned(
    bucket: web::Data<S3Bucket>,
    key: web::Path<Key>
) -> Result<HttpResponse, actix_web::Error> {

    Ok(HttpResponse::Ok().finish())
}