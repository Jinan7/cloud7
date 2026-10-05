use leptos::{prelude::*};


#[component]
pub fn Upload(
    file_name: String,
    percentage: u8
) -> impl IntoView {

    view! {
        <div class="upload">
            <div class="upload-icon">
            </div>
            <div class="upload-info">
                <p class="inter-normal-black">{file_name}</p>
                <div class="progress">
                    <div class="progress-background">
                        <div 
                            class="progress-bar" 
                            style:width = format!("{}%", percentage)
                        >
                        </div>
                    </div>
                    <p class="inter-12-grey-2">{format!("{}%", percentage)}</p>
                </div>      
            </div>

        </div>
    }
}