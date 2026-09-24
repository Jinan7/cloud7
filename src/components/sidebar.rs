use leptos::prelude::*;
use crate::components::{SideBarCategories, SideBarNav, SideBarWorkspaces, SideBarFooter};

#[component]
pub fn SideBar() -> impl IntoView {
   
    view! {
        <div class="sidebar">
            <div class="sidebar-header">
                <SideBarNav />
                <SideBarWorkspaces />
                <SideBarCategories />
            </div>
            <div class="sidebar-footer">
                <SideBarFooter/>
            </div>
        </div>
    }
}