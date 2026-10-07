use leptos::prelude::*;

use crate::components::SearchBar;

#[component]
pub fn SearchSection() -> impl IntoView {

    view! {
        <div class="search">
            <SearchBar/>  
        </div>  
    }
}