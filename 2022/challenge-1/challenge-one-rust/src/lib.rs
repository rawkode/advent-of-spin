use anyhow::Result;
use serde::Serialize;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[derive(Serialize)]
struct ResponsePayload {
    message: String,
}

/// A simple Spin HTTP component.
#[http_component]
fn challenge_one_rust(req: Request) -> Result<Response> {
    let payload = ResponsePayload {
        message: "Hello, world!".to_string(),
    };

    let json_response = match serde_json::to_string(&payload) {
        Ok(json_response) => json_response,
        Err(err) => {
            return Ok(http::Response::builder()
                .status(500)
                .body(Some(err.to_string().into()))?);
        }
    };

    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Some(json_response.into()))?)
}
