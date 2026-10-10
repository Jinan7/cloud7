use crate::s3_v2::actions::{Part, upload_part};
use std::io::Read;

const CHUNK_SIZE: u64 = 5*1024*1024;

pub async fn upload_parts(
    presigned_url: &str,
    mut file: &[u8]
) -> Result<Vec<Part>, anyhow::Error> {

    let mut buffer = vec![0u8; CHUNK_SIZE as usize];
    let mut part_number = 1;

    let mut parts: Vec<Part> = Vec::new();
    
    while let Ok(len) = file.read(&mut buffer[..]) {

        if len == 0 {
            break;
        }


        let _response = upload_part(  
            presigned_url,
            &buffer,
        ).await;

        // if let Err(e) = response {
        //     leptos::logging::log!("{e}")
        // } else {
        //     parts.push(
        //         Part {
        //             e_tag: response.unwrap().e_tag,
        //             part_number
        //         }
        //     );
        // }

        parts.push(
            Part {
                e_tag: uuid::Uuid::new_v4().to_string(),
                part_number,
            }
        );

        part_number += 1;
    }
    
    Ok(parts)

}