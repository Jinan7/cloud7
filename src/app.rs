use leptos::prelude::*;
use leptos_router::{components::{ParentRoute, Route, Router, Routes}, path};

use crate::pages::{Hero, Home, NotFound};



#[component]
pub fn App() -> impl IntoView {

    view! {
        <Router>
            <nav></nav>
            <main>
                <Routes fallback=|| NotFound>
                    <ParentRoute path=path!("/") view=Hero>
                        <Route path=path!("") view=Home />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}