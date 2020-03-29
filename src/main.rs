use actix_cors::Cors;
use actix_web::{web, Result};
use std::env;
mod serializers;
use serde::Deserialize;

pub use crate::serializers::{
    AirPollutantsInfo, AirQualityEnum, AirQualityInfo, Baqi, Index, PlantTypes, PollenCount,
    PollenCountInfo, PollenIndex, PollenType, PollenTypes, PollutantInfo,
    PollutantSourcesAndEffects, Precipitation, ValueUnits, Weather, WeatherInfo, Wind,
};

#[derive(Deserialize)]
pub struct QueryRequest {
    lat: Option<String>,
    lon: Option<String>,
    key: Option<String>,
    features: Option<String>,
}

fn build_temperature_value() -> ValueUnits {
    let data = ValueUnits {
        value: 2.4,
        units: "C".to_string(),
    };
    return data;
}

fn build_speed_value() -> ValueUnits {
    let data = ValueUnits {
        value: 30.6,
        units: "km/h".to_string(),
    };
    return data;
}

fn build_pollen_type(display_name: String) -> PollenType {
    let data = PollenType {
        display_name: display_name,
        in_season: false,
        data_available: false,
        index: PollenIndex {
            value: None,
            category: None,
            color: None,
        },
    };
    return data;
}

async fn weather(_info: web::Path<()>) -> Result<web::Json<WeatherInfo>> {
    let data = WeatherInfo {
        weatherInfo: Weather {
            datetime: "2020-02-19T11:00:00Z".to_string(),
            icon_code: 2,
            is_day_time: true,
            weather_text: "Cloudy with percipitation".to_string(),
            relative_humidity: 40,
            cloud_cover: 23,
            temperature: build_temperature_value(),
            feels_like_temperature: build_temperature_value(),
            precipitation: Precipitation {
                precipitation_probability: 26,
                total_precipitation: ValueUnits {
                    value: 15.4,
                    units: "mm".to_string(),
                },
            },
            wind: Wind {
                speed: build_speed_value(),
                direction: 249,
            },
            wind_gust: build_speed_value(),
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
        },
    };
    Ok(web::Json(data))
}

// https://api.breezometer.com/air-quality/v2/current-conditions'
async fn air_quality(
    web::Query(info): web::Query<QueryRequest>,
) -> Result<web::Json<AirQualityEnum>> {
    if info.features != None {
        // Simulate: features=health_recommendations, pollutants_concentrations, sources_and_effects
        println!("Got features");
        let data = AirQualityEnum::airPollutants( AirPollutantsInfo{
                co: PollutantInfo {
                    display_name: "CO".to_string(),
                    full_name: "Carbon monoxide".to_string(),
                    concentration: ValueUnits {
                        value: 315.04,
                        units: "ppb".to_string(),
                    },
                    sources_and_effects: PollutantSourcesAndEffects{
                        sources: "Typically originates from incomplete combustion of carbon fuels, such as that which occurs in car engines and power plants.".to_string(),
                        effects: "When inhaled, carbon monoxide can prevent the blood from carrying oxygen. Exposure may cause dizziness, nausea and headaches. Exposure to extreme concentrations can lead to loss of consciousness.".to_string(),
                    }
                },
                no2: PollutantInfo{
                    display_name: "NO2".to_string(),
                    full_name: "Nitrogen dioxide".to_string(),
                    concentration: ValueUnits{
                        value: 7.67,
                        units: "ppb".to_string(),
                    },
                    sources_and_effects: PollutantSourcesAndEffects{
                        sources: "Main sources are fuel burning processes, such as those used in industry and transportation.".to_string(),
                        effects: "Exposure may cause increased bronchial reactivity in patients with asthma, lung function decline in patients with COPD, and increased risk of respiratory infections, especially in young children.".to_string(),
                    }
                },
                o3: PollutantInfo{
                    display_name: "O3".to_string(),
                    full_name: "Ozone".to_string(),
                    concentration: ValueUnits{
                        value: 30.8,
                        units: "ppb".to_string()
                    },
                    sources_and_effects: PollutantSourcesAndEffects{
                        sources: "Ozone is created in a chemical reaction between atmospheric oxygen, nitrogen oxides, carbon monoxide and organic compounds, in the presence of sunlight.".to_string(),
                        effects: "Ozone can irritate the airways and cause coughing, a burning sensation, wheezing and shortness of breath. Additionally, ozone is one of the major components of photochemical smog.".to_string()
                    }
                },
                pm10: PollutantInfo{
                    display_name: "PM10".to_string(),
                    full_name: "Inhalable particulate matter (<10µm)".to_string(),
                    concentration: ValueUnits{
                        value: 11.78,
                        units: "ug/m3".to_string(),
                    },
                    sources_and_effects: PollutantSourcesAndEffects{
                        sources: "Main sources are combustion processes (e.g. indoor heating, wildfires), mechanical processes (e.g. construction, mineral dust, agriculture) and biological particles (e.g. pollen, bacteria, mold).".to_string(),
                        effects: "Inhalable particles can penetrate into the lungs. Short term exposure can cause irritation of the airways, coughing, and aggravation of heart and lung diseases, expressed as difficulty breathing, heart attacks and even premature death.".to_string(),
                    }
                },
                pm25: PollutantInfo{
                    display_name: "PM2.5".to_string(),
                    full_name: "Fine particulate matter (<2.5µm)".to_string(),
                    concentration: ValueUnits{
                        value: 6.75,
                        units: "ug/m3".to_string(),
                    },
                    sources_and_effects: PollutantSourcesAndEffects{
                        sources: "Main sources are combustion processes (e.g. power plants, indoor heating, car exhausts, wildfires), mechanical processes (e.g. construction, mineral dust) and biological particles (e.g. bacteria, viruses).".to_string(),
                        effects: "Fine particles can penetrate into the lungs and bloodstream. Short term exposure can cause irritation of the airways, coughing and aggravation of heart and lung diseases, expressed as difficulty breathing, heart attacks and even premature death.".to_string()
                    }
                },
                so2: PollutantInfo{
                    display_name: "SO2".to_string(),
                    full_name: "Sulfur dioxide".to_string(),
                    concentration: ValueUnits{
                        value: 0.99,
                        units: "ppb".to_string()
                    },
                    sources_and_effects: PollutantSourcesAndEffects{
                        sources: "Main sources are burning processes of sulfur-containing fuel in industry, transportation and power plants.".to_string(),
                        effects: "Exposure causes irritation of the respiratory tract, coughing and generates local inflammatory reactions. These in turn, may cause aggravation of lung diseases, even with short term exposure.".to_string(),
                    }
                }
        });
        Ok(web::Json(data))
    } else {
        let data = AirQualityEnum::airQuality(AirQualityInfo {
            baqi: Baqi {
                display_name: "BreezoMeter AQI".to_string(),
                aqi: 86,
                aqi_display: "86".to_string(),
                color: "#3DB436".to_string(),
                category: "Excellent air quality".to_string(),
                dominant_pollutant: "o3".to_string(),
            },
        });
        Ok(web::Json(data))
    }
}

// https://api.breezometer.com/air-quality/v2/current-conditions?'
async fn pollen_count(_info: web::Path<()>) -> Result<web::Json<PollenCount>> {
    let data = PollenCount {
        pollenCountInfo: PollenCountInfo {
            date: "2020-03-05".to_string(),
            index_id: "bpi".to_string(),
            index_display_name: "BreezoMeter Pollen Index".to_string(),
            types: PollenTypes {
                grass: build_pollen_type("Grass".to_string()),
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
    };
    Ok(web::Json(data))
}

async fn index(_info: web::Path<()>) -> Result<web::Json<Index>> {
    let data = Index {
        urls: [
            "http://localhost:3000/air-quality/v2/current-conditions".to_string(),
            "http://localhost:3000/weather/v1/current-conditions".to_string(),
            "http://localhost:3000/pollen/v2/forecast/daily".to_string(),
        ]
        .to_vec(),
    };
    Ok(web::Json(data))
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
            .wrap(Cors::new().supports_credentials().finish())
            .service(web::resource("/").route(web::get().to(index)))
            .service(
                web::resource("/air-quality/v2/current-conditions")
                    .route(web::get().to(air_quality)),
            )
            .service(web::resource("/weather/v1/current-conditions").route(web::get().to(weather)))
            .service(web::resource("/pollen/v2/forecast/daily").route(web::get().to(pollen_count)))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
