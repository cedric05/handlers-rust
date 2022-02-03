use crate::{Context, Response};
use hyper::StatusCode;
use local_ip_address::local_ip;
use serde::Deserialize;

pub async fn test_handler(ctx: Context) -> String {
    format!("test called, state_thing was: {}", ctx.state.state_thing)
}

#[derive(Deserialize)]
struct IpRequest {
    from_ip: String,
    active: bool,
}

#[derive(Deserialize)]
struct Record {
    name: String,
}

pub async fn verify_handler(mut ctx: Context) -> Response {
    let body: Record = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            return hyper::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("could not parse JSON: {}", e).into())
                .unwrap()
        }
    };

    Response::new(format!("username is {}", body.name).into())
}

pub async fn ping_handler(mut ctx: Context) -> Response {
    let body: IpRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            return hyper::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("could not parse JSON: {}", e).into())
                .unwrap()
        }
    };
    println!(
        "recieved a request from {} and is active {}",
        body.from_ip, body.active
    );

    Response::new(format!("pong from {}", local_ip().unwrap()).into())
}
