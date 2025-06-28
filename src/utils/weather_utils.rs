pub fn get_weather_category(code: usize) -> &'static str {
    match code {
        0 => "sunny",
        1 | 2 | 3 | 45 | 48 => "cloudy",
        51 | 53 | 55 | 56 | 57 | 61 | 63 | 65 | 66 | 67 | 80 | 81 | 82 | 95 | 96 | 99 => "rainy",
        71 | 73 | 75 | 77 | 85 | 86 => "snowy",
        _ => "unknown",
    }
}

pub fn get_weather_icon(category: &str) -> &'static str {
    match category {
        "sunny" => "☀️",
        "cloudy" => "☁️",
        "rainy" => "🌧️",
        "snowy" => "❄️",
        _ => "❓",
    }
}
