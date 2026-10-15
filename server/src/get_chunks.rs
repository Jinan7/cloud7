use sqlx::PgPool;


#[derive(Debug, serde::Deserialize, sqlx::Type)]
#[sqlx(type_name = "chunk")]
pub struct Chunk {
    pub part_number: i32,
    pub status: String
}
pub async fn get_chunks(
    pool: &PgPool,
    key: &str
) -> Result<Vec<Chunk>, anyhow::Error>{

    let chunks = sqlx::query!(
        r#"
            SELECT chunks as "chunks: Vec<Chunk>"
            FROM files
            WHERE name = $1
        "#,
        key
    )
    .fetch_one(pool)
    .await?;


    
    Ok(chunks.chunks)
}