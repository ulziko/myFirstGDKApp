//! `GET /api/health` — lightweight health check endpoint.

use vercel_runtime::{run, service_fn, Request, Response, Error};
use crate::json_response;
use serde_json::json;

type Body = vercel_runtime::ResponseBody;

async fn health_handler(_req: Request) -> Result<Response<Body>, Error> {
    let body = json!({
        "status": "ok",
        "service": "portfolio-api",
        "target": "vercel-serverless",
    });
    json_response(200, &body)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(health_handler)).await
}
