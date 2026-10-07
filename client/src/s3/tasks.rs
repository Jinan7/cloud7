use chrono::Utc;

use crate::s3::actions::{CreateMultipartResponse, Part, complete_multipart_upload, create_multipart_upload, upload_parts};
pub async fn upload(
    key: String,
    file: &[u8]
) -> Result<(), anyhow::Error> {

    dotenvy::dotenv().ok();
    
    let access_key = "";
    let secret = "";
    let bucket = "";

    let CreateMultipartResponse { upload_id } = create_multipart_upload(
        &bucket, 
        &key, 
        Utc::now(), 
        &access_key, 
        &secret
    )
    .await?;

    let response = upload_parts(
        &bucket, 
        &key, 
        &upload_id, 
        file, 
        &access_key, 
        &secret
    ).await?;

    let parts = get_parts_xml_string(response);
    leptos::logging::log!("parts : {}", &parts);
    complete_multipart_upload(
        &bucket, 
        &key, 
        &upload_id, 
        parts, 
        Utc::now(), 
        &access_key, 
        &secret
    )
    .await?
    .error_for_status()?;

    Ok(())
}


fn get_part_xml_string(
    part: &Part
) -> String {

    format!("<Part><ETag>{}</ETag><PartNumber>{}</PartNumber></Part>", part.e_tag, part.part_number)
}

fn get_parts_xml_string(
    parts: Vec<Part>,
) -> String {

    let parts: Vec<String> = parts.iter()
        .map(|part | {
            get_part_xml_string(part)
        })
        .collect();

    let parts_xml_string = parts.join("");
    format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<CompleteMultipartUpload xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">{}</CompleteMultipartUpload>", parts_xml_string)
}