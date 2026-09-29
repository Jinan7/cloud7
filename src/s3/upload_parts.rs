use std::io::{Cursor, Read};

use aws_sdk_s3::{primitives, types::{self, CompletedMultipartUpload, CompletedPart}};

use crate::s3::{complete_multipart_upload, upload_part};

const CHUNK_SIZE: u64 = 1024 * 1024 * 5;
const MAX_CHUNKS: u64 = 10000;

pub async fn upload_parts(
    client: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
    upload_id: &str,
    file: &[u8],
    file_size: u64,
) -> anyhow::Result<()> {
    
    
    let mut chunk_count = (file_size / CHUNK_SIZE) + 1;
    let size_of_last_chunk = file_size % CHUNK_SIZE;

   
    if size_of_last_chunk == 0 {
        chunk_count -= 1;
        
    }

    if file_size == 0 {
        return Err(anyhow::anyhow!("Invalid File"))
    }

    if chunk_count > MAX_CHUNKS {
        return Err(anyhow::anyhow!("File too large"))
    }

    let mut file_reader = Cursor::new(file);
    let mut upload_parts: Vec<types::CompletedPart> = Vec::new();
    let mut buffer: [u8;CHUNK_SIZE as usize] = [0;CHUNK_SIZE as usize];

    for chunk_index in 0..chunk_count {

        
        
        
        let len = file_reader.read(&mut buffer)?;
            
        let stream = primitives::ByteStream::from(bytes::Bytes::copy_from_slice(&buffer[0..len]));
        let part_number = (chunk_index as i32) + 1;
        let upload_part_res = upload_part(client, bucket, key, upload_id, part_number, stream)
            .await?;

        upload_parts.push(
            CompletedPart::builder()
                .e_tag(upload_part_res.e_tag.unwrap_or_default())
                .part_number(part_number)
                .build()
        );
           
        
    }

    let completed_multipart_upload: CompletedMultipartUpload = 
        CompletedMultipartUpload::builder()
        .set_parts(Some(upload_parts))
        .build();

    complete_multipart_upload(client, bucket, key, upload_id, completed_multipart_upload)
        .await?;
    Ok(())
}