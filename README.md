# Simple Weather Flow

- This flow returns weather data by city name by calling API from OpenWeatherMap.
- This flow requires an `OpenWeatherMap API Key` environment variable setup in `flows.network` console and use `http_req_wasi` to send the request to OpenWeatherMap API and deserializes the response with `serde`

- You can search by input a city name in the query. See `Usage` below.

## Lambda Endpoint
- https://code.flows.network/lambda/fKSpCCGHvw

## Usage
```bash
curl 'https://code.flows.network/lambda/fKSpCCGHvw?city=new+york'

# new york, weather: Clouds, temperature: 20.97 °C, min temperature: 19.78 °C, max temperature: 21.98 °C, pressure: 1013, humidity: 88
```
