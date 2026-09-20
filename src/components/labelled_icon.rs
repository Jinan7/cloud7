use leptos::prelude::*;

#[component]
pub fn ReactiveLabelledIcon(
    label: String,
    icon_src: Signal<String>,
    class: String,
    label_class: Signal<String>,
    icon_class: String,

) -> impl IntoView {

    view! {
        <div class=class>
            <img class=icon_class src=icon_src/>
            <p class=label_class>{label}</p>
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
            <img class=icon_class src=icon_src/>
            <p class=label_class>{label}</p>
        </div>

    }
}