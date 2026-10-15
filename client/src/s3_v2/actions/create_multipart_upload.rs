use serde::{Deserialize};

const CHUNK_SIZE: u64 = 5 * 1024 * 1024;

#[derive(Debug, Deserialize)]
pub struct CreateMultipartUploadResponse {
    pub upload_id: String,
    pub file_id: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateMultipartUploadPayload {
    pub mime: String,
    pub chunks: Vec<Chunk>
}

#[derive(Debug, serde::Serialize)]
pub struct Chunk {
    pub part_number: i32,
    pub status: String,
}

pub async fn create_multipart_upload_v2(
    key: &str,
    mime: &str,
    size: u64
) -> Result<CreateMultipartUploadResponse, anyhow::Error>{

    let number_of_parts = {
        let n = size / CHUNK_SIZE;

        if size % CHUNK_SIZE != 0 {
            n + 1
        } else {
            n
        }
    };
    

    let mut chunks = vec![];

    for i in 0..number_of_parts {

        let chunk = Chunk { part_number: i as i32 + 1, status: "pending".to_string()};
        chunks.push(chunk);
    }


    let payload = CreateMultipartUploadPayload {
        mime: mime.to_string(),
        chunks
    };

    let payload = serde_json::to_string(&payload)?;
    let response = reqwest::Client::new()
        .post(format!("http://localhost:8000/files/create_multipart/{}", key))
        .header("Content-Type", "application/json")
        .body(payload)
        .send()
        .await?;

    let response_body = String::from_utf8(response.bytes().await?.to_vec())?;
    
    let output: CreateMultipartUploadResponse = serde_json::from_str(&response_body)?;

    Ok(output)
}