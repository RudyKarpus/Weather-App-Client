export function get_user_location(callback) {
  if (!navigator.geolocation) {
    console.warn("Geolocation is not supported.");
    return;
  }

  navigator.geolocation.getCurrentPosition(
    (pos) => {
      const lat = pos.coords.latitude;
      const lng = pos.coords.longitude;
      callback(lat, lng);
    },
    (err) => {
      console.error("Geolocation:", err);
    }
  );
}