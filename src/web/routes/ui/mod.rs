use crate::context::DataDanceContext;
use axum::Router;
use axum_embed::{FallbackBehavior, ServeEmbed};
use rust_embed::RustEmbed;
use std::sync::Arc;

#[derive(RustEmbed, Clone)]
#[folder = "target/site/"]
struct StaticUi;

pub fn ui_router(context: &Arc<DataDanceContext>) -> Router {
    let serve_embedded = ServeEmbed::<StaticUi>::with_parameters(
        Some("404.html".to_string()),
        FallbackBehavior::NotFound,
        Some("index.html".to_string()),
    );
    let router = Router::new().fallback_service(serve_embedded);

    router
}
