mod weather;

use std::collections::HashMap;

use flowsnet_platform_sdk::logger;
use lambda_flows::{request_received, send_response};
use serde_json::Value;

use weather::get_weather;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    request_received(|headers, qry, body| handler(headers, qry, body)).await;
    Ok(())
}

async fn handler(headers: Vec<(String, String)>, qry: HashMap<String, Value>, _body: Vec<u8>) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    let resp = if let Some(city) = qry.get("city").map(|city| city.as_str().unwrap()) {
        get_weather(city).map(|data| {
            let weather = data.weather.first().map(|w| w.main.as_str()).unwrap_or_else(|| "");
            let temp = data.main.temp;
            let temp_min = data.main.temp_min;
            let temp_max = data.main.temp_max;
            let pressure = data.main.pressure;
            let humidity = data.main.humidity;
            format!("{city}, weather: {weather}, temperature: {temp}, min temperature: {temp_min}, max temperature: {temp_max}, pressure: {pressure}, humidity: {humidity}")
        })
    } else {
        Err("no city provided".to_string())
    };

    let return_headers = vec![(String::from("content-type"), String::from("text/html"))];
    match resp {
        Ok(data) => send_response(200, return_headers, data.as_bytes().to_vec()),
        Err(error) => send_response(400, return_headers, error.as_bytes().to_vec()),
    };
}
