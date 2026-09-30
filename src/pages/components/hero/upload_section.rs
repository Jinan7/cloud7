use leptos::prelude::*;
use crate::components::Upload;

#[component]
pub fn UploadSection() -> impl IntoView {

    view! {
        <div class="uploads">
            <Upload />
            <Upload />
            <Upload />
            <Upload />
            <Upload />
            <Upload />
        </div>
    }
}