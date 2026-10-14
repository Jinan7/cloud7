use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::utils::e500;

const PAGE_LIMIT: i64 = 50;
#[derive(Deserialize)]
pub struct GetFilesQuery {
    pub page: i64,
}

#[derive(Serialize)]
pub struct File {
    id: String,
    name: String,
    size: i32,
}

#[derive(Serialize)]
pub struct GetFilesResponse {
    files: Vec<File>
}

pub async fn get(
    pool: web::Data<PgPool>,
    query: web::Query<GetFilesQuery>
) -> Result<HttpResponse, actix_web::Error> {

    let pool: &PgPool = &pool;
    let offset = (query.0.page - 1) * PAGE_LIMIT;

    let records = sqlx::query!(
        r#"
            SELECT id, name, size
            FROM files 
            LIMIT $1
            OFFSET $2
        "#,
        PAGE_LIMIT,
        offset
    )
    .fetch_all(pool)
    .await
    .map_err(e500)?;


    let files = records.iter()
        .map(|record| {
            
            File {
                id: record.id.to_string(),
                name: record.name.clone(),
                size: record.size
            }
        })
        .collect::<Vec<_>>();

    let files = GetFilesResponse { files };



    Ok(
        HttpResponse::Ok()
            .json(&files)
    )
}