use leptos::prelude::*;
use icons::ChevronDown;

use crate::components::sidebar_section_option::SideBarSectionOption;

#[derive(Clone)]
pub struct SideBarSectionOptionPayload {
    pub key: String,
    pub label: String,
    pub icon_name: String,
}

#[derive(Clone)]
pub struct SelectedSideBarSectionOption(pub String);

#[component]
pub fn SideBarSection(
    label: String,
) -> impl IntoView {

    let sidebar_section_options = use_context::<ReadSignal<Vec<SideBarSectionOptionPayload>>>().expect("Failed to find context sidebar_section_options");
    view! {
        <div class="sidebar-section">
            <div class="labelled-dropdown">
                <p class="inter-12-grey-2">{label}</p>
                <ChevronDown class="icon-16-grey" />
            </div>
            <div class="sidebar-section-content">
                <For
                    each=move || sidebar_section_options.get()
                    key=|sidebar_section_option| sidebar_section_option.key.clone()
                    let(child)
                >
                    <SideBarSectionOption
                        label=child.label.clone()
                        icon_name=child.icon_name.clone()
                    />
                </For>
            </div>
        </div>
    }
}