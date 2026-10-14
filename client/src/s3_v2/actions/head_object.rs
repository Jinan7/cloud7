use leptos::server_fn::request;

#[derive(serde::Deserialize)]
pub struct HeadObjectResponse {
    pub content_length: i64
}
pub async fn head_object(
    key: &str,
) -> anyhow::Result<HeadObjectResponse> {

    let response = reqwest::Client::new()
        .get(format!("http://localhost:8000/files/head_object/{}", key))
        .send()
        .await?;

    let response: HeadObjectResponse = serde_json::from_str(&response.text().await?)?;
    Ok(response)
    
}