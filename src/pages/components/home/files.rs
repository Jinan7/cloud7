use leptos::prelude::*;
use crate::{components::FileItem, pages::components::home::FileFilterSection};
#[component]
pub fn Files() -> impl IntoView {
    
    view! {
        <div class="files">
                <div class="files-header">
                    <p class="">"All files"</p>
                </div>
                <FileFilterSection/>
                <div class="files-content">
                    <FileItem/>
                    <FileItem/>
                    <FileItem/>
                    <FileItem/>
                    <FileItem/>
                    <FileItem/>
                    <FileItem/>
                    <FileItem/>
                    <FileItem/>
                    <FileItem/>
                    <FileItem/>
                    <FileItem/>
                    <FileItem/>
                    <FileItem/>
                    <FileItem/>
                    <FileItem/>
                    <FileItem/>
                </div>
            </div>   
    }
}