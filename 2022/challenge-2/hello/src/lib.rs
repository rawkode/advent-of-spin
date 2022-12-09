use anyhow::Result;
use http::Uri;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
use std::str::FromStr;

#[derive(serde::Serialize)]
struct OutboundPayload {
    value: String,
}

#[derive(serde::Deserialize)]
struct InboundPayload {
    message: String,
}

#[derive(serde::Serialize)]
struct ResponsePayload {
    message: String,
}

/// A simple Spin HTTP component.
#[http_component]
fn hello(req: Request) -> Result<Response> {
    let full_uri = req
        .headers()
        .get("spin-full-url")
        .unwrap()
        .to_str()
        .unwrap();

    let uri = Uri::from_str(full_uri).unwrap();
    let domain = uri.host().unwrap();

    let name = req.uri().path().split('/').last().unwrap();

    // No name
    if name.eq("hello") {
        let output_payload: ResponsePayload = ResponsePayload {
            message: "Hello, world!".to_string(),
        };

        return Ok(http::Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(Some(serde_json::to_string(&output_payload).unwrap().into()))?);
    }

    let outbound_payload = OutboundPayload {
        value: name.to_string(),
    };

    let res = spin_sdk::outbound_http::send_request(
        http::Request::builder()
            .method("POST")
            .uri(format!("https://{}/lowercase", &domain))
            .body(Some(
                serde_json::to_string(&outbound_payload).unwrap().into(),
            ))?,
    )?;

    if res.body().is_none() {
        return Ok(http::Response::builder()
            .status(500)
            .header("content-type", "application/json")
            .body(Some("No response body".into()))?);
    }

    let v = res.body().clone().unwrap().to_vec();

    let inbound_payload: InboundPayload = serde_json::from_slice(v.as_slice()).unwrap();

    let output_payload: ResponsePayload = ResponsePayload {
        message: format!("Hello, {}!", inbound_payload.message),
    };

    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Some(serde_json::to_string(&output_payload).unwrap().into()))?)
}
