use crate::components::send_get_request;
use crate::components::{Forecast, Map, Navbar};
use crate::utils::local_storage_handler::is_light_theme;
use crate::wasm_bindgen::JsCast;
use crate::wasm_bindgen::closure::Closure;
use crate::wasm_bindgen::prelude::wasm_bindgen;
use js_sys::Function;
use leptos::prelude::*;
use log::Level;
use serde::Deserialize;
use web_sys::js_sys;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct DayData {
    pub weather_code: usize,
    pub date: String,
    pub temp_min: f64,
    pub temp_max: f64,
    pub estimated_energy: f64,
}

async fn get_weekly_data(lat_lng: (f64, f64)) -> anyhow::Result<Vec<DayData>> {
    let endpoint = format!("/weather/weekly/data/{}/{}/", lat_lng.0, lat_lng.1);

    return send_get_request(&endpoint).await;
}

#[component]
pub fn HomePage() -> impl IntoView {
    let (is_light, set_is_light) = signal(is_light_theme());
    let (is_forecast, set_is_forecast) = signal(true);
    let (location, set_location) = signal::<(f64, f64)>((0.0, 0.0));
    let (day_data, set_day_data) = signal::<Vec<DayData>>(vec![]);
    let (lat_input, set_lat_input) = signal(0.0);
    let (lng_input, set_lng_input) = signal(0.0);
    let (error_lat, set_error_lat) = signal(None::<String>);
    let (error_lng, set_error_lng) = signal(None::<String>);
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
    // update input value
    Effect::new(move |_| {
        set_lat_input(location.get().0);
        set_lng_input(location.get().1);
    });

    //getting weekly data
    let weekly_data = LocalResource::new(move || get_weekly_data(location.get()));
    Effect::new(move || match &*weekly_data.read() {
        Some(Ok(res)) => set_day_data(res.to_vec()),
        Some(Err(err)) => {
            log::log!(Level::Error, "{err}");
        }
        _ => log::info!("Receivecfd"),
    });

    move || {
        let theme_suffix = if is_light.get() { "" } else { "-dark" };
        view! {
                <div class=format!("body{}", theme_suffix)>
                    <Navbar is_light=is_light set_is_light=set_is_light is_forecast=is_forecast set_is_forecast=set_is_forecast/>
                    <div style="width: fit-content; height: fit-content;">
                        <Show when=move || !is_forecast.get() fallback=move ||
                            view! {
                                <div style="display: flex; justify-content: center; gap:20px; width:100vw; margin-top:20px;">
                                    <div style="display: flex; flex-direction: column; align-items: center;">
                                        <input type="number" placeholder="Latitude" required  min="-90" max="90" step="any" class=format!("input-coordinates{}", theme_suffix)
                                            value=move || location().0.to_string()
                                            on:input=move |ev| {
                                                let value = event_target_value(&ev);
                                                if let Ok(number) = value.parse::<f64>() {
                                                    set_lat_input.set(number);
                                                    if number < -90.0{
                                                        set_error_lat(Some("Cannot be less than 90".to_string()));
                                                    } else if number> 90.0 {
                                                        set_error_lat(Some("Cannot be greater than 90".to_string()));

                                                    } else {
                                                        set_error_lat(Some("".to_string()));
                                                    }
                                                }
                                        }/>
                                        <div class=format!("text-error{}", theme_suffix)>{error_lat.get()}</div>
                                    </div>
                                    <div style="display: flex; flex-direction: column; align-items: center;">
                                        <input type="number" placeholder="Longitude" required min="-180" max="180" step="any" inputmode="decimal" class=format!("input-coordinates{}", theme_suffix)
                                            value=move || location().1.to_string()
                                            on:input=move |ev| {
                                                let value = event_target_value(&ev);
                                                if let Ok(number) = value.parse::<f64>() {
                                                    set_lng_input.set(number);
                                                    if number < -180.0{
                                                        set_error_lng(Some("Cannot be less than 180".to_string()));
                                                    } else if number> 180.0 {
                                                        set_error_lng(Some("Cannot be greater than 180".to_string()));

                                                    } else {
                                                        set_error_lng(Some("".to_string()));
                                                    }
                                                }
                                        }/>
                                        <div class=format!("text-error{}", theme_suffix)>{error_lng.get()}</div>
                                    </div>
                                    <button class= format!("button-search{}", theme_suffix) on:click=move |_| {
                                        set_location((lat_input.get(), lng_input.get()));
                                    }>
                                        "Find"
                                    </button>
                                </div>
                                <Forecast day_data=day_data is_light=is_light/>
                            }
                        >
                            <Map set_is_forecast=set_is_forecast set_location=set_location/>
                        </Show>
                    </div>
                </div>

        }
    }
}
