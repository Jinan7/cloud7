use leptos::prelude::*;
use crate::{components::Upload, context::{UploadPayloadStoreFields, Uploads, UploadsStoreFields, use_upload_context}};


#[component]
pub fn UploadSection() -> impl IntoView {

    let uploads = use_upload_context();
    uploads.set(
        Uploads {
            rows: Vec::new()
        }
    );

    view! {
        <div class="uploads">

            <For
                each = move || uploads.rows()
                key = move |row| row.read().key.clone()
                let(child)
            >
                <Upload 
                    file_name=child.file_name().get()
                    percentage=child.percentage().get()
                />
            </For>

        </div>
    }
}