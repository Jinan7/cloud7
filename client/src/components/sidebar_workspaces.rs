use leptos::prelude::*;

use crate::components::{SideBarSectionOptionPayload, sidebar_section::{SelectedSideBarSectionOption, SideBarSection}};


#[component]
pub fn SideBarWorkspaces() -> impl IntoView {
    let (selected_sidebar_section_option, _set_selected_sidebar_section_option) = signal(SelectedSideBarSectionOption("College".to_string()));
    let (sidebar_workspaces_options, _set_side_bar_workspaces_options) = signal(
        vec! [
            SideBarSectionOptionPayload {
                key: "College".to_string(),
                label: "College".to_string(),
                icon_name: "college".to_string(),

            },
            SideBarSectionOptionPayload {
                key: "Work".to_string(),
                label: "Work".to_string(),
                icon_name: "work".to_string(),

            },
            SideBarSectionOptionPayload {
                key: "Family".to_string(),
                label: "Family".to_string(),
                icon_name: "family".to_string(),

            },
            SideBarSectionOptionPayload {
                key: "Others".to_string(),
                label: "Others".to_string(),
                icon_name: "others".to_string(),

            },
        ]
    );

    provide_context(sidebar_workspaces_options);
    provide_context((selected_sidebar_section_option, _set_selected_sidebar_section_option));
    view! { <SideBarSection label="Workspaces".to_string() /> }
}