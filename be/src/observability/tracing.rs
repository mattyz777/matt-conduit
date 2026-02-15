use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_subscriber::fmt::time::FormatTime;
use std::fs::OpenOptions;
use chrono::Timelike;

/// 自定义时间格式: 2026-02-15 10:19:12,638
struct LocalTime;

impl FormatTime for LocalTime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = chrono::Local::now();

        write!(
            w,
            "{} {:02}:{:02}:{:02},{:03}",
            now.format("%Y-%m-%d"),
            now.hour(),
            now.minute(),
            now.second(),
            now.timestamp_subsec_millis()
        )
    }
}

/**
 * 设置日志级别 - 通过 RUST_LOG 环境变量控制（默认 info）
 * 配置日志输出 - 同时输出到终端和 app.log 文件
 * 注册全局 subscriber - 让 tracing::info!() 等宏生效
 */
pub fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("app.log")
        .expect("failed to open app.log");

    tracing_subscriber::registry()
        // 日志级别
        .with(env_filter)
        // 终端输出
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_timer(LocalTime)
                .with_writer(std::io::stdout),
        )
        // 文件输出
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_ansi(false)
                .with_timer(LocalTime)
                .with_writer(log_file),
        )
        .init();
}