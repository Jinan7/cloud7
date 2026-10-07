use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::{components::{NavBar, SideBar}, pages::components::hero::UploadSection};

#[component]
pub fn Hero() -> impl IntoView {
   
    view! {
        <div class="hero">
            <SideBar />
            <div class="hero-content">
                <NavBar />
                <div class="main">
                    <div class="outlet">
                        <Outlet/>
                    </div> 
                    <UploadSection/>
                </div>
            </div>
        </div>
    }
}