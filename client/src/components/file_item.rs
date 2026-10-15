use leptos::{prelude::*, reactive::spawn_local};
use web_sys::MouseEvent;

use crate::s3_v2;

#[component]
pub fn FileItem(
    label: String
) -> impl IntoView {

    let key = label.clone();
    let download_file = move |e: MouseEvent| {
        e.prevent_default();
        let key_clone = key.clone();
        spawn_local(async {

            let result = s3_v2::tasks::download(key_clone).await;

            if let Err(e) = result {
                leptos::logging::log!("{}", e)
            }
        });
    };

    
    view! {
        <div class="file-item"
            on:click= download_file
        >
            <div class="icon-24-blue">
            </div>
            <p class="inter-13-grey-4">{label}</p>
        </div>
    }
}