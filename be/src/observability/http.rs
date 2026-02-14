use axum::Router;
use tower_http::{
    trace::TraceLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
};
use tracing::Level;
use http::header::HeaderName;
use axum::body::Body;

pub fn with_http_tracing(router: Router) -> Router {
    let request_id_header = HeaderName::from_static("x-request-id");
    router
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &http::Request<Body>| {
                    let request_id = req
                        .headers()
                        .get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("unknown");

                    tracing::span!(
                        Level::INFO,
                        "http_request",
                        method = %req.method(),
                        uri = %req.uri(),
                        request_id = %request_id,
                    )
                })
                .on_request(|_req: &http::Request<Body>, _span: &tracing::Span| {
                    tracing::info!("Request started");
                })
                .on_response(
                    |_res: &http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                        tracing::info!(latency_ms = latency.as_millis(), "Request completed");
                    },
                ),
        )
        .layer(PropagateRequestIdLayer::new(request_id_header.clone()))
        .layer(SetRequestIdLayer::new(request_id_header, MakeRequestUuid))
}