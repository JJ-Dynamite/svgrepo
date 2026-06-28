use axum::{
    routing::get,
    Router, Json, response::IntoResponse,
};
use tower_http::cors::{CorsLayer, Any};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct HealthResponse { status: String, service: String, version: String }
#[derive(Serialize)]
struct RootResponse { service: String, version: String, description: String, endpoints: Vec<String> }
#[derive(Serialize)]
struct Icon { id: String, name: String, category: String, svg: String, tags: Vec<String> }
#[derive(Serialize)]
struct IconListResponse { icons: Vec<Icon>, total: usize }

async fn health() -> impl IntoResponse {
    Json(HealthResponse { status: "healthy".into(), service: "svgrepo".into(), version: "0.1.0".into() })
}

async fn root() -> impl IntoResponse {
    Json(RootResponse {
        service: "svgrepo".into(), version: "0.1.0".into(),
        description: "500k+ free SVG vectors & icons".into(),
        endpoints: vec!["GET /health".into(), "GET /icons?q=search".into(), "GET /icons/:id".into()],
    })
}

async fn search_icons(axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>) -> impl IntoResponse {
    let q = params.get("q").unwrap_or(&"".to_string()).clone();
    let icons = vec![
        Icon { id: "1".into(), name: "Home".into(), category: "ui".into(), svg: "<svg viewBox='0 0 24 24'><path d='M3 12l9-9 9 9'/></svg>".into(), tags: vec!["home".into(), "house".into()] },
        Icon { id: "2".into(), name: "Settings".into(), category: "ui".into(), svg: "<svg viewBox='0 0 24 24'><circle cx='12' cy='12' r='3'/></svg>".into(), tags: vec!["settings".into(), "gear".into()] },
        Icon { id: "3".into(), name: "User".into(), category: "people".into(), svg: "<svg viewBox='0 0 24 24'><circle cx='12' cy='8' r='4'/></svg>".into(), tags: vec!["user".into(), "person".into()] },
    ];
    Json(IconListResponse { icons, total: 3 })
}

async fn get_icon(axum::extract::Path(id): axum::extract::Path<String>) -> impl IntoResponse {
    Json(Icon {
        id: id.clone(), name: "Icon".into(), category: "ui".into(),
        svg: "<svg viewBox='0 0 24 24'><path d='M3 12l9-9 9 9'/></svg>".into(),
        tags: vec!["icon".into()],
    })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let app = Router::new()
        .route("/", get(root)).route("/health", get(health))
        .route("/icons", get(search_icons)).route("/icons/:id", get(get_icon))
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!("svgrepo backend running on port 3001");
    axum::serve(listener, app).await.unwrap();
}
