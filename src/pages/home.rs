use crate::components::{Forecast, Map, Navbar};
use crate::utils::local_storage_handler::is_light_theme;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (is_light, set_is_light) = signal(is_light_theme());
    let (is_forecast, set_is_forecast) = signal(true);

    move || {
        let theme_suffix = if is_light.get() { "" } else { "-dark" };

        view! {
                <div class=format!("body{}", theme_suffix)>
                    <Navbar is_light=is_light set_is_light=set_is_light is_forecast=is_forecast set_is_forecast=set_is_forecast/>
                    <div style="width: fit-content; height: fit-content;">
                        <Show when=move || !is_forecast.get() fallback=move ||
                            view! {
                                <Forecast/>
                            }
                        >
                            <Map/>
                        </Show>
                    </div>
                </div>

        }
    }
}
