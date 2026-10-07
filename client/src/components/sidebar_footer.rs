use leptos::prelude::*;

use crate::components::{SideBarSectionOptionPayload, labelled_icon::IconNames, SideBarSectionOption};


#[component]
pub fn SideBarFooter() -> impl IntoView {

    let (sidebar_footer_options, _set_sidebar_footer_options )  = signal(
        vec![
            SideBarSectionOptionPayload {
                key: "Settings".to_string(),
                label: "Settings".to_string(),
                icon_name: IconNames::SETTINGS.to_owned()
            },
            SideBarSectionOptionPayload {
                key: "Help".to_string(),
                label: "Help & Support".to_string(),
                icon_name: IconNames::HELP.to_owned(),
            },
        ]
    );

    view! {
        
        <For 
            each = move || sidebar_footer_options.get()
            key = move |sidebar_footer_option| sidebar_footer_option.key.clone()
            let(child)
        >
            <SideBarSectionOption
                label=child.label
                icon_name=child.icon_name
            />
        </For>
    }
}