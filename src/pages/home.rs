use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {

    view! { 
        <div class="home">
            <div class="nav">
                <div class="nav-content">
                    <div class="nav-home">
                        <img class="icon-20" src="./public/home.png"/>
                        <p class="inter-normal-black">"Home"</p>
                    </div>
                    <div class="nav-upload-and-profile">
                        <div class="upload-button">
                            <img class="icon-20" src="./public/upload.png"/>
                            <p class="inter-normal-white">"Upload file"</p>
                        </div>
                    </div>
                </div>
            </div>
            <div class="search">
            </div>
            <div class="recents">
            </div>
            <div class="files">
            </div>
        </div> 
    }
}