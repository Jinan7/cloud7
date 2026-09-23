use leptos::prelude::*;

use crate::components::{IconClass, LabelledIcon, sidebar_categories, sidebar_section::SelectedSideBarSectionOption, sidebar_workspaces};



#[component]
pub fn SideBarSectionOption(
    label: String,
    icon_name: String,
) -> impl IntoView {
    
    let (_selected_side_bar_section_option, set_selected_side_bar_section_option) =  use_context::<(ReadSignal<SelectedSideBarSectionOption>,WriteSignal<SelectedSideBarSectionOption>)>().expect("No context for SelectedSideBarNavOption found");
    let icon_class = use_context::<IconClass>()
        .unwrap_or_else(|| IconClass("icon-20".to_string()));
   
    let value = label.clone();
    view! {
        <div class="sidebar-section-option"
            on:click = move |_| { set_selected_side_bar_section_option.set(SelectedSideBarSectionOption(value.clone()));}
        >
            <LabelledIcon
                label=label
                icon_name=Some(icon_name.clone())
                icon_src=format!("./public/icons/side_bar/{}.png", icon_name)
                class="labelled-icon-gap-12".to_string()
                label_class="inter-normal-grey".to_string()
                icon_class=icon_class.0
            />
        </div>
    }
}