use leptos::prelude::*;
use icons::{House, LayoutGrid, Search, Bell};
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
                        "Home" => view! { <House class=icon_class.get()/> }.into_any(),
                        "Workspaces" => view! { <LayoutGrid class=icon_class.get()/> }.into_any(),
                        "Search" => view! { <Search class=icon_class.get()/> }.into_any(),
                        "Notifications" => view! { <Bell class=icon_class.get()/> }.into_any(),
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