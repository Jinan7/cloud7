pub fn get_canonical_query_string(query: Vec<(String, String)>) -> String {

    //url encode key and value
    let mut stage1 = query.iter()
        .map(|(key, value)| {

            (urlencoding::encode(key).into_owned(), urlencoding::encode(value).into_owned())
        })
        .collect::<Vec<_>>();
    //sort base on key
    stage1.sort();
    //join key and value with =
    //join all queries with &
    let stage2 = stage1.iter()
        .map(|(key, value)| {
            format!("{}={}", key, value)
        })
        .reduce(|acc, query_pair| {
            format!("{}&{}", acc, query_pair)
        });

    //append newline character
    let canonical_query_string = format!("{}\n", 
        if let Some(s) = stage2 {
            s
        } else {
            "".to_string()
        }
    );
    //return

    //if key does not have a value, the replace value with empty string
    //if no query string present, return newline character
    canonical_query_string
}

