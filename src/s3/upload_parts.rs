use anyhow::Context;
use aws_sdk_s3::{primitives, types::{self, CompletedMultipartUpload, CompletedPart}};

use crate::s3::{complete_multipart_upload, upload_part};

const CHUNK_SIZE: u64 = 1024 * 1024 * 5;
const MAX_CHUNKS: u64 = 10000;
pub async fn upload_parts(
    client: aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
    upload_id: &str,
    file: &[u8],
) -> anyhow::Result<()> {
    
    let file_size: u64 = u64::default();

    let mut chunk_count = (file_size / CHUNK_SIZE) + 1;
    let mut size_of_last_chunk = file_size % CHUNK_SIZE;

    if size_of_last_chunk == 0 {
        chunk_count -= 1;
        size_of_last_chunk = CHUNK_SIZE;
    }

    if file_size == 0 {
        return Err(anyhow::anyhow!("Invalid File"))
    }

    if chunk_count > MAX_CHUNKS {
        return Err(anyhow::anyhow!("File too large"))
    }

    let mut upload_parts: Vec<types::CompletedPart> = Vec::new();

    for chunk_index in 0..chunk_count {

        let chunk_size = if chunk_count - 1 == chunk_index {
            size_of_last_chunk
        } else {
            CHUNK_SIZE
        };

        let stream = primitives::ByteStream::read_from()
            .file(todo!())
            .offset(chunk_index * CHUNK_SIZE)
            .length(primitives::Length::Exact(chunk_size))
            .build()
            .await
            .context("Upload failed")?;

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