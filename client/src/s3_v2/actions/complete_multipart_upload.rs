
#[derive(Debug, serde::Serialize)]
pub struct Part {
    pub e_tag: String,
    pub part_number: i32,
}
#[derive(Debug, serde::Serialize)]
pub struct CompleteMultipartUploadPayload {
    pub key: String,
    pub upload_id: String,
    pub parts: Vec<Part>
}

pub async fn complete_multipart_upload(
    key: &str,
    upload_id: &str,
    parts: Vec<Part>,
) -> Result<(), anyhow::Error> {

    let payload = CompleteMultipartUploadPayload {
        key: key.to_string(),
        upload_id: upload_id.to_string(),
        parts,
    };

    let payload = serde_json::to_string(&payload)?;

    dbg!(payload);
    Ok(())
}


#[cfg(test)]
mod test {
    
    use crate::s3_v2::actions::complete_multipart_upload::{Part, complete_multipart_upload};


    #[tokio::test] 
    async fn complete_multipart_upload_test() {

        let part1 = Part {
            e_tag: uuid::Uuid::new_v4().to_string(),
            part_number: 1
        };

        let part2 = Part {
            e_tag: uuid::Uuid::new_v4().to_string(),
            part_number: 2
        };

        let part3 = Part {
            e_tag: uuid::Uuid::new_v4().to_string(),
            part_number: 3
        };

        let parts = vec![part1, part2, part3];

        let key = "my_example_file.txt";
        let upload_id= uuid::Uuid::new_v4().to_string();

        complete_multipart_upload(
            key,
            &upload_id,
            parts
        )
        .await
        .expect("something went wrong");

    }

}