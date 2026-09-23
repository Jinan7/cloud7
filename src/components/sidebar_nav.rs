use leptos::prelude::*;
use crate::components::{SideBarNavOption, labelled_icon::IconLabels};

#[derive(Clone)]
struct SideBarNavOptionPayload {
    key: String,
    label: String,
    icon_name: String
}

#[derive(Clone)]
pub struct SelectedSideBarNavOption(pub String);

#[component]
pub fn SideBarNav() -> impl IntoView {
    
    let (selected_sidebar_nav_option, set_selected_sidebar_nav_option) = signal(SelectedSideBarNavOption("Home".to_string()));
    provide_context((selected_sidebar_nav_option, set_selected_sidebar_nav_option));

    let (sidebar_nav_options, _set_sidebar_nav_options) = signal(vec! [
        SideBarNavOptionPayload {
            key: "Home".to_string(),
            label: IconLabels::HOME.to_owned(),
            icon_name: IconLabels::HOME.to_owned(),
        },
        SideBarNavOptionPayload {
            key: "Workspaces".to_string(),
            label: IconLabels::WORKSPACES.to_owned(),
            icon_name: IconLabels::WORKSPACES.to_owned(),
        },
        SideBarNavOptionPayload {
            key: "Search".to_string(),
            label: IconLabels::SEARCH.to_owned(),
            icon_name: IconLabels::SEARCH.to_owned(),
        },
        SideBarNavOptionPayload {
            key: "Notifications".to_string(),
            label: IconLabels::NOTIFICATIONS.to_owned(),
            icon_name: IconLabels::NOTIFICATIONS.to_owned(),
        },
    ]);

    view! {
        <div class="sidebar-nav">
            // sidebar nav option

            <For 
                each = move || sidebar_nav_options.get()
                key = |sidebar_nav_option| sidebar_nav_option.key.clone()
                let(child)
            >
                <SideBarNavOption
                    label=child.label.clone()
                    icon_name=child.icon_name.clone()
                />
            </For>
            
        </div>
    }
}