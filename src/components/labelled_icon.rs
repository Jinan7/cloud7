use leptos::prelude::*;
use icons::{Bell, ChevronDown, Film, House, Image, LayoutGrid, Music, Search, Users, Trash2, File};


pub struct IconLabels;

impl IconLabels {
    pub const HOME: &'static str = "Home";
    pub const WORKSPACES: &'static str = "Workspaces";
    pub const SEARCH: &'static str = "Search";
    pub const NOTIFICATIONS: &'static str = "Notifications";
    pub const DOWN: &'static str = "Down";
    pub const IMAGE: &'static str = "Image";
    pub const FILM: &'static str = "Film";
    pub const FILE: &'static str = "File";
    pub const MUSIC: &'static str = "Music";
    pub const USERS: &'static str = "Users";
    pub const TRASH: &'static str = "Trash";
}
#[component]
pub fn ReactiveLabelledIcon(
    label: String,
    icon_name: Option<String>,
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

                    match icon_name.clone() {
                        Some(s) if s.as_str() == IconLabels::WORKSPACES => view! { <LayoutGrid class=icon_class.get()/> }.into_any(),
                        Some(s) if s.as_str() == IconLabels::SEARCH => view! { <Search class=icon_class.get()/> }.into_any(),
                        Some(s) if s.as_str() == IconLabels::NOTIFICATIONS => view! { <Bell class=icon_class.get()/> }.into_any(),
                        Some(s) if s.as_str() == IconLabels::HOME => view! { <House class=icon_class.get()/> }.into_any(),
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
    icon_name: Option<String>,
    icon_src: String,
    class: String,
    label_class: String,
    icon_class: String,

) -> impl IntoView {

    view! {
        <div class=class>
            {
                match icon_name {
                    Some(s) if s == IconLabels::HOME => view! { <House class=icon_class/> }.into_any(),
                    Some(s) if s == IconLabels::WORKSPACES => view! { <LayoutGrid class=icon_class/> }.into_any(),
                    Some(s) if s == IconLabels::SEARCH => view! { <Search class=icon_class/> }.into_any(),
                    Some(s) if s == IconLabels::NOTIFICATIONS => view! { <Bell class=icon_class/> }.into_any(),
                    Some(s) if s == IconLabels::DOWN => view! { <ChevronDown class=icon_class/> }.into_any(),
                    Some(s) if s == IconLabels::IMAGE => view! { <Image class=icon_class/> }.into_any(),
                    Some(s) if s == IconLabels::FILM => view! { <Film class=icon_class/> }.into_any(),
                    Some(s) if s == IconLabels::FILE => view! { <File class=icon_class/> }.into_any(),
                    Some(s) if s == IconLabels::MUSIC => view! { <Music class=icon_class/> }.into_any(),
                    Some(s) if s == IconLabels::USERS => view! { <Users class=icon_class/> }.into_any(),
                    Some(s) if s == IconLabels::TRASH => view! { <Trash2 class=icon_class/> }.into_any(),
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
    icon_name: Option<String>,
    icon_src: String,
    class: String,
    label_class: String,
    icon_class: String,

) -> impl IntoView {

    view! {
        <div class=class>
            <p class=label_class>{label.clone()}</p>
            {
                match icon_name {
                    Some(s) if s == IconLabels::HOME => view! { <House class=icon_class/> }.into_any(),
                    Some(s) if s == IconLabels::WORKSPACES  => view! { <LayoutGrid class=icon_class/> }.into_any(),
                    Some(s) if s == IconLabels::SEARCH  => view! { <Search class=icon_class/> }.into_any(),
                    Some(s) if s == IconLabels::NOTIFICATIONS  => view! { <Bell class=icon_class/> }.into_any(),
                    Some(s) if s == IconLabels::DOWN  => view! { <ChevronDown class=icon_class/> }.into_any(),
                    _ => view! {
                        <img class=icon_class src=icon_src />
                    }.into_any()
                }    
            }    
        </div>
    }
}