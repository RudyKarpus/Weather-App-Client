use chrono::NaiveDate;

pub fn get_weekday_from_string(date_str: &str) -> Option<String> {
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        Some(date.format("%A").to_string())
    } else {
        None
    }
}
