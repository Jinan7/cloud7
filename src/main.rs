use cloud7::app::App;
use leptos::prelude::*;
use leptos::IntoView;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

