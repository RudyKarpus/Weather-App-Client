use web_sys::window;

pub fn is_light_theme() -> bool {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
        if let Ok(Some(saved)) = storage.get_item("theme") {
            return match saved.as_str() {
                "Light" => true,
                "Dark" => false,
                _ => true,
            };
        }
    }

    true
}

pub fn set_theme(is_light: bool) {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
        if is_light {
            let _ = storage.set_item("theme", "Light");
        } else {
            let _ = storage.set_item("theme", "Dark");
        }
    }
}
