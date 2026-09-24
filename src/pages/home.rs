use leptos::prelude::*;

use crate::{components::SearchSection, pages::components::home::Recents};

#[component]
pub fn Home() -> impl IntoView {
    
    view! {
        <div class="home">
            <SearchSection/>
            <Recents/>
            <div class="files"></div>
        </div>
    }
}