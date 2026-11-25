//! `GET /api/portfolio` — returns the full structured portfolio document
//! (sections 00–08, projects, experience, certifications, build log).

use vercel_runtime::{run, service_fn, Request, Response, Error};
use crate::json_response;
use portfolio_shared::data::portfolio_data;
use portfolio_shared::PortfolioData;
use serde_json::json;

type Body = vercel_runtime::ResponseBody;

async fn portfolio_handler(_req: Request) -> Result<Response<Body>, Error> {
    let data: PortfolioData = portfolio_data();
    // Envelope: { ok: true, data: <PortfolioData> }
    let body = json!({ "ok": true, "data": data });
    json_response(200, &body)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(portfolio_handler)).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portfolio_data_round_trip() {
        let data = portfolio_data();
        assert!(!data.name.is_empty());
        assert!(!data.projects.is_empty());
        assert!(!data.build_log.is_empty());
        // Serialize/deserialize must be lossless.
        let json = serde_json::to_string(&data).unwrap();
        let back: PortfolioData = serde_json::from_str(&json).unwrap();
        assert_eq!(data.projects.len(), back.projects.len());
    }

    #[test]
    fn section_ids_cover_00_to_08() {
        use portfolio_shared::SectionId;
        let ids = SectionId::all();
        assert_eq!(ids.len(), 9);
        let anchors: Vec<_> = ids.iter().map(|s| s.anchor()).collect();
        assert_eq!(
            anchors,
            vec![
                "home", "about", "experience", "stack", "skills",
                "projects", "blog", "contact", "elsewhere",
            ]
        );
    }
}
