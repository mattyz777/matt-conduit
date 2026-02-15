use axum::{Router, extract::Request, middleware::Next, response::Response};
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use http::header::HeaderName;
use std::time::Instant;

pub fn with_request_id(router: Router) -> Router {
    let request_id_header = HeaderName::from_static("x-request-id");
    router
        // 生效顺序 3: 记录请求日志
        .layer(axum::middleware::from_fn(log_request))
        // 生效顺序 2: 传递 request_id 到下游服务
        .layer(PropagateRequestIdLayer::new(request_id_header.clone()))
        // 生效顺序 1: 如果没有 request_id，生成新的
        .layer(SetRequestIdLayer::new(request_id_header, MakeRequestUuid))
}

async fn log_request(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = req.method().to_string();
    let uri = req.uri().to_string();
    let request_id = req
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let response = next.run(req).await;

    let status = response.status().as_u16();
    let ms = start.elapsed().as_millis();

    tracing::info!(method, uri, request_id, status, ms, "http request");

    response
}