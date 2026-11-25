//! Shared helpers for the portfolio serverless functions.
//!
//! Provides JSON/error response builders and a best-effort webhook/email
//! notification sender used by the contact handler.

use serde::Serialize;
use vercel_runtime::{Response, Error};

pub type Body = vercel_runtime::ResponseBody;

/// Build a `200 OK`-ish JSON response with the given status code.
pub fn json_response<T: Serialize>(
    status: u16,
    value: &T,
) -> Result<Response<Body>, Error> {
    let bytes = serde_json::to_vec(value).map_err(|e| Error::from(e))?;
    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(Body::from(bytes))?)
}

/// Build a JSON error response.
pub fn error_response(
    status: u16,
    err: portfolio_shared::ApiError,
) -> Result<Response<Body>, Error> {
    json_response(status, &err)
}

/// Best-effort notification delivery.
///
/// If `CONTACT_WEBHOOK_URL` is set, POSTs the contact payload as JSON to that
/// webhook (e.g. a Resend/SMTP relay, Slack, or custom endpoint). Otherwise the
/// payload is logged to stderr so it surfaces in Vercel function logs.
///
/// All delivery failures are swallowed — a contact submission must never fail
/// because the notification channel is down.
pub async fn send_notification(
    payload: &portfolio_shared::ContactPayload,
) -> Result<(), Error> {
    let url = match std::env::var("CONTACT_WEBHOOK_URL") {
        Ok(u) => u,
        Err(_) => {
            eprintln!("[contact] no CONTACT_WEBHOOK_URL set; logging payload: {payload:?}");
            return Ok(());
        }
    };

    let _ = reqwest::Client::new()
        .post(&url)
        .json(payload)
        .send()
        .await
        .map_err(|e| eprintln!("[contact] webhook send failed: {e}"));

    Ok(())
}
