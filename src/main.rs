use actix_web::{web, Result};
use std::env;

use serde::Serialize;

#[derive(Serialize)]
struct PollenCount {
    pollenCountInfo: PollenCountInfo,
}

#[derive(Serialize)]
struct PollenCountInfo {
    date: String,
    index_id: String,
    index_display_name: String,
    types: PollenTypes,
    plants: PlantTypes,
}

#[derive(Serialize)]
struct PollenTypes {
    grass: PollenType,
    tree: PollenType,
    weed: PollenType,
}

#[derive(Serialize)]
struct PlantTypes {
    olive: PollenType,
    graminales: PollenType,
    ragweed: PollenType,
    birch: PollenType,
}

#[derive(Serialize)]
struct PollenType {
    display_name: String,
    in_season: bool,
    data_available: bool,
    index: PollenIndex,
}

#[derive(Serialize)]
struct PollenIndex {
    value: Option<u8>,
    category: Option<String>,
    color: Option<String>,
}

#[derive(Serialize)]
struct AirQuality {
    airQualityInfo: AirQualityInfo,
}

#[derive(Serialize)]
struct AirQualityInfo {
    baqi: Baqi,
}

#[derive(Serialize)]
struct Baqi {
    display_name: String,
    aqi: u8,
    aqi_display: String,
    color: String,
    category: String,
    dominant_pollutant: String,
}

#[derive(Serialize)]
struct CurrentConditions {
    feels_like_temperature: ValueUnits,
    temperature: ValueUnits,
    datetime: String,
    icon_code: u8,
    is_day_time: bool,
    weather_text: String,
    relative_humidity: u8,
    cloud_cover: u8,
    wind: Wind,
    precipitation: Precipitation,
    wind_gust: ValueUnits,
    pressure: ValueUnits,
    visibility: ValueUnits,
    dew_point: ValueUnits,
}

#[derive(Serialize)]
struct ValueUnits {
    value: f32,
    units: String,
}

#[derive(Serialize)]
struct Precipitation {
    precipitation_probability: u8,
    total_precipitation: ValueUnits,
}

#[derive(Serialize)]
struct Wind {
    speed: ValueUnits,
    direction: i32,
}

#[derive(Serialize)]
struct Index {
    urls: Vec<String>,
}

// https://api.breezometer.com/weather/v1/current-conditions'
async fn weather(_info: web::Path<()>) -> Result<web::Json<CurrentConditions>> {
    Ok(web::Json(CurrentConditions {
        datetime: "2020-02-19T11:00:00Z".to_string(),
        icon_code: 2,
        is_day_time: true,
        weather_text: "Cloudy with percipitation".to_string(),
        relative_humidity: 40,
        cloud_cover: 23,
        temperature: ValueUnits {
            value: 2.4,
            units: "C".to_string(),
        },
        feels_like_temperature: ValueUnits {
            value: 2.4,
            units: "C".to_string(),
        },
        precipitation: Precipitation {
            precipitation_probability: 26,
            total_precipitation: ValueUnits {
                value: 15.4,
                units: "mm".to_string(),
            },
        },
        wind: Wind {
            speed: ValueUnits {
                value: 18.756,
                units: "km/h".to_string(),
            },
            direction: 249,
        },
        wind_gust: ValueUnits {
            value: 30.6,
            units: "km/h".to_string(),
        },
        pressure: ValueUnits {
            value: 1012.68,
            units: "mb".to_string(),
        },
        visibility: ValueUnits {
            value: 9.0,
            units: "km".to_string(),
        },
        dew_point: ValueUnits {
            value: 6.14,
            units: "C".to_string(),
        },
    }))
}

// https://api.breezometer.com/air-quality/v2/current-conditions?'
async fn air_quality(_info: web::Path<()>) -> Result<web::Json<AirQuality>> {
    Ok(web::Json(AirQuality {
        airQualityInfo: AirQualityInfo {
            baqi: Baqi {
                display_name: "BreezoMeter AQI".to_string(),
                aqi: 86,
                aqi_display: "86".to_string(),
                color: "#3DB436".to_string(),
                category: "Excellent air quality".to_string(),
                dominant_pollutant: "o3".to_string(),
            },
        },
    }))
}

// https://api.breezometer.com/air-quality/v2/current-conditions?'
async fn pollen_count(_info: web::Path<()>) -> Result<web::Json<PollenCount>> {
    Ok(web::Json(PollenCount {
        pollenCountInfo: PollenCountInfo {
            date: "2020-03-05".to_string(),
            index_id: "bpi".to_string(),
            index_display_name: "BreezoMeter Pollen Index".to_string(),
            types: PollenTypes {
                grass: PollenType {
                    display_name: "Grass".to_string(),
                    in_season: false,
                    data_available: false,
                    index: PollenIndex {
                        value: None,
                        category: None,
                        color: None,
                    },
                },
                tree: PollenType {
                    display_name: "Tree".to_string(),
                    in_season: true,
                    data_available: true,
                    index: PollenIndex {
                        value: Some(0),
                        category: Some("None".to_string()),
                        color: None,
                    },
                },
                weed: PollenType {
                    display_name: "Weed".to_string(),
                    in_season: false,
                    data_available: false,
                    index: PollenIndex {
                        value: None,
                        category: None,
                        color: None,
                    },
                },
            },
            plants: PlantTypes {
                olive: PollenType {
                    display_name: "Olive".to_string(),
                    in_season: false,
                    data_available: false,
                    index: PollenIndex {
                        value: None,
                        category: None,
                        color: None,
                    },
                },
                graminales: PollenType {
                    display_name: "Graminales".to_string(),
                    in_season: false,
                    data_available: false,
                    index: PollenIndex {
                        value: None,
                        category: None,
                        color: None,
                    },
                },
                ragweed: PollenType {
                    display_name: "Ragweed".to_string(),
                    in_season: false,
                    data_available: false,
                    index: PollenIndex {
                        value: None,
                        category: None,
                        color: None,
                    },
                },
                birch: PollenType {
                    display_name: "Birch".to_string(),
                    in_season: true,
                    data_available: true,
                    index: PollenIndex {
                        value: Some(0),
                        category: Some("None".to_string()),
                        color: None,
                    },
                },
            },
        },
    }))
}

async fn index(_info: web::Path<()>) -> Result<web::Json<Index>> {
    Ok(web::Json(Index {
        urls: [
            "http://localhost:3000/air-quality/v2/current-conditions".to_string(),
            "http://localhost:3000/weather/v1/current-conditions".to_string(),
            "http://localhost:3000/pollen/v2/forecast/daily".to_string(),
        ]
        .to_vec(),
    }))
}

// https://api.breezometer.com/pollen/v2/forecast/daily?'

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route(
                "/air-quality/v2/current-conditions",
                web::get().to(air_quality),
            )
            .route("/weather/v1/current-conditions", web::get().to(weather))
            .route("pollen/v2/forecast/daily", web::get().to(pollen_count))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
