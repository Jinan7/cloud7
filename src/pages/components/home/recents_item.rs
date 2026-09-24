use icons::Folder;
use leptos::prelude::*;

#[component]
pub fn RecentsItem() -> impl IntoView {

    view!{
        <div class="recents-item">
            <Folder class="icon-65-blue"/>
            <p class="inter-13-grey-4">"Travel 2026"</p>
        </div>
    }
}