use std::io::Read;

use chrono::{Utc};

use crate::s3::actions::upload_part::upload_part;

const CHUNK_SIZE: u64 = 5*1024*1024;
#[derive(Debug)]
pub struct Part {
    pub e_tag: String,
    pub part_number: u64,
}
pub async fn upload_parts(
    bucket: &str,
    key: &str,
    upload_id: &str,
    mut file: &[u8],
    access: &str,
    secret: &str,
) -> Result<Vec<Part>, anyhow::Error> {

    
    let mut buffer = vec![0u8; CHUNK_SIZE as usize];
    let mut part_number = 1;

    let mut parts: Vec<Part> = Vec::new();
    
    while let Ok(len) = file.read(&mut buffer[..]) {

        if len == 0 {
            break;
        }


        let response = upload_part(
            bucket, 
            key,
            part_number,
            upload_id, 
            &buffer[0..len],
            Utc::now(),
             access, 
             secret
        ).await;

        if let Err(e) = response {
            leptos::logging::log!("{e}")
        } else {
            parts.push(
                Part {
                    e_tag: response.unwrap().e_tag,
                    part_number
                }
            );
        }

        part_number += 1;
    }
    
    Ok(parts)
}