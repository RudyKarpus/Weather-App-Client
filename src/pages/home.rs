use crate::components::Navbar;
use crate::utils::local_storage_handler::is_light_theme;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (is_light, set_is_light) = signal(is_light_theme());

    move || {
        let theme_suffix = if is_light.get() { "" } else { "-dark" };

        view! {
                <div class=format!("body{}", theme_suffix)>
                    <Navbar is_light=is_light set_is_light=set_is_light/>
                </div>

        }
    }
}
