use leptos::prelude::*;
use icons::{Bell, ChevronDown, CircleHelp, File, Film, House, Image, LayoutGrid, Music, Search, Settings, Trash2, Users};


pub struct IconNames;

impl IconNames {
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
    pub const SETTINGS: &'static str = "Settings";
     pub const HELP: &'static str = "Help";
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
            {move || {
                match icon_name.clone() {
                    Some(s) if s.as_str() == IconNames::WORKSPACES => {

                        view! { <LayoutGrid class=icon_class.get() /> }
                            .into_any()
                    }
                    Some(s) if s.as_str() == IconNames::SEARCH => {
                        view! { <Search class=icon_class.get() /> }.into_any()
                    }
                    Some(s) if s.as_str() == IconNames::NOTIFICATIONS => {
                        view! { <Bell class=icon_class.get() /> }.into_any()
                    }
                    Some(s) if s.as_str() == IconNames::HOME => {
                        view! { <House class=icon_class.get() /> }.into_any()
                    }
                    _ => view! { <img class=icon_class src=icon_src /> }.into_any(),
                }
            }} <p class=label_class>{value}</p>
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
            {match icon_name {
                Some(s) if s == IconNames::HOME => view! { <House class=icon_class /> }.into_any(),
                Some(s) if s == IconNames::WORKSPACES => {
                    view! { <LayoutGrid class=icon_class /> }.into_any()
                }
                Some(s) if s == IconNames::SEARCH => {
                    view! { <Search class=icon_class /> }.into_any()
                }
                Some(s) if s == IconNames::NOTIFICATIONS => {
                    view! { <Bell class=icon_class /> }.into_any()
                }
                Some(s) if s == IconNames::DOWN => {
                    view! { <ChevronDown class=icon_class /> }.into_any()
                }
                Some(s) if s == IconNames::IMAGE => view! { <Image class=icon_class /> }.into_any(),
                Some(s) if s == IconNames::FILM => view! { <Film class=icon_class /> }.into_any(),
                Some(s) if s == IconNames::FILE => view! { <File class=icon_class /> }.into_any(),
                Some(s) if s == IconNames::MUSIC => view! { <Music class=icon_class /> }.into_any(),
                Some(s) if s == IconNames::USERS => view! { <Users class=icon_class /> }.into_any(),
                Some(s) if s == IconNames::TRASH => {
                    view! { <Trash2 class=icon_class /> }.into_any()
                }
                Some(s) if s == IconNames::SETTINGS => {
                    view! { <Settings class=icon_class /> }.into_any()
                }
                Some(s) if s == IconNames::HELP => {
                    view! { <CircleHelp class=icon_class /> }.into_any()
                }
                _ => view! { <img class=icon_class src=icon_src /> }.into_any(),
            }} <p class=label_class>{label}</p>
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
            {match icon_name {
                Some(s) if s == IconNames::HOME => view! { <House class=icon_class /> }.into_any(),
                Some(s) if s == IconNames::WORKSPACES => {
                    view! { <LayoutGrid class=icon_class /> }.into_any()
                }
                Some(s) if s == IconNames::SEARCH => {
                    view! { <Search class=icon_class /> }.into_any()
                }
                Some(s) if s == IconNames::NOTIFICATIONS => {
                    view! { <Bell class=icon_class /> }.into_any()
                }
                Some(s) if s == IconNames::DOWN => {
                    view! { <ChevronDown class=icon_class /> }.into_any()
                }
                _ => view! { <img class=icon_class src=icon_src /> }.into_any(),
            }}
        </div>
    }
}