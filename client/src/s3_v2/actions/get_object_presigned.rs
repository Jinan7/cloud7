#[derive(serde::Deserialize, Debug)]
pub struct GetObjectPresignedResponse {
    pub presigned_url: String
}
pub async fn get_object_presigned(
    key: &str
) -> Result<GetObjectPresignedResponse, anyhow::Error> {

    let response = reqwest::Client::new()
        .get(format!("http://localhost:8000/files/{}", key))
        .send()
        .await?;

    let presigned_url: GetObjectPresignedResponse = serde_json::from_str(response.text().await?.as_str())?;

    Ok(presigned_url)

}