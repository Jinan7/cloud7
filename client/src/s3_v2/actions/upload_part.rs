
pub async fn upload_part(
    presigned_url: &str,
    part: &[u8]
) -> Result<(), anyhow::Error> {


    let _response = reqwest::Client::new()
        .put(presigned_url)
        .body(part.to_vec())
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}