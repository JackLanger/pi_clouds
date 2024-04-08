
import { Component } from "@angular/core";


@Component({
  selector: "current-time",
  templateUrl: "./app.time.html",
  styleUrls: ["./app.time.css"],
})
export class TimeComponent {
    currentTime = "hh:mm:ss";
    
    constructor() {
        const update = () => {
            this.currentTime = new Date().toLocaleTimeString();
            setTimeout(update, 1000);
        };
        update();
    } 
}