use leptos::prelude::*;
use icons::{House, LayoutGrid, Search, Bell, ChevronDown};


pub struct IconLabels;

impl IconLabels {
    pub const HOME: &'static str = "Home";
    pub const WORKSPACES: &'static str = "Workspaces";
    pub const SEARCH: &'static str = "Search";
    pub const NOTIFICATIONS: &'static str = "Notifications";
    pub const DOWN: &'static str = "Down";
}
#[component]
pub fn ReactiveLabelledIcon(
    label: String,
    icon_src: Signal<String>,
    class: String,
    label_class: Signal<String>,
    icon_class: Signal<String>,

) -> impl IntoView {

    let value = label.clone();
    view! {
        <div class=class>
            { 
                move || {

                    match label.as_str() {
                        s if s == IconLabels::WORKSPACES => view! { <LayoutGrid class=icon_class.get()/> }.into_any(),
                        s if s == IconLabels::SEARCH => view! { <Search class=icon_class.get()/> }.into_any(),
                        s if s == IconLabels::NOTIFICATIONS => view! { <Bell class=icon_class.get()/> }.into_any(),
                        s if s == IconLabels::HOME => view! { <House class=icon_class.get()/> }.into_any(),
                        _ => view! {
                            <img class=icon_class src=icon_src />
                        }.into_any()
                    } 
                }      
            }
            <p class=label_class>{value}</p>
        </div>
    }
}

#[component]
pub fn LabelledIcon(
    label: String,
    icon_src: String,
    class: String,
    label_class: String,
    icon_class: String,

) -> impl IntoView {

    view! {
        <div class=class>
            {
                match label.as_str() {
                    "Home" => view! { <House class=icon_class/> }.into_any(),
                    "Workspaces" => view! { <LayoutGrid class=icon_class/> }.into_any(),
                    "Search" => view! { <Search class=icon_class/> }.into_any(),
                    "Notifications" => view! { <Bell class=icon_class/> }.into_any(),
                    _ => view! {
                        <img class=icon_class src=icon_src />
                    }.into_any()
                }    
            }
            <p class=label_class>{label}</p>
        </div>
    }
}

#[component]
pub fn InvertedLabelledIcon(
    label: String,
    icon_src: String,
    class: String,
    label_class: String,
    icon_class: String,

) -> impl IntoView {

    view! {
        <div class=class>
            <p class=label_class>{label.clone()}</p>
            {
                match label.as_str() {
                    s if s == IconLabels::HOME => view! { <House class=icon_class/> }.into_any(),
                    s if s == IconLabels::WORKSPACES  => view! { <LayoutGrid class=icon_class/> }.into_any(),
                    s if s == IconLabels::SEARCH  => view! { <Search class=icon_class/> }.into_any(),
                    s if s == IconLabels::NOTIFICATIONS  => view! { <Bell class=icon_class/> }.into_any(),
                    s if s == IconLabels::DOWN  => view! { <ChevronDown class=icon_class/> }.into_any(),
                    _ => view! {
                        <img class=icon_class src=icon_src />
                    }.into_any()
                }    
            }    
        </div>
    }
}