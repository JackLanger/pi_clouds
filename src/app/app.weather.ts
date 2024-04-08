
import { Component } from "@angular/core";
import { invoke } from "@tauri-apps/api/tauri";


@Component({
  selector: "current-temperature",
  templateUrl: "./app.currentWeather.html",
  styleUrls: ["./app.weather.css"],
})
export class CurrentTemperatureComponent {
    currentTemperature = "";
    imageSource = ""
    
    constructor () {
       
        const update = () => {
            const hour = new Date().getHours();
            
            // pick correct time index
                invoke<number>("fetch_temp", { hour }).then((result) => {
                this.currentTemperature = `${result} °C`;
            });

            // invoke<string>("fetchCurrentTemperatureIcon", { }).then((result) => {
            //     console.log(result);
            //         this.imageSource = `${result}`;
            // });
            setTimeout(update, 300000);
        };
        update();
    }
}


// @Component({
//   selector: "forecast3h",
//   templateUrl: "./app.forecast.html",
//   styleUrls: ["./app.weather.css"],
// })
// export class Forecast3hComponent {
   
//     constructor() {
//         const row = document.querySelector(".forecast-row");
        
//         const update = () => {
//             invoke("getForecastData", { }).then((data) => {

//                 row?.childNodes.forEach((node) => {
//                     node.remove();
//                 });
                
//                 row!.innerHTML += `<td class="card">
//                 <p class="forecast-time">${data}</p>
//                 <p class="forecast-temp">${data}</p>
//                 <img src="${data}" alt="" />
//     			</td>`;
//             })
//         };
            
//             setTimeout(update, 300000); // call every 5 minutes
//         update();
//     }
// }