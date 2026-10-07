use std::collections::HashMap;

use crate::s3::utils::{get_canonical_headers_string,get_canonical_query_string,get_canonical_url};

pub fn get_canonical_request(
    http_verb:&str,
    canonical_url: &str,
    canonical_query_params: &str,
    canonical_headers: &str,
    signed_headers: &str,
    payload_hash: &str,
) -> String {


    let canonical_request = format!(
        "{}{}{}{}{}{}", 
        http_verb, 
        canonical_url, 
        canonical_query_params, 
        canonical_headers,
        signed_headers,
        payload_hash
    );

    canonical_request
}

pub fn get_canonical_request_v2(
    http_verb: &str,
    url: &str,
    query_params: Vec<(String, String)>,
    headers: &HashMap<String, String>,
    payload_hash: &str,
) -> (String, String) {

    let canonical_url = get_canonical_url(url);
    let canonical_query_params = get_canonical_query_string(query_params);
    let (canonical_headers, signed_headers) = get_canonical_headers_string(headers);

    let canonical_request = get_canonical_request(http_verb, &canonical_url, &canonical_query_params, &canonical_headers, &signed_headers, payload_hash);
    (canonical_request, signed_headers)
}