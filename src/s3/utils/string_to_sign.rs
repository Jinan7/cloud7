use chrono::{DateTime, Utc};
use sha2::Digest;

use crate::s3::utils::{get_scope, to_iso8601};

pub fn string_to_sign(
    date: DateTime<Utc>,
    canonical_request: &str,
    region: &str,
    service: &str,
) -> String {

    
    let scope = get_scope(date, region, service);
    let canonical_request = hex::encode(sha2::Sha256::digest(canonical_request));
    let timestamp = to_iso8601(date);
    format!("AWS4-HMAC-SHA256\n{}\n{}\n{}", timestamp, scope, canonical_request)
}


