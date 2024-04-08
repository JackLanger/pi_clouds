const BASE_URL: &str = "https://app-prod-ws.warnwetter.de/v30/stationOverviewExtended?stationIds=";
const MANNHEIM_ID: i16 = 10729;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Debug, Serialize, Deserialize)]
pub struct StationInfo {
    root: HashMap<String, ForecastRoot>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForecastRoot {
    forecast1: Forecast,
    forecast2: Forecast,
    days: Vec<Day>,
    #[serde(rename = "forecastStart")]
    forecast_start: Option<String>,
    warnings: Vec<WeatherWarning>,
    #[serde(rename = "threeHourSummaries")]
    three_hour_summaries: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct WeatherWarning {
    // todo:
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Forecast {
    #[serde(rename = "stationId")]
    station_id: String,
    #[serde(rename = "start")]
    start: i64,
    #[serde(rename = "timeStep")]
    time_step: i32,
    #[serde(rename = "temperature")]
    temperature: Vec<Number>,
    #[serde(rename = "windSpeed")]
    wind_speed: Option<String>,
    #[serde(rename = "windDirection")]
    wind_direction: Option<String>,
    #[serde(rename = "windGust")]
    wind_gust: Option<String>,
    #[serde(rename = "precipitationTotal")]
    precipitation_total: Vec<i32>,
    #[serde(rename = "sunshine")]
    sunshine: Vec<i32>,
    #[serde(rename = "dewPoint2m")]
    dew_point2m: Vec<i32>,
    #[serde(rename = "surfacePressure")]
    surface_pressure: Vec<i32>,
    #[serde(rename = "humidity")]
    humidity: Vec<i32>,
    #[serde(rename = "isDay")]
    is_day: Vec<bool>,
    #[serde(rename = "cloudCoverTotal")]
    cloud_cover_total: Vec<i32>,
    #[serde(rename = "temperatureStd")]
    temperature_std: Vec<i32>,
    #[serde(rename = "icon")]
    icon: Vec<i32>,
    #[serde(rename = "icon1h")]
    icon1h: Vec<i32>,
    #[serde(rename = "precipitationProbablity")]
    precipitation_probablity: Option<String>,
    #[serde(rename = "precipitationProbablityIndex")]
    precipitation_probablity_index: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Day {
    #[serde(rename = "stationId")]
    station_id: Option<String>,
    #[serde(rename = "dayDate")]
    day_date: String,
    #[serde(rename = "temperatureMin")]
    temperature_min: i32,
    #[serde(rename = "temperatureMax")]
    temperature_max: i32,
    #[serde(rename = "windSpeed")]
    wind_speed: i32,
    #[serde(rename = "windGust")]
    wind_gust: i32,
    #[serde(rename = "windDirection")]
    wind_direction: i32,
    #[serde(rename = "MoonPhase")]
    moon_phase: Option<i32>,
    precipitation: i32,
    sunshine: i32,
    sunrise: i64,
    sunset: i64,
    moonrise: i64,
    moonset: i64,
    icon: i32,
    icon1: Option<String>,
    icon2: Option<String>,
}

impl ForecastRoot {
    pub fn get_temperature(&self, i: usize) -> i32 {
        self.forecast1.temperature_std[i].clone()
    }

    pub fn get_current_wind_speed(&self) -> Option<String> {
        self.forecast1.wind_speed.clone()
    }

    pub fn get_wind_speeds(&self, len: usize) -> Vec<String> {
        self.forecast1
            .wind_speed
            .iter()
            .take(len)
            .map(|s| s.to_string())
            .collect()
    }

    pub fn get_current_temperature_icon(&self) -> i32 {
        self.forecast1.icon[0]
    }

    pub fn get_current_temperatures_icon(&self, len: usize) -> Vec<i32> {
        self.forecast1
            .icon
            .iter()
            .take(len)
            .map(|s| s.clone())
            .collect()
    }

    pub fn get_temperatures(&self, len: usize) -> Vec<Number> {
        self.forecast1
            .temperature
            .iter()
            .take(len)
            .cloned()
            .collect()
    }
}

pub mod controller;
#[cfg(test)]
mod data_converter_tests;
