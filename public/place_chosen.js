export default function init_place_chosen(lat, lng) {
  if (window.choose_place_from_map) {
    window.choose_place_from_map(lat, lng);
  } else {
    console.warn("Rust callback not registered");
  }
}