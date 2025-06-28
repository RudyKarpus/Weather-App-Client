import init_place_chosen from "/place_chosen.js";

let userMarker = null;
let chosenMarker = null;

export function init_map() {
  const map = L.map('map').setView([52.237, 21.017], 4);

  L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    attribution: '&copy; OpenStreetMap contributors'
  }).addTo(map);

  if (navigator.geolocation && (!userMarker || !chosenMarker)) {
    navigator.geolocation.getCurrentPosition(
      (position) => {
        const { latitude, longitude } = position.coords;
        map.setView([latitude, longitude], 4);
        userMarker = L.marker([latitude, longitude]).addTo(map)
          .bindPopup("You are here.")
          .openPopup();

      },
      (err) => {
        console.warn("Geolocation error:", err);
      }
    );
  } else {
    const latlng = chosenMarker.getLatLng();
    const latitude = latlng.lat;
    const longitude = latlng.lng;
    map.setView([latitude, longitude], 3);
    chosenMarker = L.marker([latitude, longitude]).addTo(map)
          .bindPopup("You are here.")
          .openPopup();
  }

  map.on('click', function(e) {
    const { lat, lng } = e.latlng;
    const popupContent = `
      <div>
        <p>Do you want to choose this place?</p>
        <div style="display: flex; align-items: center; justify-content: space-between;">
            <button id="choose-place-btn">Yes</button>
            <button id="close-pop-up-btn">No</button>
        </div>
      </div>
    `;

    const popup = L.popup()
      .setLatLng(e.latlng)
      .setContent(popupContent)
      .openOn(map);

    // Delay to make sure that DOM exists
    setTimeout(() => {
      const btn_choose = document.getElementById("choose-place-btn");
      const btn_cancel = document.getElementById("close-pop-up-btn");
      if (btn_choose) {
        btn_choose.addEventListener("click", () => {
          if (chosenMarker) {
            chosenMarker.setLatLng([lat, lng]);
          } else {
            chosenMarker = L.marker([lat, lng]).addTo(map)
              .bindPopup("Chosen place").openPopup();
          }

          init_place_chosen(lat, lng)
        });
      }
      if (btn_cancel){
        btn_cancel.addEventListener("click", () =>{
            map.closePopup();
        });
      }
    }, 0);
  });
}