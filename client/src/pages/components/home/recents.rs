use leptos::prelude::*;

use crate::pages::components::home::RecentsItem;


#[component]
pub fn Recents() -> impl IntoView {
    
    view! {
        <div class="recents">
            <div class="recents-header">
                <p class="inter-normal-black">"Recently opened"</p>
                <p class="inter-normal-grey-2">"Clear"</p>
            </div>  
            <div class="recents-content">
                <div class="recents-content-overflow">
                    <RecentsItem/>
                    <RecentsItem/>
                    <RecentsItem/>
                    <RecentsItem/>
                    <RecentsItem/>
                    <RecentsItem/>
                    <RecentsItem/>
                    <RecentsItem/>
                    <RecentsItem/>
                </div>
            </div>
        </div>
    }
}