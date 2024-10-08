mod health_check;

use std::{sync::Arc, time::Duration};

use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response,
    routing::{get, post},
    Router,
};

pub use health_check::*;
use sqlx::PgPool;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};
use uuid::Uuid;

use crate::{speak::yoda_speak, state::AppState};

pub fn route(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/yoda", post(yoda_speak))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);
                    info_span!(
                        "http-request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                        request_id=%Uuid::new_v4(),
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {})
                .on_response(|_response: &Response<_>, _latency: Duration, _span: &Span| {})
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {})
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {},
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {},
                ),
        )
}
