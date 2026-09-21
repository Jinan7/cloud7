use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    
    view! {
        <div class="home">
            <div class="search"></div>
            <div class="recents"></div>
            <div class="files"></div>
        </div>
    }
}