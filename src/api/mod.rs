//! API route handlers for CryptoScope HTTP API.

pub mod auth;
pub mod error;
pub mod exchanges;
pub mod extractors;
pub mod refresh;
pub mod screener;
pub mod stats;
pub mod symbols;
pub mod types;
pub mod utils;

use axum::{routing::get, routing::post, Router};
use governor::middleware::StateInformationMiddleware;
use tower_governor::{
    governor::GovernorConfigBuilder,
    key_extractor::PeerIpKeyExtractor,
    GovernorLayer,
};

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub keys: auth::Keys,
    pub admin_credentials: auth::AdminCredentials,
}

/// Build the API router by merging all domain routers with rate limiting
pub fn router() -> Router<AppState> {
    Router::new()
        // Permissive tier: 50 req/s, burst 100 (health, exchanges)
        .merge(
            Router::new()
                .route("/health", get(health_check))
                .merge(exchanges::router())
                .layer(create_rate_limiter(50, 100)),
        )
        // General tier: 10 req/s, burst 20 (symbols, stats)
        .merge(
            Router::new()
                .merge(symbols::router())
                .merge(stats::router())
                .layer(create_rate_limiter(10, 20)),
        )
        // Strict tier: 2 req/s, burst 5 (screener, refresh, auth)
        .merge(
            Router::new()
                .merge(screener::router())
                .merge(refresh::router())
                .route("/api/v1/auth/login", post(auth::login))
                .layer(create_rate_limiter(2, 5)),
        )
}

/// Create a rate limiter with specified requests per second and burst size
fn create_rate_limiter(
    per_second: u64,
    burst_size: u32,
) -> GovernorLayer<PeerIpKeyExtractor, StateInformationMiddleware, axum::body::Body> {
    let config = Box::new(
        GovernorConfigBuilder::default()
            .per_second(per_second)
            .burst_size(burst_size)
            .key_extractor(PeerIpKeyExtractor)
            .use_headers()
            .finish()
            .unwrap(),
    );
    GovernorLayer::new(config)
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = str),
    ),
)]
pub async fn health_check() -> &'static str {
    "OK"
}
