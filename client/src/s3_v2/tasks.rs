use crate::s3_v2::actions::{CreateMultipartUploadResponse, complete_multipart_upload, create_multipart_upload_v2, upload_parts};
pub async fn upload(
    key: String,
    _mime_type: String, 
    file: &[u8],
) -> Result<(), anyhow::Error> {

   

    let CreateMultipartUploadResponse { upload_id } = create_multipart_upload_v2(
        &key,
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
