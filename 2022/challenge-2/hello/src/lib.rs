use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

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
    println!("HELP: {:?}", req.uri());

    return Ok(http::Response::builder()
        .status(500)
        .body(Some(format!("{:?}", req.uri()).into()))?);

    let domain = "http://google.com";
    // req.uri().host().unwrap();
    let name = req.uri().path().split('/').last();

    if name.is_none() {
        let output_payload: ResponsePayload = ResponsePayload {
            message: "Hello, world!".to_string(),
        };

        return Ok(http::Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(Some(serde_json::to_string(&output_payload).unwrap().into()))?);
    }

    let name = name.unwrap();

    let outbound_payload = OutboundPayload {
        value: name.to_string(),
    };

    let res = spin_sdk::outbound_http::send_request(
        http::Request::builder()
            .method("POST")
            .uri(format!("{}/lowercase", &domain))
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
