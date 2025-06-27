#![feature(iter_intersperse)]
use leptos::prelude::*;
use leptos_router::{components::*, path};
#[allow(unused_imports, reason = "Fixes trunk error")]
use web_sys::*;

mod auth;
mod components;
mod pages;
mod utils;
use crate::pages::home::HomePage;
#[allow(clippy::redundant_static_lifetimes, reason = "Will be changed")]
const BACKEND: &'static str = "http://localhost:8000";

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { NotFound }>
                <Route path=path!("/") view=HomePage />
            </Routes>
        </Router>
    }
}

fn main() {
    // better error logging
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    // sets up app
    leptos::mount::mount_to_body(|| view! { <App/> });
}
