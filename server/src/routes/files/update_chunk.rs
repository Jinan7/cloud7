use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{get_chunks::get_chunks, key::Key, utils::e500,};

#[derive(Debug, Deserialize)]
pub struct UpdateChunkPayload {
    part_number: i32,
    status: String,
}
pub async fn update_chunk(
    pool: web::Data<PgPool>,
    key: web::Path<Key>,
    payload: web::Form<UpdateChunkPayload>
) -> Result<HttpResponse, actix_web::Error> {

    let mut chunks = get_chunks(&pool, &key.key).await.map_err(e500)?;

    let chunk = chunks
        .iter_mut()
        .find(|chunk| chunk.part_number == payload.0.part_number)
        .ok_or(format!("could not find part"))
        .map_err(e500)?;

    chunk.status = "complete".to_string();

    let pool: &PgPool = &pool;

    sqlx::query_unchecked!(
        r#"
            UPDATE files
            SET chunks = $1
            WHERE name = $2
        "#,
        chunks,
        key.key
    )
    .execute(pool)
    .await
    .map_err(e500)?;

    Ok(HttpResponse::Ok().finish())

}