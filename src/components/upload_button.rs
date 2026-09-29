use icons::Upload;
use leptos::{html, prelude::*, reactive::spawn_local};
use web_sys::{Event, HtmlInputElement, wasm_bindgen::JsCast};

use crate::s3::{self, get_s3_client};


#[component]
pub fn UploadButton() -> impl IntoView {

    let file_input: NodeRef<html::Input> = NodeRef::new();

    let upload_file_handler = move |ev: Event| {

            
        ev.prevent_default();
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();

        let files = input.files().unwrap();
        let file = files.get(0).unwrap();
        let file_name = file.name();
        let file = gloo_file::File::from(file);
    
        
        
        spawn_local(async move {
            let file_bytes_result = gloo_file::futures::read_as_bytes(&file)
                .await
                .ok();

            
            if let Some(file_bytes) = file_bytes_result {
                let client = get_s3_client().await;
                let err = s3::tasks::multipart_upload(&client, "my-bucket",&file_name, &file_bytes, file.size())
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