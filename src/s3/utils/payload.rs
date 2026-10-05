use sha2::Digest;

pub fn get_payload_hash(
    chunk: &[u8]
) -> String {

    let hash = sha2::Sha256::digest(chunk);

    let hex = hex::encode(hash);

    hex
}

pub fn get_empty_string_hash() -> String {
    //used for when no request payload (body) is present
    //hash empty string ""
    let hash = sha2::Sha256::digest("");
    //hex encode
    let hex = hex::encode(hash);
    //return
    hex
}