use chrono::{DateTime, Utc};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

use crate::s3::utils::to_yyymmdd;

pub fn get_signing_key(
    date: DateTime<Utc>,
    secret: &str,
    region: &str,
    service: &str,
) -> Vec<u8> {

    let date_key =  get_hmac(format!("AWS4{}", secret).as_bytes(), &to_yyymmdd(date));
    let date_region_key = get_hmac(&date_key, region);
    let date_region_service_key = get_hmac(&date_region_key, service);
    let signing_key = get_hmac(&date_region_service_key, "aws4_request");
    
    signing_key
}

fn get_hmac(
    key: &[u8],
    message: &str,
) -> Vec<u8> {

    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(
        key
        )
        .unwrap();
    mac.update(message.as_bytes());
    let hmac = mac.finalize().into_bytes().to_vec();
    hmac
}