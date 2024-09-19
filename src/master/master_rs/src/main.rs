mod routes;
mod utils;


// OPERATIONS
#[tokio::main]
async fn main() {
    // basics
    let host = utils::get_env!("MASTER_HOST", "0.0.0.0");
    let port = 8080;
    let addr = format!("{}:{}", host, port);
    let max_workers = 4_usize;

    // loging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let trace_layer = tower_http::trace::TraceLayer::new_for_http()
        .make_span_with(tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    // limits
    let sync_arc = std::sync::Arc::new(tokio::sync::Semaphore::new(max_workers));

    // routes
    let app = axum::Router::new()
        .route("/ping", axum::routing::get(routes::ping::ping))
        .route("/ping_1s", axum::routing::get(routes::ping::ping_1s))
        .route("/upload", axum::routing::post(routes::upload::upload))
        .layer(axum::Extension(sync_arc))
        .layer(axum::extract::DefaultBodyLimit::max(64 * 1024* 1024))
        .layer(trace_layer);

    // starts
    println!("-----------------------------");
    tracing::info!("Running at {}", addr);
    let tcp_listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}
