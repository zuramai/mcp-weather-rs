use crate::model::WeatherResponse;

pub fn format_forecast(response: WeatherResponse) -> String {
    let mut result = String::new();
    if response.hourly.relative_humidity_2m.is_empty() {
        return "No forecast weather found.".into()
    }
    result.push_str(format!(
        "Current temperature: {}\nCurrent wind speed: {}\nElevation: {}\nHourly forecast:\n",
        response.current.temperature_2m,
        response.current.wind_speed_10m,
        response.elevation,
    ).as_str());
    for i in 0..response.hourly.relative_humidity_2m.len() {
        result.push_str(format!(
            "Date time: {}\nTemperature: {}\nHumidity: {}\nWind Speed: {}\n",
            response.hourly.time[i],
            response.hourly.temperature_2m[i],
            response.hourly.relative_humidity_2m[i],
            response.hourly.wind_speed_10m[i],
        ).as_str());
    }
    result
}