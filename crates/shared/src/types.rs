//! Core serde types shared between the WASM frontend and the Rust API.

use serde::{Deserialize, Serialize};

/// One of the nine portfolio sections (00–08) reachable via navigation,
/// keyboard shortcuts, or camera focus inside the Bevy 3D scene.
///
/// Variants are serialized as their section-prefixed IDs (e.g. `"00-home"`)
/// so the frontend and API agree on the canonical section identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SectionId {
    #[serde(rename = "00-home")]
    Home,
    #[serde(rename = "01-about")]
    About,
    #[serde(rename = "02-experience")]
    Experience,
    #[serde(rename = "03-stack")]
    Stack,
    #[serde(rename = "04-skills")]
    Skills,
    #[serde(rename = "05-projects")]
    Projects,
    #[serde(rename = "06-blog")]
    Blog,
    #[serde(rename = "07-contact")]
    Contact,
    #[serde(rename = "08-elsewhere")]
    Elsewhere,
}

impl SectionId {
    /// Human-readable label used in the HUD window title bars.
    pub fn label(self) -> &'static str {
        match self {
            SectionId::Home => "00 — Home",
            SectionId::About => "01 — About",
            SectionId::Experience => "02 — Experience",
            SectionId::Stack => "03 — Stack",
            SectionId::Skills => "04 — Skills",
            SectionId::Projects => "05 — Projects",
            SectionId::Blog => "06 — Blog",
            SectionId::Contact => "07 — Contact",
            SectionId::Elsewhere => "08 — Elsewhere",
        }
    }

    /// Anchor hash used in URLs (`#home`, `#about`, …).
    pub fn anchor(self) -> &'static str {
        match self {
            SectionId::Home => "home",
            SectionId::About => "about",
            SectionId::Experience => "experience",
            SectionId::Stack => "stack",
            SectionId::Skills => "skills",
            SectionId::Projects => "projects",
            SectionId::Blog => "blog",
            SectionId::Contact => "contact",
            SectionId::Elsewhere => "elsewhere",
        }
    }

    /// All nine sections in order.
    pub const fn all() -> [SectionId; 9] {
        [
            SectionId::Home,
            SectionId::About,
            SectionId::Experience,
            SectionId::Stack,
            SectionId::Skills,
            SectionId::Projects,
            SectionId::Blog,
            SectionId::Contact,
            SectionId::Elsewhere,
        ]
    }
}

/// Top-level portfolio document returned by `GET /api/portfolio`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioData {
    /// Deployed build/version string (e.g. git SHA or semver).
    pub version: String,
    /// Short build identifier shown in the telemetry strip.
    pub build: String,
    /// Public display name — REPLACE with your name.
    pub name: String,
    /// Professional headline / role — REPLACE with your headline.
    pub title: String,
    /// Location / availability — REPLACE with your location.
    pub location: String,
    /// Current status shown in the telemetry strip (e.g. "Available for work").
    pub status: String,
    /// Short bio used on the About panel.
    pub bio: String,
    /// Categorized technical stack.
    pub stack: Stack,
    /// Featured projects shown on the Projects panel and as 3D focus targets.
    pub projects: Vec<ProjectEntry>,
    /// Professional experience (About / Experience panels).
    pub experience: Vec<ExperienceEntry>,
    /// Certifications (About / Certifications panels).
    pub certifications: Vec<CertificationEntry>,
    /// Recent build/deploy log entries for the telemetry strip.
    pub build_log: Vec<BuildLogEntry>,
    /// External ventures / agencies / related sites (Elsewhere panel).
    pub elsewhere: Vec<ElsewhereEntry>,
    /// Contact information and social links.
    pub contact: ContactInfo,
}

/// Categorized technical stack (03 — Stack).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stack {
    pub languages: Vec<String>,
    pub frameworks: Vec<String>,
    pub cloud_hosting: Vec<String>,
    pub tooling: Vec<String>,
    pub platforms: Vec<String>,
}

/// A featured project (05 — Projects). Synchronized with a 3D focus target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEntry {
    pub id: String,
    pub title: String,
    pub description: String,
    /// Short teaser shown on the 3D node / card.
    pub teaser: String,
    pub tech: Vec<String>,
    /// Deployment/runtime platform (e.g. "Vercel", "AWS", "fly.io").
    pub platform: String,
    /// External demo URL.
    pub demo_url: Option<String>,
    /// Source repository URL.
    pub repo_url: Option<String>,
    /// Section this project focuses the camera on when selected.
    pub focus_section: Option<SectionId>,
}

/// A work history entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceEntry {
    pub company: String,
    pub role: String,
    pub start: String,
    pub end: Option<String>,
    pub current: bool,
    pub highlights: Vec<String>,
    pub tech: Vec<String>,
}

/// A certification or credential.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificationEntry {
    pub name: String,
    pub issuer: String,
    pub date: String,
    pub url: Option<String>,
}

/// A single build/deploy log entry shown in the telemetry strip.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildLogEntry {
    pub id: String,
    pub timestamp: String,
    pub status: String, // "ok" | "building" | "failed" | "deployed"
    pub message: String,
}

/// External venture / agency / related site (08 — Elsewhere).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElsewhereEntry {
    pub name: String,
    pub description: String,
    pub url: String,
    pub logo_hint: Option<String>,
}

/// Contact information and social links (07 — Contact).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub email: String,
    pub location: String,
    pub links: Vec<ContactLink>,
}

/// A single contact/social link.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactLink {
    pub label: String,
    pub url: String,
    pub icon: Option<String>,
}

/// Payload submitted by the interactive contact form to `POST /api/contact`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactPayload {
    pub name: String,
    pub email: String,
    pub message: String,
}

/// Structured error returned by API endpoints.
#[derive(Debug, Clone, Serialize)]
pub struct ApiError {
    pub error: String,
    pub code: Option<String>,
    pub details: Option<String>,
}

impl ApiError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            error: message.into(),
            code: Some("bad_request".into()),
            details: None,
        }
    }
    pub fn server_error(message: impl Into<String>) -> Self {
        Self {
            error: message.into(),
            code: Some("internal_error".into()),
            details: None,
        }
    }
}

/// Success envelope for API responses.
#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    pub ok: bool,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self { ok: true, data }
    }
}

/// Success envelope for the contact endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct ContactResult {
    pub accepted: bool,
    pub message: String,
    pub id: Option<String>,
}
