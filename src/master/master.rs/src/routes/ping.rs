// GLOBAL
lazy_static::lazy_static! {

    static ref START_TIME : std::time::Instant = {
        std::time::Instant::now()
    };
}


// FUNCTIONS
pub async fn ping() -> impl axum::response::IntoResponse {
    let json_response = serde_json::json!({

        "up_time": format!("{:?}", START_TIME.elapsed()),
        "request_id": uuid::Uuid::new_v4(),

    });
    axum::Json(json_response)
}
