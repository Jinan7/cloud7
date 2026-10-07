use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::s3::utils::{get_auth_header, get_canonical_request_v2, get_empty_string_hash, get_host_and_uri, get_scope, headers::{AUTHORIZATION, HOST, X_AMZ_CONTENT_SHA256, X_AMZ_DATE}, post, signature_v2, to_iso8601};

#[derive(Deserialize)]
pub struct CreateMultipartResponse {
    #[serde(rename = "UploadId")]
    pub upload_id: String,
}

pub async fn create_multipart_upload(
    bucket: &str,
    key: &str,
    date: DateTime<Utc>,
    access: &str,
    secret: &str,
) -> Result<CreateMultipartResponse, anyhow::Error> {

    
    let (host, uri) = get_host_and_uri(bucket, key);
    let address = format!("https://{}/{}?uploads", &host, &key);
    let payload_hash = get_empty_string_hash();
    let mut headers = HashMap::new();
    headers.insert(HOST.to_string(), host);
    headers.insert(X_AMZ_CONTENT_SHA256.to_string(), payload_hash.clone());
    headers.insert(X_AMZ_DATE.to_string(), to_iso8601(date));

   
    let query_params = vec![
        ("uploads".to_string(), "".to_string())
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
        &access,
        &scope,
        &signed_headers,
        &signature
    );

    let client = reqwest::Client::new();
    let response = client.post(address)
        .header(AUTHORIZATION, auth_header)
        .headers((&headers).try_into()?)
        .send()
        .await?
        .error_for_status()?;
        
    if response.status() == 200 {
        let response_xml = String::from_utf8(response.bytes().await?.to_vec())?;
        dbg!("{}", &response_xml);
        let response_body = quick_xml::de::from_str::<CreateMultipartResponse>(&response_xml)?;
        Ok(response_body)
    } else {
        Err(anyhow::anyhow!("Invalid response"))
    }
    
    
}