use std::error::Error;

use actix_web::HttpResponse;
use serde::Deserialize;
use serde_nested_json;
use crate::utils::e500;

#[derive(Debug, Deserialize)]
pub struct Notification {
    #[serde(rename = "Type")]
    message_type: String,
    #[serde(rename = "MessageId")]
    message_id: String,
    #[serde(rename = "TopicArn")]
    topic_arn: String,
    #[serde(rename = "Subject")]
    subject: String,
    #[serde(rename = "Message")]
    #[serde(with = "serde_nested_json")]
    message: Message,
    #[serde(rename = "Timestamp")]
    timestamp: String,
    #[serde(rename = "SignatureVersion")]
    signature_version: String,
    #[serde(rename = "Signature")]
    signature: String,
    #[serde(rename = "SigningCertURL")]
    signing_cert_url: String,
    #[serde(rename = "UnsubscribeURL")]
    unsubscribe_url: String,
    
}

#[derive(Debug, Deserialize)]
pub struct Message {
    #[serde(rename = "Records")]
    records: Vec<Record>,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "eventVersion")]
    event_version: String,
    #[serde(rename = "eventSource")]
    event_source: String,
    #[serde(rename = "awsRegion")]
    aws_region: String,
    #[serde(rename = "eventTime")]
    event_time: String,
    #[serde(rename = "eventName")]
    event_name: String,
    #[serde(rename = "userIdentity")]
    user_identity: UserIdentity,
    #[serde(rename = "requestParameters")]
    request_parameters:  RequestParameters,
     #[serde(rename = "responseElements")]
    response_elements: ResponseElements,
    s3: S3,
}

#[derive(Debug, Deserialize)]
pub struct UserIdentity {
    #[serde(rename = "principalId")]
    principal_id: String
}

#[derive(Debug, Deserialize)]
pub struct RequestParameters {
    #[serde(rename = "sourceIPAddress")]
    source_ip_address: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseElements {
    #[serde(rename = "x-amz-request-id")]
    x_amz_request_id: String,
    #[serde(rename = "x-amz-id-2")]
    x_amz_id_2: String
}

#[derive(Debug, Deserialize)]
pub struct S3 {
    #[serde(rename = "s3SchemaVersion")]
    s3_schema_version: String,
    #[serde(rename = "configurationId")]
    configuration_id: String,
    bucket: Bucket,
    object: Object,
}

#[derive(Debug, Deserialize)]
pub struct Bucket {
    name: String,
    #[serde(rename = "ownerIdentity")]
    owner_identity: UserIdentity,
    arn: String,
}

#[derive(Debug, Deserialize)]
pub struct Object {
    key: String,
    size: u64,
    #[serde(rename = "eTag")]
    e_tag: String,
    sequencer: String,
}
pub async fn post(
    payload: String,
) -> Result<HttpResponse, actix_web::Error> {

    let notification: Notification = serde_json::from_str(&payload)
        .map_err(e500)?;


   Ok(HttpResponse::Ok().finish())   
}

fn err_chain(e: &impl Error ) {

    println!("{e:?}");

    let mut error = e.source();

    while error.is_some() {
        println!("{e:?}");
        error = e.source()
    };
}

