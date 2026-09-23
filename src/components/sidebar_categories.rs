use leptos::{attr::Icon, prelude::*};

use crate::components::{SelectedSideBarSectionOption, SideBarSectionOptionPayload, labelled_icon::IconLabels, sidebar_section::SideBarSection};


#[component]
pub fn SideBarCategories() -> impl IntoView {

    let (selected_sidebar_section_option, _set_selected_sidebar_section_option) = signal(SelectedSideBarSectionOption("College".to_string()));
    let (sidebar_categories_options, _set_side_bar_workspaces_options) = signal(
        vec! [
            SideBarSectionOptionPayload {
                key: "Photo".to_string(),
                label: "Photo".to_string(),
                icon_name: IconLabels::IMAGE.to_owned(),

            },
            SideBarSectionOptionPayload {
                key: "Videos".to_string(),
                label: "Videos".to_string(),
                icon_name: IconLabels::FILM.to_owned(),

            },
            SideBarSectionOptionPayload {
                key: "Documents".to_string(),
                label: "Documents".to_string(),
                icon_name: IconLabels::FILE.to_owned(),

            },
            SideBarSectionOptionPayload {
                key: "Audio".to_string(),
                label: "Audio".to_string(),
                icon_name: IconLabels::MUSIC.to_owned(),

            },
            SideBarSectionOptionPayload {
                key: "Shared with me".to_string(),
                label: "Shared with me".to_string(),
                icon_name: IconLabels::USERS.to_owned(),

            },
            SideBarSectionOptionPayload {
                key: "Audio".to_string(),
                label: "Audio".to_string(),
                icon_name: IconLabels::MUSIC.to_owned(),

            },
        ]
    );
    provide_context(sidebar_categories_options);
    provide_context((selected_sidebar_section_option, _set_selected_sidebar_section_option));
    view! {
        <SideBarSection label="Categories".to_string()/>
    }
}