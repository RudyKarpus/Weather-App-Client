use crate::components::{Forecast, Map, Navbar};
use crate::utils::local_storage_handler::is_light_theme;
use crate::wasm_bindgen::JsCast;
use crate::wasm_bindgen::closure::Closure;
use crate::wasm_bindgen::prelude::wasm_bindgen;
use js_sys::Function;
use leptos::prelude::*;
use web_sys::js_sys;

#[component]
pub fn HomePage() -> impl IntoView {
    let (is_light, set_is_light) = signal(is_light_theme());
    let (is_forecast, set_is_forecast) = signal(true);
    #[allow(unused_variables, reason = "Will be used in future")]
    let (location, set_location) = signal::<(f64, f64)>((0.0, 0.0));
    //getting user's current location
    #[wasm_bindgen(module = "/public/geolocation.js")]
    extern "C" {
        fn get_user_location(callback: &Function);
    }

    let callback = Closure::wrap(Box::new(move |lat: f64, lng: f64| {
        #[allow(clippy::uninlined_format_args)]
        web_sys::console::log_1(&format!("Chosen place: lat {}, lng {}", lat, lng).into());
        set_location((lat, lng));
    }) as Box<dyn FnMut(f64, f64)>);

    Effect::new(move |_| {
        get_user_location(callback.as_ref().unchecked_ref());
    });

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
                            <Map set_is_forecast=set_is_forecast set_location=set_location/>
                        </Show>
                    </div>
                </div>

        }
    }
}
