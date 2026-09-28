use icons::Upload;
use leptos::{prelude::*, reactive::spawn_local};
use rfd::AsyncFileDialog;

#[component]
pub fn UploadButton() -> impl IntoView {

   
    view! {
         
        <label for="upload_file" class="upload-button">
            <Upload class="icon-20-white" />
            <p class="inter-normal-white">"Upload file"</p>
        </label>
        <input 
            type="file"
            id="upload_file"
        />    
        
        
    }
}