use leptos::prelude::*;
use crate::components::{SideBarNavOptionState, SideBarNavOption};

#[component]
pub fn SideBarNav() -> impl IntoView {
    let (state, set_state) = signal(SideBarNavOptionState::SideBarNavOptionHighlight);
    let (state_off, set_state) = signal(SideBarNavOptionState::SideBarNavOptionNoHighlight);
    view! {
        <div class="sidebar-nav">
            // sidebar nav option
            <SideBarNavOption
                label="Home".to_string()
                icon_name="home".to_string()
                state=state.into()
            />
            <SideBarNavOption
                label="Workspaces".to_string()
                icon_name="workspaces".to_string()
                state=state_off.into()
            />
            <SideBarNavOption
                label="Search".to_string()
                icon_name="search".to_string()
                state=state_off.into()
            />
            <SideBarNavOption
                label="Notifications".to_string()
                icon_name="notifications".to_string()
                state=state_off.into()
            />
        </div>
    }
}