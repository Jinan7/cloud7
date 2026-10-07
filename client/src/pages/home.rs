use leptos::prelude::*;

use crate::{components::SearchSection, pages::components::home::{Files, Recents}};

#[component]
pub fn Home() -> impl IntoView {
    
    view! {
        <div class="home">
            <SearchSection/>
            <Recents/>
            <Files/>
        </div>
    }
}