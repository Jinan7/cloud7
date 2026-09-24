use leptos::{prelude::*};

use crate::components::{SelectedSideBarSectionOption, SideBarSectionOptionPayload, labelled_icon::IconNames, sidebar_section::SideBarSection};

#[derive(Clone)]
pub struct IconClass (pub String);
#[component]
pub fn SideBarCategories() -> impl IntoView {

    let (selected_sidebar_section_option, _set_selected_sidebar_section_option) = signal(SelectedSideBarSectionOption("College".to_string()));
    let (sidebar_categories_options, _set_side_bar_workspaces_options) = signal(
        vec! [
            SideBarSectionOptionPayload {
                key: "Photo".to_string(),
                label: "Photo".to_string(),
                icon_name: IconNames::IMAGE.to_owned(),

            },
            SideBarSectionOptionPayload {
                key: "Videos".to_string(),
                label: "Videos".to_string(),
                icon_name: IconNames::FILM.to_owned(),

            },
            SideBarSectionOptionPayload {
                key: "Documents".to_string(),
                label: "Documents".to_string(),
                icon_name: IconNames::FILE.to_owned(),

            },
            SideBarSectionOptionPayload {
                key: "Audio".to_string(),
                label: "Audio".to_string(),
                icon_name: IconNames::MUSIC.to_owned(),

            },
            SideBarSectionOptionPayload {
                key: "Shared with me".to_string(),
                label: "Shared with me".to_string(),
                icon_name: IconNames::USERS.to_owned(),

            },
            SideBarSectionOptionPayload {
                key: "Audio".to_string(),
                label: "Audio".to_string(),
                icon_name: IconNames::MUSIC.to_owned(),

            },
        ]
    );
    provide_context(sidebar_categories_options);
    provide_context((selected_sidebar_section_option, _set_selected_sidebar_section_option));
    provide_context(IconClass("icon-20-grey".to_string()));
    view! { <SideBarSection label="Categories".to_string() /> }
}