use leptos::prelude::*;

#[component]
pub fn UploadButton() -> impl IntoView {
    view! {
        <div class="upload-button">
            <img class="icon-20" src="./public/upload.png"/>
            <p class="inter-normal-white">"Upload file"</p>
        </div>
    }
}