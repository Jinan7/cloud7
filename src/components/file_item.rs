use leptos::prelude::*;

#[component]
pub fn FileItem() -> impl IntoView {

    view! {
        <div class="file-item">
            <div class="icon-24-blue">
            </div>
            <p class="inter-13-grey-4">"Crash file"</p>
        </div>
    }
}