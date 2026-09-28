use gloo_file::callbacks::read_as_bytes;
use icons::Upload;
use leptos::{html, prelude::*};
use web_sys::{Event, HtmlInputElement, wasm_bindgen::JsCast};


#[component]
pub fn UploadButton() -> impl IntoView {

    let (file_path, set_file_path) = signal("Upload file".to_string());
    let file_input: NodeRef<html::Input> = NodeRef::new();

    let callback = move |_| { *set_file_path.write() = "Success".to_string()};
    let oninput = move |ev: Event| {

            
        ev.prevent_default();
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();

        let files = input.files().unwrap();
        let file = files.get(0).unwrap();
        let file = gloo_file::File::from(file);
        let bytes = gloo_file::callbacks::read_as_bytes(&file, callback);
        
        *set_file_path.write() = file.name();
            
                    
    };

   
    view! {
         
        <label for="upload_file" class="upload-button">
            <Upload class="icon-20-white" />
            <p class="inter-normal-white">{move || file_path.get()}</p>
        </label>
        <input 
            type="file"
            id="upload_file"
            on:change=oninput
            node_ref=file_input
        />    
        
        
    }
}