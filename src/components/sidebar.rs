use icons::ChevronDown;
use leptos::prelude::*;
use crate::components::{SideBarNav};

#[component]
pub fn SideBar() -> impl IntoView {
   
    view! {
        <div class="sidebar">
            <div class="sidebar-header">
                <SideBarNav />
                <div class="sidebar-workspaces">
                    <div class="labelled-dropdown">
                        <p class="inter-12-grey-2">"Workspaces"</p>
                        <ChevronDown class="icon-16-grey"/>
                    </div>
                    <div>
                    </div>
                </div>
                <div class="sidebar-categories"></div>
            </div>
            <div class="sidebar-footer">
            </div>
        </div>
    }
}