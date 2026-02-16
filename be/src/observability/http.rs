use axum::{
    Router,
    extract::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tower_http::{request_id::{PropagateRequestIdLayer, SetRequestIdLayer, MakeRequestUuid}, catch_panic::CatchPanicLayer};

pub fn with_request_id(router: Router) -> Router {
    let request_id_header = http::header::HeaderName::from_static("x-request-id");
    router
        // 生效顺序 4: 捕获 panic，返回 500 错误
        .layer(CatchPanicLayer::new())
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

    // 尝试读取请求体（仅记录，不消耗原始请求）
    let body_summary = if method == "POST" || method == "PUT" || method == "PATCH" {
        // 检查 Content-Type 判断是否是 JSON
        let content_type = req
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown");

        if content_type.contains("application/json") {
            // 对于 JSON 请求，在 handler 中会记录具体参数
            "[json_body]".to_string()
        } else if content_type.contains("multipart/form-data")
            || content_type.contains("application/x-www-form-urlencoded")
        {
            "[form_data]".to_string()
        } else {
            format!("[content_type: {}]", content_type)
        }
    } else {
        "".to_string()
    };

    // 创建包含 request_id 的 span
    let request_id_header = req
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    let span = tracing::info_span!(
        "http_request",
        request_id = request_id_header,
        method = method,
        uri = uri
    );

    let _guard = span.enter();

    tracing::info!(
        body_summary = body_summary,
        "http request started"
    );

    let response = next.run(req).await;

    let status = response.status().as_u16();
    let ms = start.elapsed().as_millis();

    tracing::info!(
        status = status,
        ms = ms,
        "http request completed"
    );

    response
}
