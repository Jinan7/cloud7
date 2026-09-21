use leptos::prelude::*;
use crate::components::{SideBarNav};

#[component]
pub fn SideBar() -> impl IntoView {
    view! {
        
        <div class="sidebar">
            //sidebar nav
            <SideBarNav/>
            <div class="sidebar-workspaces">
            </div>
            <div class="sidebar-categories">
            </div>
        </div>

    }
}