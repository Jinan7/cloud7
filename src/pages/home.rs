use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    
    view! {
        <div class="home">
            <div class="search">
                <input class="search-bar inter-20-grey" 
                placeholder="Search"
                >
                    <p class="inter-20">"Search"</p>
                </input>
            </div>
            <div class="recents"></div>
            <div class="files"></div>
        </div>
    }
}