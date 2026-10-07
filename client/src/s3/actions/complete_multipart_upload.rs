use std::collections::HashMap;

use chrono::{DateTime, Utc};
use reqwest::Response;

use crate::s3::utils::{get_auth_header, get_canonical_request_v2, get_host_and_uri, get_payload_hash, get_scope, headers::{AUTHORIZATION, HOST, X_AMZ_CONTENT_SHA256, X_AMZ_DATE}, post, signature_v2, to_iso8601};

pub async fn complete_multipart_upload(
    bucket: &str,
    key: &str,
    upload_id: &str,
    parts: String,
    date: DateTime<Utc>,
    access: &str,
    secret: &str,
) -> Result<Response, anyhow::Error> {

    let (host, uri) = get_host_and_uri(bucket, key);
    let address = format!("https://{}/{}?uploadId={}", host, key, upload_id);
    let payload_hash = get_payload_hash(parts.as_bytes());
    
    let mut headers = HashMap::new();
    headers.insert(HOST.to_string(), host);
    headers.insert(X_AMZ_CONTENT_SHA256.to_string(), payload_hash.clone());
    headers.insert(X_AMZ_DATE.to_string(), to_iso8601(date));
    let query_params = vec![
        ("uploadId".to_string(), upload_id.to_string())
    ];
    let http_verb = post();

    let (canonical_request, signed_headers) = get_canonical_request_v2(
        &http_verb, 
        &uri, 
        query_params, 
        &headers, 
        &payload_hash
    );

    println!("{canonical_request}");

    let signature = signature_v2(date, "eu-north-1", "s3", &canonical_request, secret);

    let scope = get_scope(date, "eu-north-1", "s3");
    let auth_header = get_auth_header(   
        access,
        &scope,
        &signed_headers,
        &signature
    );

    let client = reqwest::Client::new();
    let response = client.post(address)
        .header(AUTHORIZATION, auth_header)
        .headers((&headers).try_into()?)
        .body(
            parts
        )
        .send()
        .await?
        .error_for_status()?;
        
    Ok(response)   
}