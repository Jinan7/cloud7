pub fn get_host_and_uri(
    bucket: &str,
    key: &str,
) -> (String, String) {
    let host = format!("{}.s3.amazonaws.com", &bucket);
    let _address = format!("https://{}/{}?uploads", &host, &key);
    let uri = format!("/{}", &key);

    (host, uri)
}