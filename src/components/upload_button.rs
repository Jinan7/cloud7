use gloo_file::FileReadError;
use icons::Upload;
use leptos::{html, prelude::*, reactive::spawn_local};
use tokio::io::AsyncReadExt;
use web_sys::{Event, HtmlInputElement, wasm_bindgen::JsCast};

use crate::s3;


#[component]
pub fn UploadButton() -> impl IntoView {

    let (file_path, set_file_path) = signal("Upload file".to_string());
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
                let err = s3::tasks::multipart_upload(todo!(), todo!(),&file_name, file_bytes.as_ref(), file.size())
                .await
                .err();

                if let Some(e) = err {
                    todo!()
                }
            }
            
                
                
        });
            
                    
    };

   
    view! {
         
        <label for="upload_file" class="upload-button">
            <Upload class="icon-20-white" />
            <p class="inter-normal-white">{move || file_path.get()}</p>
        </label>
        <input 
            type="file"
            id="upload_file"
            on:change=upload_file_handler
            node_ref=file_input
        />    
        
        
    }
}