use leptos::prelude::*;

use crate::pages::components::home::FileFilterItem;

#[component]
pub fn FileFilterSection() -> impl IntoView {

    view! {
        <div class="files-filter">
            <div class="files-filter-overflow">
                <FileFilterItem/>
                <FileFilterItem/>
                <FileFilterItem/>
                <FileFilterItem/>
                <FileFilterItem/>
                <FileFilterItem/>
                <FileFilterItem/>
                <FileFilterItem/>
                <FileFilterItem/>
                <FileFilterItem/>
            </div>
        </div>
    }
}