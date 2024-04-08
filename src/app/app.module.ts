import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";

import { AppComponent } from "./app.component";
import { TimeComponent } from "./app.time";
import { CurrentTemperatureComponent } from "./app.weather";

@NgModule({
  declarations: [AppComponent,TimeComponent,CurrentTemperatureComponent],
  imports: [BrowserModule],
  providers: [],
  bootstrap: [AppComponent,TimeComponent,CurrentTemperatureComponent],
})
export class AppModule {}
