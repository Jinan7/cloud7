use icons::Upload;
use leptos::{html, prelude::*, reactive::spawn_local};
use reactive_stores::StoreFieldIterator;
use web_sys::{Event, HtmlInputElement, wasm_bindgen::JsCast};

use crate::{context::{UploadPayload, UploadPayloadStoreFields, UploadsStoreFields, use_upload_context}, s3};

// use crate::s3::{self, get_s3_client};


#[component]
pub fn UploadButton() -> impl IntoView {

    let uploads = use_upload_context();

    let file_input: NodeRef<html::Input> = NodeRef::new();

    let upload_file_handler = move |ev: Event| {

            
        ev.prevent_default();
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();

        let files = input.files().unwrap();
        let file = files.get(0).unwrap();
         
        let file_name = file.name();

        let key = uuid::Uuid::new_v4();

        let new_upload = UploadPayload {
            key,
            file_name: file_name.clone(),
            percentage: 40
        };

        uploads.rows().update(|rows|{
            rows.push(new_upload);
        });

        let upload = uploads.rows()
            .iter_unkeyed().
            find(|row| {
                row.key().get() == key
            });
        
        if upload.is_none() {return}
        
        let file = gloo_file::File::from(file);
    
        
        
        spawn_local(async move {
                
            let file_bytes = gloo_file::futures::read_as_bytes(&file).await;

            if let Ok(file_bytes) = file_bytes {
                let err = s3::tasks::upload(file_name, &file_bytes[..])
                .await
                .err();

                if let Some(e) = err {
                    print!("{e}");
                }
            }

            
            
        
        });
            
                    
    };

   
    view! {
         
        <label for="upload_file" class="upload-button">
            <Upload class="icon-20-white" />
            <p class="inter-normal-white">"Upload file"</p>
        </label>
        <input 
            type="file"
            id="upload_file"
            on:change=upload_file_handler
            node_ref=file_input
        />    
        
        
    }
}