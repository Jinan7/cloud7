use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn Hero() -> impl IntoView {

    view! { 
        <div class="hero">
            <Outlet/>
        </div> 
    }
}