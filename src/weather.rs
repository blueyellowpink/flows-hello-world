use http_req::request;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response {
    pub main: Main,
    pub weather: Vec<Weather>,
}

#[derive(Deserialize, Default)]
pub struct Weather {
    pub main: String,
}

#[derive(Deserialize)]
pub struct Main {
    pub temp: f64,
    pub pressure: f64,
    pub humidity: f64,
    pub temp_max: f64,
    pub temp_min: f64,
}

pub fn get_weather(city: &str) -> Result<Response, String> {
    let mut buffer = Vec::new();
    let api_key = std::env::var("API_KEY").unwrap();
    let query_str = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={city}&units=metric&appid={api_key}"
    );

    request::get(query_str, &mut buffer)
        .map_err(|e| e.to_string())
        .and_then(|_| {
            serde_json::from_slice::<Response>(&buffer).map_err(|_| {
                "Please check if you've typed the name of your city correctly".to_string()
            })
        })
}
