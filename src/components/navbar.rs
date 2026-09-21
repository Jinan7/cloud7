use leptos::prelude::*;
use crate::components::{LabelledIcon, UploadButton};

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <div class="nav">
            <div class="nav-content">
                <LabelledIcon
                    label="Home".to_string()
                    icon_src="./public/home.png".to_string()
                    class="labelled-icon-gap-16".to_string()
                    label_class="inter-normal-black".to_string()
                    icon_class="icon-20".to_string()
                />
                <div class="nav-upload-and-profile">
                    <UploadButton />
                </div>
            </div>
        </div>
    }
}