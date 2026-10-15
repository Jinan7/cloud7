use crate::s3_v2::actions::{CreateMultipartUploadResponse, GetObjectPresignedResponse, HeadObjectResponse, complete_multipart_upload, create_multipart_upload_v2, get_object, get_object_presigned, head_object, upload_parts};
pub async fn upload(
    key: String,
    mime_type: String, 
    file: &[u8],
    size: u64,
) -> Result<(), anyhow::Error> {

   

    let CreateMultipartUploadResponse { upload_id, file_id } = create_multipart_upload_v2(
        &key,
        &mime_type,
        size,
    )
    .await?;

    leptos::logging::log!("{}", upload_id);

    let parts = upload_parts(
        &key,
        &upload_id, 
        file,    
    ).await?;

    leptos::logging::log!("{:?}", &parts);
    
    complete_multipart_upload(
        &key, 
        &upload_id, 
        parts, 
    )
    .await?;

    Ok(())
}

pub async fn download(
    key: String
) -> anyhow::Result<()> {

    let GetObjectPresignedResponse { presigned_url }  = get_object_presigned(&key).await?;
    let HeadObjectResponse { content_length } = head_object(&key).await?;
    get_object(&key, &presigned_url, content_length).await?;
    Ok(())
}
