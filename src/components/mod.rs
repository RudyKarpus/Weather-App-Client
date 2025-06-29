mod forecast;
mod map;
mod navbar;

pub use forecast::Forecast;
pub use map::Map;
pub use navbar::Navbar;

use gloo_net::http::Request;
use serde::de::DeserializeOwned;
#[allow(clippy::extra_unused_lifetimes)]
pub async fn send_get_request<'a, T>(endpoint: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let endpoint = format!("/api{endpoint}");
    log::info!("Sending request to: {endpoint}");
    let request = Request::get(&endpoint).header("Content-Type", "application/json");
    let response = request.send().await?;
    Ok(response.json().await?)
}
