use leptos::prelude::*;

use crate::components::{ReactiveLabelledIcon, SelectedSideBarNavOption};



#[component]
pub fn SideBarNavOption(
    label: String,
    icon_name: String,
) -> impl IntoView {
    
    let (selected_side_bar_nav_option, set_selected_side_bar_nav_option) =  use_context::<(ReadSignal<SelectedSideBarNavOption>,WriteSignal<SelectedSideBarNavOption>)>().expect("No context for SelectedSideBarNavOption found");
    
    let labelfn = label.clone();
    let derived_class = move || {
        if selected_side_bar_nav_option.get().0 == labelfn {
            "sidebar-nav-option-highlight".to_string()
        } else {
            "sidebar-nav-option".to_string()
        }
        
    };

    let labelfn = label.clone();
    let icon_name_fn = icon_name.clone();
    let derived_icon_src = move || {
        if selected_side_bar_nav_option.get().0 == labelfn {
            format!("./public/icons/side_bar_nav/{}-highlight.png", icon_name_fn)
        } else {
            format!("./public/icons/side_bar_nav/{}.png", icon_name_fn)
        }
    };

    let labelfn = label.clone();
    let derived_label_class = move || {
        if selected_side_bar_nav_option.get().0 == labelfn {
            "inter-normal-blue".to_string()
        } else {
            "inter-normal-grey".to_string()
        }
    };

    let labelfn = label.clone();
    let derived_icon_class = move || {
        if selected_side_bar_nav_option.get().0 == labelfn {
            "icon-20-blue".to_string()
        } else {
            "icon-20-grey".to_string()
        }
    };

    let labelfn = label.clone();
    view! {
        <div
            class=derived_class
            on:click=move |_| {
                set_selected_side_bar_nav_option.set(SelectedSideBarNavOption(labelfn.clone()));
            }
        >
            <ReactiveLabelledIcon
                label=label
                icon_name=Some(icon_name)
                icon_src=Signal::derive(derived_icon_src)
                class="labelled-icon-gap-12".to_string()
                label_class=Signal::derive(derived_label_class)
                icon_class=Signal::derive(derived_icon_class)
            />
        </div>
    }
}