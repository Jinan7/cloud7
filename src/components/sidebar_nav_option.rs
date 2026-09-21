use leptos::prelude::*;

use crate::components::ReactiveLabelledIcon;

#[derive(Clone)]
pub enum SideBarNavOptionState {
    SideBarNavOptionHighlight,
    SideBarNavOptionNoHighlight
}



#[component]
pub fn SideBarNavOption(
    label: String,
    icon_name: String,
    state: Signal<SideBarNavOptionState>
) -> impl IntoView {
    
    
    let derived_class = move || {
        match state.get() {
            SideBarNavOptionState::SideBarNavOptionHighlight => "sidebar-nav-option-highlight".to_string(),
            SideBarNavOptionState::SideBarNavOptionNoHighlight => "sidebar-nav-option".to_string(),
        }
    };

    let derived_icon_src = move || {
        match state.get() {
            SideBarNavOptionState::SideBarNavOptionHighlight => format!("./public/icons/side_bar_nav/{}-highlight.png", icon_name),
            SideBarNavOptionState::SideBarNavOptionNoHighlight => format!("./public/icons/side_bar_nav/{}.png", icon_name)
        }
    };

    let derived_label_class = move || {
        match state.get() {
            SideBarNavOptionState::SideBarNavOptionHighlight => "inter-normal-blue".to_string(),
            SideBarNavOptionState::SideBarNavOptionNoHighlight => "inter-normal-grey".to_string()
        }
    };

    view! {
        <div class=derived_class>
            <ReactiveLabelledIcon
                label=label
                icon_src=Signal::derive(derived_icon_src)
                class="labelled-icon-gap-12".to_string()
                label_class=Signal::derive(derived_label_class)
                icon_class="icon-20".to_string()
            />
        </div>
    }
}