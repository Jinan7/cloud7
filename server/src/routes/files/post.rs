use std::error::Error;

use actix_web::HttpResponse;
use serde::Deserialize;
use serde_nested_json;
use crate::utils::e500;

#[derive(Debug, Deserialize)]
pub struct Notification {
    #[serde(rename = "Type")]
    _message_type: String,
    #[serde(rename = "MessageId")]
    _message_id: String,
    #[serde(rename = "TopicArn")]
    _topic_arn: String,
    #[serde(rename = "Subject")]
    _subject: String,
    #[serde(rename = "Message")]
    #[serde(with = "serde_nested_json")]
    message: Message,
    #[serde(rename = "Timestamp")]
    _timestamp: String,
    #[serde(rename = "SignatureVersion")]
    _signature_version: String,
    #[serde(rename = "Signature")]
    _signature: String,
    #[serde(rename = "SigningCertURL")]
    _signing_cert_url: String,
    #[serde(rename = "UnsubscribeURL")]
    _unsubscribe_url: String,
    
}

#[derive(Debug, Deserialize)]
pub struct Message {
    #[serde(rename = "Records")]
    records: Vec<Record>,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "eventVersion")]
    _event_version: String,
    #[serde(rename = "eventSource")]
    _event_source: String,
    #[serde(rename = "awsRegion")]
    _aws_region: String,
    #[serde(rename = "eventTime")]
    _event_time: String,
    #[serde(rename = "eventName")]
    _event_name: String,
    #[serde(rename = "userIdentity")]
    _user_identity: UserIdentity,
    #[serde(rename = "requestParameters")]
    _request_parameters:  RequestParameters,
     #[serde(rename = "responseElements")]
    _response_elements: ResponseElements,
    s3: S3,
}

#[derive(Debug, Deserialize)]
pub struct UserIdentity {
    #[serde(rename = "principalId")]
    _principal_id: String
}

#[derive(Debug, Deserialize)]
pub struct RequestParameters {
    #[serde(rename = "sourceIPAddress")]
    _source_ip_address: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseElements {
    #[serde(rename = "x-amz-request-id")]
    _x_amz_request_id: String,
    #[serde(rename = "x-amz-id-2")]
    _x_amz_id_2: String
}

#[derive(Debug, Deserialize)]
pub struct S3 {
    #[serde(rename = "s3SchemaVersion")]
    _s3_schema_version: String,
    #[serde(rename = "configurationId")]
    _configuration_id: String,
    _bucket: Bucket,
    object: Object,
}

#[derive(Debug, Deserialize)]
pub struct Bucket {
    _name: String,
    #[serde(rename = "ownerIdentity")]
    _owner_identity: UserIdentity,
    _arn: String,
}

#[derive(Debug, Deserialize)]
pub struct Object {
    key: String,
    size: u64,
    #[serde(rename = "eTag")]
    e_tag: String,
    _sequencer: String,
}
pub async fn post(
    payload: String,
) -> Result<HttpResponse, actix_web::Error> {

    let notification: Notification = serde_json::from_str(&payload)
        .map_err(e500)?;

    let records = notification.message.records;

    for record in records {
        let file_name = record.s3.object.key;
        let file_size = record.s3.object.size;
        let e_tag = record.s3.object.e_tag;

    }
    Ok(HttpResponse::Ok().finish())   
}


