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

use std::env;

use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use aws_lambda_events::http::status::StatusCode;
use aws_lambda_events::http::HeaderMap;
use github_webhook_message_validator::validate as validate_gh;
use hex::decode;
use lambda_runtime::{run, service_fn, Error as LambdaError, LambdaEvent};
use lazy_static::lazy_static;
use log::{error, info};

use crate::models::GithubHook;

mod models;

lazy_static! {
    static ref WEBHOOK_SECRET: String =
        env::var("WEBHOOK_GH_SECRET").expect("Unable to get GH webhook secret from environment!");
    static ref TELEGRAM_TOKEN: String =
        env::var("TELEGRAM_TOKEN").expect("Unable to get Telegram API token from environment!");
    static ref TELEGRAM_GROUP_ID: String = env::var("TELEGRAM_GROUP_ID")
        .expect("Unable to get group ID for ABM Telegram group from environment!");
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    env_logger::init();

    run(service_fn(webhook_handler)).await?;

    Ok(())
}

fn validate(sig: &str, msg: &str) -> Option<ApiGatewayProxyResponse> {
    let hex_sig =
        decode(sig.replace("sha1=", "")).expect("Error decoding X-Hub-Signature into Hex.");

    if !validate_gh(WEBHOOK_SECRET.as_ref(), &hex_sig, msg.as_bytes()) {
        error!("ERROR. GitHub signature invalid. Return 403.");
        return Some(ApiGatewayProxyResponse {
            status_code: i64::from(StatusCode::FORBIDDEN.as_u16()),
            headers: HeaderMap::default(),
            multi_value_headers: HeaderMap::default(),
            body: Some(Body::Empty),
            is_base64_encoded: false,
        });
    }

    None
}

fn process_webhook(payload: &str) -> Option<GithubHook> {
    let decoded: GithubHook = serde_json::from_str::<GithubHook>(payload)
        .expect("Unable to decode GitHub webhook payload!");

    if decoded.repository.full_name.contains("planet_")
        || decoded.repository.full_name.contains("abm_git_update_bot")
        || decoded
            .head_commit
            .message
            .as_ref()
            .expect("Unable to get commit message.")
            .contains("Update submodule")
    {
        return None;
    }

    Some(decoded)
}

async fn webhook_handler(
    evt: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, LambdaError> {
    let sig = evt
        .payload
        .headers
        .get("X-Hub-Signature")
        .expect("No GitHub signature found in headers.")
        .to_str()
        .expect("[GH Signature]: Unable to convert to &str.");
    let body = evt.payload.body.expect("No body passed to request.");

    if let Some(resp) = validate(sig, &body) {
        return Ok(resp);
    }

    info!("Webhook validated, signature confirmed OK.");

    if process_webhook(&body).is_none() {
        info!("Can't act on this event - it is suppressed.");
        return Ok(ApiGatewayProxyResponse {
            status_code: i64::from(StatusCode::OK.as_u16()),
            headers: HeaderMap::default(),
            multi_value_headers: HeaderMap::default(),
            body: None,
            is_base64_encoded: false,
        });
    }

    // The event is worth reporting on.
    // TODO: Pass on notification to Telegram.

    let response = ApiGatewayProxyResponse {
        status_code: i64::from(StatusCode::ACCEPTED.as_u16()),
        headers: HeaderMap::default(),
        multi_value_headers: HeaderMap::default(),
        body: Some(Body::Empty),
        is_base64_encoded: false,
    };

    Ok(response)
}
