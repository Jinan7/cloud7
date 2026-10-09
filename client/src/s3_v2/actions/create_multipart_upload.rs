use serde::{Deserialize};


#[derive(Debug, Deserialize)]
pub struct CreateMultipartUploadResponse {
    pub upload_id: String
}

pub async fn create_multipart_upload_v2(
    key: &str
) -> Result<CreateMultipartUploadResponse, anyhow::Error>{

    let response = reqwest::Client::new()
        .get("http://localhost:8000/files/create_multipart/{key}")
        .send()
        .await?;

    let response_body = String::from_utf8(response.bytes().await?.to_vec())?;
    
    let output: CreateMultipartUploadResponse = serde_json::from_str(&response_body)?;

    Ok(output)
}