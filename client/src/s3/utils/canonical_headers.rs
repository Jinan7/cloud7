use std::collections::{HashMap};

pub fn get_canonical_headers_string(headers: &HashMap<String, String>) -> (String, String) {
    //lowercase all header names
    //trim all header values
    let mut stage1= headers.iter()
        .map(|(key, value)| {
            (key.to_lowercase().to_string(), value.trim().to_string())
        })    
        .collect::<Vec<_>>();
    //sort header names alphabetically
    stage1.sort();
    //join header pair with :
    //join all headers with newline character
     //last header pair should also have newline character
    let stage2 = stage1.iter()
        .map(|(key, value)| {
            format!("{}:{}\n", key, value)
        })
        .reduce(|acc, header| {
            format!("{}{}", acc, header)
        });
    
    let stage3 = format!("{}\n",
        if let Some(s) = stage2 {
            s
        } else {
            "".to_string()
        }
    );

    
    //signedheaders
    //join all header names with ; 
    let signed_headers = stage1.iter()
        .fold("".to_string(), |acc, (key, _value)| {
            if acc == "" {
                key.to_string()
            } else {
                format!("{};{}", acc, key)
            }
        });
    //add new line character
    let signed_headers = format!("{}\n", signed_headers);
    //return (canonical_headers, signed_headers)
    (stage3, signed_headers)
}



