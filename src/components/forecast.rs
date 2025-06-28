use crate::pages::home::{DayData, WeeklySummaryData};
use crate::utils::date_utils::get_weekday_from_string;
use crate::utils::weather_utils::{get_weather_category, get_weather_icon};
use leptos::prelude::*;

#[component]
pub fn forecast(
    day_data: ReadSignal<Vec<DayData>>,
    weekly_data: ReadSignal<WeeklySummaryData>,
    is_light: ReadSignal<bool>,
) -> impl IntoView {
    let theme_suffix = if is_light.get() { "" } else { "-dark" };
    view! {
        <div  style="width: 100%; display: flex;
             margin-top:20px; align-items: center; justify-content: center;">
                <table class=format!("forecast-table{}", theme_suffix) style="width: 95%; border-collapse: collapse;">
                    <thead>
                        <tr>
                            <th class=format!("text-table{}", theme_suffix)>"Date"</th>
                            <th class=format!("text-table{}", theme_suffix)>"Weather Code"</th>
                            <th class=format!("text-table{}", theme_suffix)>"Min Temp (C)"</th>
                            <th class=format!("text-table{}", theme_suffix)>"Max Temp (C)"</th>
                            <th class=format!("text-table{}", theme_suffix)>"Estimated Energy (kWh)"</th>
                        </tr>
                    </thead>
                    <tbody>

                        {move || day_data.get().iter().map(|data| view! {
                            <tr>
                                <td class=format!("text-table{}", theme_suffix)>{format!("{} - {}", data.date.clone(), get_weekday_from_string(&data.date.clone()).unwrap_or("Invalid".to_string().to_string()))}</td>
                                <td class=format!("text-table{}", theme_suffix)>
                                    {get_weather_icon(get_weather_category(data.weather_code))}
                                </td>
                                <td class=format!("text-table{}", theme_suffix)>{format!("{:.1}", data.temp_min)}</td>
                                <td class=format!("text-table{}", theme_suffix)>{format!("{:.1}", data.temp_max)}</td>
                                <td class=format!("text-table{}", theme_suffix)>{format!("{:.1}", data.estimated_energy)}</td>

                            </tr>
                        }).collect_view()}
                    </tbody>
                    <tfoot>
                        <tr>
                            <td colspan=5 class=format!("text-table{}", theme_suffix)>
                                "Week Overview"
                            </td>
                        </tr>
                        <tr>
                            <td colspan=1 class=format!("text-table{}", theme_suffix)>
                                "Summary"
                            </td>
                            <td colspan=1 class=format!("text-table{}", theme_suffix)>
                                "Min temp (°C)"
                            </td>
                            <td colspan=1 class=format!("text-table{}", theme_suffix)>
                                "Max temp (°C)"
                            </td>
                            <td colspan=1 class=format!("text-table{}", theme_suffix)>
                                "Average pressusre hPa"
                            </td>
                            <td colspan=1 class=format!("text-table{}", theme_suffix)>
                                "Average sunshine hours"
                            </td>
                        </tr>
                        <tr>
                            <td colspan=1 class=format!("text-table{}", theme_suffix)>
                                {move || weekly_data.get().weekly_summary.clone()}
                            </td>
                            <td colspan=1 class=format!("text-table{}", theme_suffix)>
                                {move || weekly_data.get().min_temperature}
                            </td>
                            <td colspan=1 class=format!("text-table{}", theme_suffix)>
                                {move || weekly_data.get().max_temperature}
                            </td>
                            <td colspan=1 class=format!("text-table{}", theme_suffix)>
                                {move || weekly_data.get().average_pressure}
                            </td>
                            <td colspan=1 class=format!("text-table{}", theme_suffix)>
                                {move || weekly_data.get().average_sunshine_hours}
                            </td>
                        </tr>

                    </tfoot>
                </table>
        </div>
    }
}
