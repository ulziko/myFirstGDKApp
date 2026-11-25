//! Structured placeholder portfolio data.
//!
//! Replace the placeholder values (clearly marked below) with your own details.
//! This is the single source of truth for `GET /api/portfolio` and is also
//! consumable by the WASM frontend via `portfolio-shared`.

use crate::types::*;

pub const BUILD_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_ID: &str = env!("VERCEL_GIT_COMMIT_SHA").unwrap_or("dev-local");

/// Build the complete portfolio document.
///
/// All personal details are placeholders. Edit `name`, `title`, `location`,
// `bio`, and the entry lists to reflect your real background.
pub fn portfolio_data() -> PortfolioData {
    PortfolioData {
        version: BUILD_VERSION.to_string(),
        build: BUILD_ID.to_string(),
        name: "Your Name".to_string(), // <-- REPLACE
        title: "Systems & Agentic Architect | Rust Developer".to_string(), // <-- REPLACE
        location: "City, Country · Open to remote".to_string(), // <-- REPLACE
        status: "Available for work".to_string(), // <-- REPLACE
        bio:
            "I build reliable systems and agentic architectures, primarily in Rust.\n\
            This portfolio pairs a Bevy 3D WebAssembly frontend with a Rust\n\
            serverless backend on Vercel. Sections 00–08 are navigable from the\n\
            3D scene, the keyboard, or the HTML/CSS windowed HUD."
            .to_string(),
        stack: Stack {
            languages: vec![
                "Rust".into(),
                "TypeScript".into(),
                "Python".into(),
                "C++".into(),
                "SQL".into(),
            ],
            frameworks: vec![
                "Bevy".into(),
                "Axum".into(),
                "tokio".into(),
                "serde".into(),
                "Next.js".into(),
            ],
            cloud_hosting: vec!["Vercel".into(), "AWS".into(), "fly.io".into()],
            tooling: vec![
                "cargo".into(),
                "wasm-pack / wasm-opt".into(),
                "Docker".into(),
                "GitHub Actions".into(),
                "pgvector".into(),
            ],
            platforms: vec!["Linux".into(), "WebAssembly".into(), "macOS".into()],
        },
        projects: vec![
            ProjectEntry {
                id: "bevy-portfolio".into(),
                title: "Bevy 3D Developer Portfolio".into(),
                teaser: "This site — a Bevy WASM 3D frontend + Rust serverless API.".into(),
                description:
                    "A personal developer portfolio with a 3D Bevy scene compiled to\n\
                    WebAssembly (`wasm32-unknown-unknown`) and a Rust backend deployed\n\
                    on Vercel via Git. Sections 00–08 are navigable from the 3D scene,\n\
                    keyboard shortcuts, or an HTML/CSS windowed HUD."
                    .into(),
                tech: vec!["Bevy".into(), "Rust".into(), "WebAssembly".into(), "vercel_runtime".into()],
                platform: "Vercel".into(),
                demo_url: Some("https://your-domain.vercel.app".into()), // <-- REPLACE
                repo_url: Some("https://github.com/you/portfolio".into()), // <-- REPLACE
                focus_section: Some(SectionId::Projects),
            },
            ProjectEntry {
                id: "agentic-orchestrator".into(),
                title: "Agentic Orchestration Framework".into(),
                teaser: "Rust-based agent workflow runtime with tool use.".into(),
                description:
                    "An agentic architecture runtime written in Rust, supporting\n\
                    LLM tool use, structured outputs, and durable execution."
                    .into(),
                tech: vec!["Rust".into(), "tokio".into(), "serde_json".into()],
                platform: "fly.io".into(),
                demo_url: None,
                repo_url: Some("https://github.com/you/agentic-orchestrator".into()), // <-- REPLACE
                focus_section: Some(SectionId::Stack),
            },
            ProjectEntry {
                id: "data-pipeline".into(),
                title: "High-Throughput Data Pipeline".into(),
                teaser: "Streaming ETL pipeline with observability.".into(),
                description:
                    "A streaming data pipeline built in Rust with:\n\
                    - Async I/O via tokio\n\
                    - Schema validation via serde\n\
                    - Structured logging and metrics\n\
                    - Deployed on AWS with zero-downtime deploys."
                    .into(),
                tech: vec!["Rust".into(), "tokio".into(), "AWS".into(), "Docker".into()],
                platform: "AWS".into(),
                demo_url: None,
                repo_url: Some("https://github.com/you/data-pipeline".into()), // <-- REPLACE
                focus_section: None,
            },
        ],
        experience: vec![
            ExperienceEntry {
                company: "Placeholder Company".into(), // <-- REPLACE
                role: "Senior Rust Engineer".into(),     // <-- REPLACE
                start: "2024-01".into(),
                end: None,
                current: true,
                highlights: vec![
                    "Designed and shipped agentic architecture components in Rust.".into(),
                    "Reduced p99 latency by ~40% through async I/O tuning.".into(),
                ],
                tech: vec!["Rust".into(), "tokio".into(), "AWS".into()],
            },
            ExperienceEntry {
                company: "Previous Placeholder".into(), // <-- REPLACE
                role: "Software Engineer".into(),        // <-- REPLACE
                start: "2022-06".into(),
                end: Some("2023-12".into()),
                current: false,
                highlights: vec!["Built internal tooling used by 50+ engineers.".into()],
                tech: vec!["TypeScript".into(), "Python".into(), "PostgreSQL".into()],
            },
        ],
        certifications: vec![
            CertificationEntry {
                name: "Placeholder Certification".into(), // <-- REPLACE
                issuer: "Placeholder Issuer".into(),      // <-- REPLACE
                date: "2024".into(),
                url: None,
            },
        ],
        build_log: vec![
            BuildLogEntry {
                id: "build-001".into(),
                timestamp: "2024-01-15T09:00:00Z".into(),
                status: "deployed".into(),
                message: "Initial production deploy (v0.1.0)".into(),
            },
            BuildLogEntry {
                id: "build-002".into(),
                timestamp: "2024-01-20T14:30:00Z".into(),
                status: "ok".into(),
                message: "Wasm bundle optimized with wasm-opt".into(),
            },
        ],
        elsewhere: vec![
            ElsewhereEntry {
                name: "Placeholder Agency".into(), // <-- REPLACE
                description: "Affiliated studio / venture.".into(),
                url: "https://example.com".into(), // <-- REPLACE
                logo_hint: None,
            },
        ],
        contact: ContactInfo {
            email: "you@example.com".into(), // <-- REPLACE
            location: "City, Country".into(), // <-- REPLACE
            links: vec![
                ContactLink {
                    label: "GitHub".into(),
                    url: "https://github.com/you".into(), // <-- REPLACE
                    icon: Some("github".into()),
                },
                ContactLink {
                    label: "LinkedIn".into(),
                    url: "https://linkedin.com/in/you".into(), // <-- REPLACE
                    icon: Some("linkedin".into()),
                },
                ContactLink {
                    label: "Twitter / X".into(),
                    url: "https://x.com/you".into(), // <-- REPLACE
                    icon: Some("twitter".into()),
                },
            ],
        },
    }
}
