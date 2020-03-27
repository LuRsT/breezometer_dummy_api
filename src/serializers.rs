use serde::Serialize;

#[derive(Serialize)]
pub struct PollenCount {
    pub pollenCountInfo: PollenCountInfo,
}

#[derive(Serialize)]
pub struct PollenCountInfo {
    pub date: String,
    pub index_id: String,
    pub index_display_name: String,
    pub types: PollenTypes,
    pub plants: PlantTypes,
}

#[derive(Serialize)]
pub struct PollenTypes {
    pub grass: PollenType,
    pub tree: PollenType,
    pub weed: PollenType,
}

#[derive(Serialize)]
pub struct PlantTypes {
    pub olive: PollenType,
    pub graminales: PollenType,
    pub ragweed: PollenType,
    pub birch: PollenType,
}

#[derive(Serialize)]
pub struct PollenType {
    pub display_name: String,
    pub in_season: bool,
    pub data_available: bool,
    pub index: PollenIndex,
}

#[derive(Serialize)]
pub struct PollenIndex {
    pub value: Option<u8>,
    pub category: Option<String>,
    pub color: Option<String>,
}

#[derive(Serialize)]
pub enum AirQualityEnum {
    AirQuality(AirQuality),
    AirPollutants(AirPollutants),
}

#[derive(Serialize)]
pub struct AirQuality {
    pub airQualityInfo: AirQualityInfo,
}

#[derive(Serialize)]
pub struct AirQualityInfo {
    pub baqi: Baqi,
}

#[derive(Serialize)]
pub struct Baqi {
    pub display_name: String,
    pub aqi: u8,
    pub aqi_display: String,
    pub color: String,
    pub category: String,
    pub dominant_pollutant: String,
}

#[derive(Serialize)]
pub struct CurrentConditions {
    pub feels_like_temperature: ValueUnits,
    pub temperature: ValueUnits,
    pub datetime: String,
    pub icon_code: u8,
    pub is_day_time: bool,
    pub weather_text: String,
    pub relative_humidity: u8,
    pub cloud_cover: u8,
    pub wind: Wind,
    pub precipitation: Precipitation,
    pub wind_gust: ValueUnits,
    pub pressure: ValueUnits,
    pub visibility: ValueUnits,
    pub dew_point: ValueUnits,
}

#[derive(Serialize)]
pub struct ValueUnits {
    pub value: f32,
    pub units: String,
}

#[derive(Serialize)]
pub struct Precipitation {
    pub precipitation_probability: u8,
    pub total_precipitation: ValueUnits,
}

#[derive(Serialize)]
pub struct Wind {
    pub speed: ValueUnits,
    pub direction: i32,
}

#[derive(Serialize)]
pub struct Index {
    pub urls: Vec<String>,
}

// Air Pollutants

#[derive(Serialize)]
pub struct AirPollutants {
    pub airPollutantsInfo: AirPollutantsInfo,
}

#[derive(Serialize)]
pub struct AirPollutantsInfo {
    pub co: PollutantInfo,
    pub no2: PollutantInfo,
    pub o3: PollutantInfo,
    pub pm25: PollutantInfo,
    pub so2: PollutantInfo,
    pub pm10: PollutantInfo,
}

#[derive(Serialize)]
pub struct PollutantInfo {
    pub display_name: String,
    pub full_name: String,
    pub concentration: ValueUnits,
    pub sources_and_effects: PollutantSourcesAndEffects,
}

#[derive(Serialize)]
pub struct PollutantSourcesAndEffects {
    pub sources: String,
    pub effects: String,
}
