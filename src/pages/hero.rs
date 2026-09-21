use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::{NavBar, SideBar};

#[component]
pub fn Hero() -> impl IntoView {
    
    view! {
        <div class="hero">
            <SideBar />
            <div class="hero-content">
                <NavBar />
                <Outlet />
            </div>
        </div>
    }
}