// src-tauri/src/model/weather.rs

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Forecast {
    pub date: String,
    pub high: String,
    pub low: String,
    pub ymd: String,
    pub week: String,
    pub sunrise: String,
    pub sunset: String,
    pub aqi: f64,
    pub fx: String,
    pub fl: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub notice: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CityInfo {
    pub city: String,
    pub citykey: String,
    pub parent: String,
    #[serde(rename = "updateTime")]
    pub update_time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherDataInner {
    pub shidu: String,
    pub pm25: Option<f64>,
    pub pm10: Option<f64>,
    pub quality: String,
    pub wendu: String,
    pub ganmao: String,
    pub forecast: Vec<Forecast>,
    #[serde(rename = "yesterday")]
    pub yesterday: YesterdayForecast,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YesterdayForecast {
    pub date: String,
    pub high: String,
    pub low: String,
    pub ymd: String,
    pub week: String,
    pub sunrise: String,
    pub sunset: String,
    pub aqi: f64,
    pub fx: String,
    pub fl: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub notice: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherResponse {
    pub status: i32,
    pub message: String,
    pub date: String,
    pub time: String,
    #[serde(rename = "cityInfo")]
    pub city_info: CityInfo,
    #[serde(rename = "data")]
    pub data: WeatherDataInner,
}