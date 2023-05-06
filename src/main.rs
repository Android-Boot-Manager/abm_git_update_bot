//! Lambda function for ABM.
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
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

use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use github_webhook_message_validator::validate as validate_gh;
use lambda_runtime::{run, service_fn, Error as LambdaError, LambdaEvent};
use lazy_static::lazy_static;
use log::{error, info};
use std::env;

lazy_static! {
    static ref WEBHOOK_SECRET: String = env::var("GITHUB_SECRET").unwrap_or_default();
    static ref TELEGRAM_TOKEN: String = env::var("TELEGRAM_TOKEN").unwrap_or_default();
    static ref TELEGRAM_GROUP_ID: String = env::var("TELEGRAM_GROUP_ID").unwrap_or_default();
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let func = service_fn(my_handler);
    run(func).await?;

    Ok(())
}

fn validate(sig: &[u8], msg: &[u8]) -> Option<ApiGatewayProxyResponse> {
    let secret: &[u8] = *&WEBHOOK_SECRET.as_bytes();
    let result = validate_gh(secret, sig, msg);

    if !result {
        error!("ERROR. GitHub signature invalid. Return 403.");
        return Some(ApiGatewayProxyResponse {
            status_code: 403,
            headers: Default::default(),
            multi_value_headers: Default::default(),
            body: Some(Body::from("AUTH_DENY")),
            is_base64_encoded: None,
        });
    }

    None
}

async fn my_handler(
    evt: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, LambdaError> {
    let ctx = evt.context;
    let sig = evt
        .payload
        .headers
        .get("X-Hub-Signature")
        .unwrap_or_default();

    info!("AWS Request ID: {}", ctx.request_id);

    if let Some(result) = validate(
        sig.as_bytes(),
        evt.payload.body.unwrap_or_default().as_bytes(),
    ) {
        return Ok(result);
    }

    let response = ApiGatewayProxyResponse {
        status_code: 201,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::from("Accepted.")),
        is_base64_encoded: None,
    };

    Ok(response)
}
