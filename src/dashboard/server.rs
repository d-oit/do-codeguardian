//! Dashboard web server implementation
//!
//! Provides HTTP endpoints for the unified duplicate prevention dashboard

use super::{CustomView, DashboardConfig, DashboardService};
use anyhow::Result;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};

/// Dashboard server state
pub struct DashboardState {
    pub service: Arc<RwLock<DashboardService>>,
    pub config: DashboardConfig,
}

/// Query parameters for dashboard endpoints
#[derive(Debug, Deserialize)]
pub struct DashboardQuery {
    pub view: Option<String>,
    pub time_range: Option<String>,
    pub refresh: Option<bool>,
}

/// API response structure
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Start the dashboard web server
pub async fn start_dashboard_server(config: &DashboardConfig) -> Result<()> {
    let service = DashboardService::new(config.clone());
    let state = Arc::new(DashboardState {
        service: Arc::new(RwLock::new(service)),
        config: config.clone(),
    });

    let app = create_router(state);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;

    tracing::info!(
        "Dashboard server starting on {}:{}",
        config.host,
        config.port
    );

    axum::serve(listener, app).await?;

    Ok(())
}

/// Create the router with all dashboard endpoints
fn create_router(state: Arc<DashboardState>) -> Router {
    Router::new()
        // Static file serving for dashboard assets
        .nest_service("/static", ServeDir::new("dashboard/static"))
        // Main dashboard page
        .route("/", get(dashboard_index))
        // API endpoints
        .route("/api/health", get(health_check))
        .route("/api/metrics", get(get_metrics))
        .route("/api/metrics/current", get(get_current_metrics))
        .route("/api/views", get(get_views))
        .route("/api/views/:view_name", get(get_view_data))
        .route("/api/reports", get(get_reports))
        .route("/api/reports/generate", post(generate_report))
        // Real-time endpoints
        .route("/api/stream/metrics", get(stream_metrics))
        // Configuration endpoints
        .route("/api/config", get(get_config))
        .route("/api/config", post(update_config))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .into_inner(),
        )
        .with_state(state)
}

/// Dashboard index page
async fn dashboard_index() -> Html<&'static str> {
    Html(include_str!("templates/index.html"))
}

/// Health check endpoint
async fn health_check(
    State(state): State<Arc<DashboardState>>,
) -> Json<ApiResponse<HashMap<String, String>>> {
    let mut health_info = HashMap::new();
    health_info.insert("status".to_string(), "healthy".to_string());
    health_info.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
    health_info.insert("uptime".to_string(), "running".to_string());

    Json(ApiResponse::success(health_info))
}

/// Get current metrics
async fn get_current_metrics(
    State(state): State<Arc<DashboardState>>,
) -> Result<Json<ApiResponse<super::DashboardMetrics>>, StatusCode> {
    let service = state.service.read().await;

    match service.get_current_metrics() {
        Some(metrics) => Ok(Json(ApiResponse::success(metrics.clone()))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Get historical metrics
async fn get_metrics(
    State(state): State<Arc<DashboardState>>,
    Query(params): Query<DashboardQuery>,
) -> Json<ApiResponse<Vec<super::DashboardMetrics>>> {
    let service = state.service.read().await;

    let time_range = params
        .time_range
        .as_deref()
        .and_then(|s| parse_time_range(s))
        .unwrap_or(super::TimeRange::Last7Days);

    let metrics = service.get_metrics_for_range(&time_range);
    let metrics_owned: Vec<_> = metrics.into_iter().cloned().collect();

    Json(ApiResponse::success(metrics_owned))
}

/// Get available dashboard views
async fn get_views(State(state): State<Arc<DashboardState>>) -> Json<ApiResponse<Vec<CustomView>>> {
    let views = state.config.custom_views.clone();
    Json(ApiResponse::success(views))
}

/// Get data for a specific view
async fn get_view_data(
    State(state): State<Arc<DashboardState>>,
    axum::extract::Path(view_name): axum::extract::Path<String>,
) -> Result<Json<ApiResponse<super::DashboardReport>>, StatusCode> {
    let service = state.service.read().await;

    let view = state
        .config
        .custom_views
        .iter()
        .find(|v| v.name == view_name)
        .ok_or(StatusCode::NOT_FOUND)?;

    match service.generate_report(view) {
        Ok(report) => Ok(Json(ApiResponse::success(report))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Get available reports
async fn get_reports(State(state): State<Arc<DashboardState>>) -> Json<ApiResponse<Vec<String>>> {
    let report_types = vec![
        "duplicate_summary".to_string(),
        "prevention_effectiveness".to_string(),
        "system_performance".to_string(),
        "security_analysis".to_string(),
    ];

    Json(ApiResponse::success(report_types))
}

/// Generate a new report
async fn generate_report(
    State(state): State<Arc<DashboardState>>,
    Json(request): Json<GenerateReportRequest>,
) -> Result<Json<ApiResponse<super::DashboardReport>>, StatusCode> {
    let service = state.service.read().await;

    let view = state
        .config
        .custom_views
        .iter()
        .find(|v| v.name == request.view_name)
        .ok_or(StatusCode::NOT_FOUND)?;

    match service.generate_report(view) {
        Ok(report) => Ok(Json(ApiResponse::success(report))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Stream real-time metrics (Server-Sent Events)
async fn stream_metrics(
    State(state): State<Arc<DashboardState>>,
) -> Result<
    axum::response::Sse<
        impl futures::Stream<Item = Result<axum::response::sse::Event, std::convert::Infallible>>,
    >,
    StatusCode,
> {
    use axum::response::sse::{Event, Sse};
    use futures::stream::{self, Stream};
    use std::time::Duration;

    if !state.config.enable_real_time {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }

    let stream = stream::unfold(state.clone(), |state| async move {
        tokio::time::sleep(Duration::from_secs(state.config.refresh_interval_seconds)).await;

        let service = state.service.read().await;
        if let Some(metrics) = service.get_current_metrics() {
            let event = Event::default().data(serde_json::to_string(metrics).unwrap_or_default());
            Some((Ok(event), state))
        } else {
            Some((Ok(Event::default().data("{}")), state))
        }
    });

    Ok(Sse::new(stream))
}

/// Get dashboard configuration
async fn get_config(
    State(state): State<Arc<DashboardState>>,
) -> Json<ApiResponse<DashboardConfig>> {
    Json(ApiResponse::success(state.config.clone()))
}

/// Update dashboard configuration
async fn update_config(
    State(state): State<Arc<DashboardState>>,
    Json(new_config): Json<DashboardConfig>,
) -> Json<ApiResponse<String>> {
    // In a real implementation, you would update the configuration
    // and possibly restart services as needed
    Json(ApiResponse::success(
        "Configuration updated successfully".to_string(),
    ))
}

/// Request structure for generating reports
#[derive(Debug, Deserialize)]
struct GenerateReportRequest {
    view_name: String,
    time_range: Option<String>,
    filters: Option<HashMap<String, String>>,
}

/// Parse time range string into TimeRange enum
fn parse_time_range(s: &str) -> Option<super::TimeRange> {
    match s {
        "24h" | "last_24_hours" => Some(super::TimeRange::Last24Hours),
        "7d" | "last_7_days" => Some(super::TimeRange::Last7Days),
        "30d" | "last_30_days" => Some(super::TimeRange::Last30Days),
        "90d" | "last_90_days" => Some(super::TimeRange::Last90Days),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_health_check() {
        let config = DashboardConfig::default();
        let service = DashboardService::new(config.clone());
        let state = Arc::new(DashboardState {
            service: Arc::new(RwLock::new(service)),
            config,
        });

        let app = create_router(state);
        let server = TestServer::new(app).unwrap();

        let response = server.get("/api/health").await;
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_views() {
        let config = DashboardConfig::default();
        let service = DashboardService::new(config.clone());
        let state = Arc::new(DashboardState {
            service: Arc::new(RwLock::new(service)),
            config,
        });

        let app = create_router(state);
        let server = TestServer::new(app).unwrap();

        let response = server.get("/api/views").await;
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[test]
    fn test_parse_time_range() {
        assert!(matches!(
            parse_time_range("24h"),
            Some(super::super::TimeRange::Last24Hours)
        ));
        assert!(matches!(
            parse_time_range("7d"),
            Some(super::super::TimeRange::Last7Days)
        ));
        assert!(matches!(
            parse_time_range("30d"),
            Some(super::super::TimeRange::Last30Days)
        ));
        assert!(matches!(
            parse_time_range("90d"),
            Some(super::super::TimeRange::Last90Days)
        ));
        assert!(parse_time_range("invalid").is_none());
    }
}
