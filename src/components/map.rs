use crate::wasm_bindgen::closure::Closure;
use crate::wasm_bindgen::prelude::wasm_bindgen;
#[warn(unused_imports, reason = "Is is used in line 29")]
use js_sys::{Function, Reflect};
use leptos::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{js_sys, window};

#[component]
pub fn map(
    set_is_forecast: WriteSignal<bool>,
    set_location: WriteSignal<(f64, f64)>,
) -> impl IntoView {
    #[wasm_bindgen(module = "/public/init_map.js")]
    extern "C" {
        fn init_map();
    }

    Effect::new(move |_| {
        //getting callback from pinpointing location
        let closure = Closure::wrap(Box::new(move |lat: f64, lng: f64| {
            #[allow(clippy::uninlined_format_args)]
            web_sys::console::log_1(&format!("Chosen place: lat {}, lng {}", lat, lng).into());
            set_location((lat, lng));
            set_is_forecast(true);
        }) as Box<dyn FnMut(f64, f64)>);

        let window = window().expect("no global `window` exists");

        Reflect::set(
            &window,
            &JsValue::from_str("choose_place_from_map"),
            closure.as_ref().unchecked_ref::<Function>(),
        )
        .expect("failed to set window.choose_place_from_map");

        closure.forget();
        //initializing map
        init_map();
    });
    view! {
        <div style="margin-left: 15vw; margin-top:5vh;">
            <div id="map"></div>
        </div>
    }
}
