use icons::Upload;
use leptos::prelude::*;

#[component]
pub fn UploadButton() -> impl IntoView {
    view! {
        <div class="upload-button">
            <Upload class="icon-20-white" />
            <p class="inter-normal-white">"Upload file"</p>
        </div>
    }
}