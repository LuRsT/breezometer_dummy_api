use actix_web::{web, Result};
use serde::Serialize;

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

#[derive(Serialize)]
struct Index {
    urls: Vec<String>,
}

async fn index(_info: web::Path<()>) -> Result<web::Json<Index>> {
    Ok(web::Json(Index {
        urls: [
            "http://localhost:8080/air-quality/v2/current-conditions".to_string(),
            "http://localhost:8080/weather/v1/current-conditions".to_string(),
        ]
        .to_vec(),
    }))
}

// https://api.breezometer.com/pollen/v2/forecast/daily?'

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route(
                "/air-quality/v2/current-conditions",
                web::get().to(air_quality),
            )
            .route("/weather/v1/current-conditions", web::get().to(weather))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
