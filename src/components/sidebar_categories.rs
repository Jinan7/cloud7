use leptos::prelude::*;

use crate::components::{SelectedSideBarSectionOption, SideBarSectionOptionPayload, sidebar_section::SideBarSection};


#[component]
pub fn SideBarCategories() -> impl IntoView {

    let (selected_sidebar_section_option, _set_selected_sidebar_section_option) = signal(SelectedSideBarSectionOption("College".to_string()));
    let (sidebar_workspaces_options, _set_side_bar_workspaces_options) = signal(
        vec! [
            SideBarSectionOptionPayload {
                key: "Photo".to_string(),
                label: "Photo".to_string(),
                icon_name: "photo".to_string(),

            },
            SideBarSectionOptionPayload {
                key: "Videos".to_string(),
                label: "Videos".to_string(),
                icon_name: "videos".to_string(),

            },
            SideBarSectionOptionPayload {
                key: "Documents".to_string(),
                label: "Documents".to_string(),
                icon_name: "documents".to_string(),

            },
            SideBarSectionOptionPayload {
                key: "Audio".to_string(),
                label: "Audio".to_string(),
                icon_name: "audio".to_string(),

            },
        ]
    );
    
    view! {
        <SideBarSection label="Categories".to_string()/>
    }
}