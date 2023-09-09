//! Lambda function for ABM.
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::all,
    clippy::pedantic,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

mod models;
use crate::models::*;

use aws_lambda_events::http::status::StatusCode;
use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use github_webhook_message_validator::validate as validate_gh;
use hex::decode;
use lambda_http::http::HeaderValue;
use lambda_runtime::{run, service_fn, Error as LambdaError, LambdaEvent};
use lazy_static::lazy_static;
use log::{error, info};
use std::env;

lazy_static! {
    static ref WEBHOOK_SECRET: String = env::var("WEBHOOK_GH_SECRET").unwrap_or_default();
    static ref TELEGRAM_TOKEN: String = env::var("TELEGRAM_TOKEN").unwrap_or_default();
    static ref TELEGRAM_GROUP_ID: String = env::var("TELEGRAM_GROUP_ID").unwrap_or_default();
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    env_logger::init();

    run(service_fn(webhook_handler)).await?;

    Ok(())
}

fn validate(sig: &str, msg: &str) -> Option<ApiGatewayProxyResponse> {
    let hex_sig =
        decode(&sig.replace("sha1=", "")).expect("Error decoding X-Hub-Signature into Hex.");

    if !validate_gh(&*WEBHOOK_SECRET.as_ref(), &hex_sig, msg.as_bytes()) {
        error!("ERROR. GitHub signature invalid. Return 403.");
        return Some(ApiGatewayProxyResponse {
            status_code: StatusCode::FORBIDDEN.as_u16() as i64,
            headers: Default::default(),
            multi_value_headers: Default::default(),
            body: Some(Body::Empty),
            is_base64_encoded: false,
        });
    }

    None
}

fn process_webhook(payload: &str) -> Option<GithubHook> {
    let decoded: GithubHook = serde_json::from_str::<GithubHook>(&payload).unwrap();
    let decoded = decoded.clone();

    if decoded.repository.name.contains("planet_") {
        return None;
    } else if decoded.repository.name.contains("bot") {
        return None;
    } else if decoded
        .head_commit
        .message
        .clone()
        .unwrap()
        .contains("Update submodule")
    {
        return None;
    }

    Some(decoded)
}

async fn webhook_handler(
    evt: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, LambdaError> {
    let ctx = evt.context;
    let empty_header_value = HeaderValue::from_str("")?;
    let sig = evt
        .payload
        .headers
        .get("X-Hub-Signature")
        .unwrap_or(&empty_header_value);
    info!("AWS Request ID: {}", ctx.request_id);

    if let Some(result) = validate(
        &sig.to_str().unwrap_or_default(),
        &evt.payload.body.clone().unwrap_or_default().as_str(),
    ) {
        return Ok(result);
    }

    if let None = process_webhook(&evt.payload.body.clone().unwrap_or_default()) {
        return Ok(ApiGatewayProxyResponse {
            status_code: StatusCode::OK.as_u16() as i64,
            headers: Default::default(),
            multi_value_headers: Default::default(),
            body: None,
            is_base64_encoded: false,
        });
    }

    // The event is worth reporting on.

    let response = ApiGatewayProxyResponse {
        status_code: StatusCode::ACCEPTED.as_u16() as i64,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Empty),
        is_base64_encoded: false,
    };

    Ok(response)
}
