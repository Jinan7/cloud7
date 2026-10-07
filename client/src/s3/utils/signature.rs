use chrono::{DateTime, Utc};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

use crate::s3::utils::{get_signing_key, string_to_sign};

pub fn signature(
    signing_key: &[u8],
    string_to_sign: &str,
) -> String {
    
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(signing_key).expect("some typa error");
    mac.update(string_to_sign.as_bytes());

    let hmac = mac.finalize();
    hex::encode(hmac.into_bytes())
}

pub fn signature_v2(
    date: DateTime<Utc>,
    region: &str,
    service: &str,
    canonical_request: &str,
    secret: &str,
) -> String {
    
    let string_to_sign = string_to_sign(
        date,
        canonical_request, 
        region, 
        service
    );


    
    let signing_key = get_signing_key(
        date,
        secret,
        region, 
        service
    );

    signature(&signing_key, &string_to_sign)
}