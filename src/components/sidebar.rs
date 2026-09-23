use leptos::prelude::*;
use crate::components::{SideBarCategories, SideBarNav, SideBarWorkspaces};

#[component]
pub fn SideBar() -> impl IntoView {
   
    view! {
        <div class="sidebar">
            <div class="sidebar-header">
                <SideBarNav />
                <SideBarWorkspaces/>
                <SideBarCategories/>
            </div>
            <div class="sidebar-footer">
            </div>
        </div>
    }
}