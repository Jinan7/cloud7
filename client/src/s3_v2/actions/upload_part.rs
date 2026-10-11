#[derive(Debug)]
pub struct UploadPartResponse {
    pub e_tag: String,
}
pub async fn upload_part(
    presigned_url: &str,
    part: &[u8]
) -> Result<UploadPartResponse, anyhow::Error> {


    let response = reqwest::Client::new()
        .put(presigned_url)
        .body(part.to_vec())
        .send()
        .await?
        .error_for_status()?;

    let e_tag = response
        .headers()
        .get("etag")
        .ok_or(anyhow::anyhow!("etag not found"))?
        .to_str()?
        .to_owned();
    Ok(UploadPartResponse { e_tag })
}