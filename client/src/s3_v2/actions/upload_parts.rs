use crate::s3_v2::actions::{Part, UploadPartPresignedResponse, UploadPartResponse, upload_part, upload_part_presigned};
use std::io::Read;

const CHUNK_SIZE: u64 = 5*1024*1024;

pub async fn upload_parts(
    key: &str,
    upload_id: &str,
    mut file: &[u8]
) -> Result<Vec<Part>, anyhow::Error> {

    let mut buffer = vec![0u8; CHUNK_SIZE as usize];
    let mut part_number = 1;

    let mut parts: Vec<Part> = Vec::new();
    
    while let Ok(len) = file.read(&mut buffer[..]) {

        if len == 0 {
            break;
        }

        let UploadPartPresignedResponse { presigned_url } = upload_part_presigned(&key, upload_id, part_number).await?;
        leptos::logging::log!("Part {}", part_number);
        leptos::logging::log!("     presigned_url: {}", &presigned_url);

        let UploadPartResponse { e_tag } = upload_part(  
            &presigned_url,
            &buffer,
        ).await?;

        
        parts.push(
            Part {
                e_tag,
                part_number
            }
        );
        part_number += 1;
    }
    
    Ok(parts)

}