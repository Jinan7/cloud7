use leptos::prelude::*;

#[component]
pub fn SearchBar() -> impl IntoView {

    view! {
        <input 
            class="search-bar inter-20-grey" 
            placeholder="Search"
        />
       
    }
}