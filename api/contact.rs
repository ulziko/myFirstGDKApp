//! `POST /api/contact` — validates a `ContactPayload`, enforces anti-spam
//! checks, sends a best-effort webhook/email notification, and returns a
//! structured JSON response.

use vercel_runtime::{run, service_fn, Request, Response, Error};
use hyper::body::to_bytes;
use crate::{error_response, json_response, send_notification};
use portfolio_shared::{ApiError, ContactPayload, ContactResult};

type Body = vercel_runtime::ResponseBody;

const MAX_MESSAGE_LEN: usize = 5000;

async fn contact_handler(req: Request) -> Result<Response<Body>, Error> {
    // Read the request body.
    let bytes = to_bytes(req.into_body())
        .await
        .map_err(|e| Error::from(format!("failed to read body: {e}")))?;

    if bytes.is_empty() {
        return error_response(400, ApiError::bad_request("body is required"));
    }

    let payload: ContactPayload = serde_json::from_slice(&bytes).map_err(|e| {
        error_response(400, ApiError::bad_request(format!("invalid json: {e}")))
    })?;

    // --- validation / anti-spam checks ---
    validate(&payload).map_err(|e| error_response(400, e))?;

    // Best-effort notification (webhook/email). Never blocks the response.
    let _ = send_notification(&payload).await;

    let result = ContactResult {
        accepted: true,
        message: "Thanks — your message was received.".into(),
        id: Some(format!("contact-{}", uuid_v4ish())),
    };

    json_response(200, &portfolio_shared::ApiResponse::ok(result))
}

/// Field-level validation and basic anti-spam heuristics.
/// Returns `Ok(())` when valid, or an error response when invalid.
fn validate(payload: &ContactPayload) -> Result<(), Error> {
    if payload.name.trim().is_empty() {
        return error_response(400, ApiError::bad_request("name is required"));
    }
    if payload.name.len() > 200 {
        return error_response(400, ApiError::bad_request("name is too long"));
    }
    if payload.email.trim().is_empty() {
        return error_response(400, ApiError::bad_request("email is required"));
    }
    if !is_valid_email(&payload.email) {
        return error_response(400, ApiError::bad_request("a valid email is required"));
    }
    if payload.message.trim().is_empty() {
        return error_response(400, ApiError::bad_request("message is required"));
    }
    if payload.message.len() > MAX_MESSAGE_LEN {
        return error_response(400, ApiError::bad_request("message is too long"));
    }
    // Crude anti-spam: reject obvious link-dumping.
    if link_density(&payload.message) > 0.5 {
        return error_response(400, ApiError::bad_request("message contains too many links"));
    }
    Ok(())
}

fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    !parts[0].is_empty()
        && parts[1].contains('.')
        && !parts[1].starts_with('.')
        && !parts[1].ends_with('.')
}

fn link_density(text: &str) -> f64 {
    if text.is_empty() {
        return 0.0;
    }
    let links = text.matches("http://").count() + text.matches("https://").count();
    links as f64 / text.len() as f64
}

/// Tiny non-cryptographic id generator (no extra dependency).
fn uuid_v4ish() -> String {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("{now:x}")
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(contact_handler)).await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_payload() -> ContactPayload {
        ContactPayload {
            name: "Ada Lovelace".into(),
            email: "ada@example.com".into(),
            message: "Hello, I would like to get in touch.".into(),
        }
    }

    #[test]
    fn valid_payload_passes() {
        assert!(validate(&valid_payload()).is_ok());
    }

    #[test]
    fn empty_name_rejected() {
        let mut p = valid_payload();
        p.name = "   ".into();
        assert!(validate(&p).is_err());
    }

    #[test]
    fn invalid_email_rejected() {
        let mut p = valid_payload();
        p.email = "not-an-email".into();
        assert!(validate(&p).is_err());
    }

    #[test]
    fn empty_message_rejected() {
        let mut p = valid_payload();
        p.message = "".into();
        assert!(validate(&p).is_err());
    }

    #[test]
    fn message_too_long_rejected() {
        let mut p = valid_payload();
        p.message = "x".repeat(MAX_MESSAGE_LEN + 1);
        assert!(validate(&p).is_err());
    }

    #[test]
    fn too_many_links_rejected() {
        let mut p = valid_payload();
        p.message = "https://a.com https://b.com https://c.com spam".into();
        assert!(validate(&p).is_err());
    }

    #[test]
    fn contact_result_serializes() {
        let result = ContactResult {
            accepted: true,
            message: "ok".into(),
            id: Some("contact-1".into()),
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("accepted"));
        assert!(json.contains("contact-1"));
    }
}
