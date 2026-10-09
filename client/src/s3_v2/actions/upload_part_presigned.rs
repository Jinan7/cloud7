use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UploadPartPresignedResponse {
    pub presigned_url: String
}

pub async fn upload_part_presigned(
    key: &str,
    upload_id: &str,
    part_number: i32,
) -> Result<UploadPartPresignedResponse, anyhow::Error> {

    let request_body = serde_json::json!({
        "key": key,
        "upload_id": upload_id,
        "part_number": part_number,
    });
    
    
    let response = reqwest::Client::new()
        .post(format!("http://localhost:8000/files/upload_part_presigned"))
        .json(&request_body)
        .send()
        .await?;
    
    let response = String::from_utf8(response.bytes().await?.to_vec())?;
    
    let response: UploadPartPresignedResponse = serde_json::from_str(&response)?;

    Ok(response)
}