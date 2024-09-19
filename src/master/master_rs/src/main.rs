mod routes;
mod utils;

// OPERATIONS
#[tokio::main]
async fn main() {
    //
    let host = utils::get_env!("MASTER_HOST", "0.0.0.0");
    let port = 8080;
    let addr = format!("{}:{}", host, port);

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let trace_layer = tower_http::trace::TraceLayer::new_for_http()
        .make_span_with(tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    let app = axum::Router::new()
        .route("/ping", axum::routing::get(routes::ping::ping))
        .route("/upload", axum::routing::post(routes::upload::upload))
        .layer(axum::extract::DefaultBodyLimit::max(64 * 1024* 1024))
        .layer(trace_layer);

    println!("-----------------------------");
    tracing::info!("Running at {}", addr);
    let tcp_listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(tcp_listener, app).await.unwrap();
}

// use webrtc_vad::{SampleRate, Vad, VadMode};
// fn main() {
//     let mut webrtc_vad_obj =
//         Vad::new_with_rate_and_mode(SampleRate::Rate16kHz, VadMode::Aggressive);

//     let audio_frame: Vec<i16> = vec![0; 160];
//     let is_frame_speech = webrtc_vad_obj.is_voice_segment(&audio_frame);
//     dbg!(is_frame_speech);
// }
