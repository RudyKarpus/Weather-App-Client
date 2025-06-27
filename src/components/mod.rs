mod forecast;
mod map;
mod navbar;

pub use forecast::Forecast;
pub use map::Map;
pub use navbar::Navbar;

use crate::{BACKEND, auth};
use gloo_net::http::Request;
use leptos::context::use_context;
use serde::de::DeserializeOwned;
#[allow(clippy::extra_unused_lifetimes)]
pub async fn send_get_request<'a, T>(endpoint: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let endpoint = format!("{BACKEND}{endpoint}");
    log::info!("Sending request to: {endpoint}");
    let mut request = Request::get(&endpoint).header("Content-Type", "application/json");
    if let Some(Some(token)) = use_context::<Option<auth::Token>>() {
        request = request.header("Authorization", &format!("Bearer {}", &token as &str))
    } else if let Some(token) = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item("access_token")
        .unwrap()
    {
        request = request.header("Authorization", &format!("Bearer {}", &token as &str))
    }

    let response = request.send().await?;
    Ok(response.json().await?)
}
