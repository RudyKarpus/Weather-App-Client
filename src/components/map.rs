use crate::wasm_bindgen::prelude::wasm_bindgen;
use leptos::prelude::*;

#[wasm_bindgen(module = "/public/init_map.js")]
extern "C" {
    fn init_map();
}

#[component]
pub fn map() -> impl IntoView {
    Effect::new(move |_| {
        init_map();
    });
    view! {
        <div style="margin-left: 15vw; margin-top:5vh;">
            <div id="map"></div>
        </div>
    }
}
