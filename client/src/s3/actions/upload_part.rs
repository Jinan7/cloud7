use std::{collections::HashMap, fmt::Display};
use chrono::{DateTime, Utc};
use crate::s3::utils::{get_auth_header, get_canonical_request_v2, get_host_and_uri, get_payload_hash, get_scope, headers::{AUTHORIZATION, HOST, X_AMZ_CONTENT_SHA256, X_AMZ_DATE}, put, signature_v2, to_iso8601};
#[derive(Debug)]
pub struct UploadPartResponse {
    pub e_tag: String,
}

impl Display for UploadPartResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.e_tag)
    }
}

pub async fn upload_part(
    bucket: &str,
    key: &str,
    part_number: u64,
    upload_id: &str,
    chunk: &[u8],
    date: DateTime<Utc>,
    access: &str,
    secret: &str,
) -> Result<UploadPartResponse, anyhow::Error> {

    let (host, uri) = get_host_and_uri(bucket, key);
    let address = format!("https://{}/{}?partNumber={}&uploadId={}", &host, key, part_number, upload_id);

    let payload_hash = get_payload_hash(chunk);
    
    let mut headers = HashMap::new();
    headers.insert(HOST.to_string(), host);
    headers.insert(X_AMZ_CONTENT_SHA256.to_string(), payload_hash.clone());
    headers.insert(X_AMZ_DATE.to_string(), to_iso8601(date));

    let query_params = vec![
        ("partNumber".to_string(), part_number.to_string()),
        ("uploadId".to_string(), upload_id.to_string()),
    ];
    let http_verb = put();

    let (canonical_request, signed_headers) = get_canonical_request_v2(
        &http_verb, 
        &uri, 
        query_params, 
        &headers, 
        &payload_hash
    );


    let signature = signature_v2(date, "eu-north-1", "s3", &canonical_request, secret);

    let scope = get_scope(date, "eu-north-1", "s3");
    let auth_header = get_auth_header(   
        access,
        &scope,
        &signed_headers,
        &signature
    );

    let client = reqwest::Client::new();
    let response = client.put(address)
        .header(AUTHORIZATION, auth_header)
        .headers((&headers).try_into()?)
        .body(
            chunk.to_vec()
        )
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