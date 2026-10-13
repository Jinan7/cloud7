use serde::{Deserialize};


#[derive(Debug, Deserialize)]
pub struct CreateMultipartUploadResponse {
    pub upload_id: String,
    pub file_id: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateMultipartUploadPayload {
    pub mime: String
}

pub async fn create_multipart_upload_v2(
    key: &str,
    mime: &str,
) -> Result<CreateMultipartUploadResponse, anyhow::Error>{

    let payload = CreateMultipartUploadPayload {
        mime: mime.to_string(),
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