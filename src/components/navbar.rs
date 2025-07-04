use crate::utils::local_storage_handler::set_theme;
use leptos::prelude::*;
#[component]
pub fn navbar(
    is_light: ReadSignal<bool>,
    set_is_light: WriteSignal<bool>,
    is_forecast: ReadSignal<bool>,
    set_is_forecast: WriteSignal<bool>,
) -> impl IntoView {
    let theme_suffix = if is_light.get() { "" } else { "-dark" };

    view! {
        <header class=format!("nav{}", theme_suffix) style="display: flex; align-items: center; justify-content: space-between;">
            <div class=format!("text-title{}",theme_suffix) style="margin-top: 0px;">"WeatherUp"</div>
            <nav style="display: flex; justify-content: center; flex: 1; gap:20px;">
                <button class= move || format!("button{}{}", theme_suffix,
                    if is_forecast.get() {
                        " active"
                    } else {""})
                on:click=move |_| {
                        set_is_forecast(true);
                    }
                >
                    "Forecast"
                </button>
                <button class= move ||format!("button{}{}", theme_suffix,
                    if !is_forecast.get() {
                        " active"
                    } else {""})
                    on:click=move |_| {
                        set_is_forecast(false);
                    }
                >
                    "Map"
                </button>
            </nav>
            <nav style="display: flex; justify-content: center;">

                <div class="theme-switch">
                    <input type="checkbox" id="theme-checkbox"
                        prop:checked=move || is_light.get()
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_is_light(checked);
                            set_theme(checked);
                    }/>
                    <label for="theme-checkbox">
                        <div></div>
                        <span>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            class="w-6 h-6"
                        >
                            <path
                            fill-rule="evenodd"
                            d="M9.528 1.718a.75.75 0 01.162.819A8.97 8.97 0 009 6a9 9 0 009 9 8.97 8.97 0 003.463-.69.75.75 0 01.981.98 10.503 10.503 0 01-9.694 6.46c-5.799 0-10.5-4.701-10.5-10.5 0-4.368 2.667-8.112 6.46-9.694a.75.75 0 01.818.162z"
                            clip-rule="evenodd"
                            ></path>
                        </svg>
                        </span>
                        <span>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            class="w-6 h-6"
                        >
                            <path
                            d="M12 2.25a.75.75 0 01.75.75v2.25a.75.75 0 01-1.5 0V3a.75.75 0 01.75-.75zM7.5 12a4.5 4.5 0 119 0 4.5 4.5 0 01-9 0zM18.894 6.166a.75.75 0 00-1.06-1.06l-1.591 1.59a.75.75 0 101.06 1.061l1.591-1.59zM21.75 12a.75.75 0 01-.75.75h-2.25a.75.75 0 010-1.5H21a.75.75 0 01.75.75zM17.834 18.894a.75.75 0 001.06-1.06l-1.59-1.591a.75.75 0 10-1.061 1.06l1.59 1.591zM12 18a.75.75 0 01.75.75V21a.75.75 0 01-1.5 0v-2.25A.75.75 0 0112 18zM7.758 17.303a.75.75 0 00-1.061-1.06l-1.591 1.59a.75.75 0 001.06 1.061l1.591-1.59zM6 12a.75.75 0 01-.75.75H3a.75.75 0 010-1.5h2.25A.75.75 0 016 12zM6.697 7.757a.75.75 0 001.06-1.06l-1.59-1.591a.75.75 0 00-1.061 1.06l1.59 1.591z"
                            ></path>
                        </svg>
                        </span>
                    </label>
                </div>
            </nav>
        </header>
    }
}
