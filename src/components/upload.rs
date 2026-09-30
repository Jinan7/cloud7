use leptos::prelude::*;


#[component]
pub fn Upload() -> impl IntoView {

    view! {
        <div class="upload">
            <div class="upload-icon">
            </div>
            <div class="upload-info">
                <p class="inter-normal-black">"Photograpy"</p>
                <div class="progress">
                    <div class="progress-background">
                        <div class="progress-bar">
                        </div>
                    </div>
                    <p class="inter-12-grey-2">90%</p>
                </div>      
            </div>

        </div>
    }
}