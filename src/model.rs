use rmcp::schemars;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WeatherResponse {
    pub latitude: f32,
    pub longitude: f32,   
    pub generationtime_ms: f32,   
    pub utc_offset_seconds: i32,   
    pub timezone: String,   
    pub timezone_abbreviation: String,   
    pub elevation: f32,   
    pub current_units: CurrentUnit, 
    pub current: Current, 
    pub hourly_units: HourlyUnit, 
    pub hourly: Hourly,
}

#[derive(Deserialize)]
pub struct CurrentUnit {
    pub time: String,
    pub interval: String,
    pub temperature_2m: String,
    pub wind_speed_10m: String,
}

#[derive(Deserialize)]
pub struct Current {
    pub time: String,
    pub interval: i32,
    pub temperature_2m: f32,
    pub wind_speed_10m: f32,
}

#[derive(Deserialize)]
pub struct HourlyUnit {
    pub time: String,
    pub temperature_2m: String,
    pub relative_humidity_2m: String,
    pub wind_speed_10m: String,
}

#[derive(Deserialize)]
pub struct Hourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f32>,
    pub relative_humidity_2m: Vec<i32>,
    pub wind_speed_10m: Vec<f32>,
}

#[derive(Deserialize, Debug, schemars::JsonSchema)]
pub struct PointRequest {
    pub latitude: f32,
    pub longitude: f32,
}