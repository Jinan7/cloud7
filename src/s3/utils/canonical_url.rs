pub fn get_canonical_url(
    url: &str
) -> String {
    let paths = url.split('/').collect::<Vec<_>>();

    let stage1 = paths.iter()
        .map(|path| {
            urlencoding::encode(path).into_owned()
        })
        .reduce(|acc, url| {
            format!("{}/{}", acc, url)
        });

    let canonical_url = format!("{}\n", 
        
        if let Some(s) = stage1 {
            s
        } else {
            "".to_string()
        }
    );

    canonical_url
}