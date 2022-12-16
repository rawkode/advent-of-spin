use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

//{ d1: { lat: 0f, long: 0f }, d2: { lat: 0f, long: 0f }}

#[derive(Serialize, Deserialize)]
struct RequestPayload {
    d1: Point,
    d2: Point,
}

#[derive(Serialize, Deserialize)]
struct Point {
    lat: f64,
    long: f64,
}

#[derive(Serialize, Deserialize)]
struct ResponsePayload {
    distance: f64,
}

#[http_component]
fn advent_of_spin(req: Request) -> Result<Response> {
    let request_payload: RequestPayload =
        match serde_json::from_slice(req.body().clone().unwrap().to_vec().as_slice()) {
            Ok(request_payload) => request_payload,
            Err(_) => {
                return Ok(http::Response::builder()
                    .status(500)
                    .body(Some("Failed to parse payload".into()))?);
            }
        };

    let point_one = haversine::Location {
        latitude: request_payload.d1.lat,
        longitude: request_payload.d1.long,
    };

    let point_two = haversine::Location {
        latitude: request_payload.d2.lat,
        longitude: request_payload.d2.long,
    };

    let distance = haversine::distance(point_one, point_two, haversine::Units::Miles);

    let nautical_miles = unit_conversions::length::miles::to_nautical_miles(distance);

    let rounded_miles = ((nautical_miles * 10.0).round()) / 10.0;

    let response = ResponsePayload {
        distance: rounded_miles,
    };

    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Some(serde_json::to_string(&response).unwrap().into()))?)
}
