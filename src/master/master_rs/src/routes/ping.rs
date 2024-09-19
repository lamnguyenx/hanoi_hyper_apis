// GLOBAL
type ArcSemp = std::sync::Arc<tokio::sync::Semaphore>;
type Ext<T> = axum::extract::Extension<T>;

lazy_static::lazy_static! {

    static ref START_TIME : std::time::Instant = {
        std::time::Instant::now()
    };
}

// FUNCTIONS
pub async fn ping(sync_arc_ext: Ext<ArcSemp>) -> impl axum::response::IntoResponse {
    let _permit = sync_arc_ext.0.acquire().await.unwrap();
    return _ping().await;
}

pub async fn ping_1s(sync_arc_ext: Ext<ArcSemp>) -> impl axum::response::IntoResponse {
    let _permit = sync_arc_ext.0.acquire().await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    return _ping().await;
}

async fn _ping() -> impl axum::response::IntoResponse {
    let json_response = serde_json::json!({

        "up_time": format!("{:?}", START_TIME.elapsed()),
        "request_id": uuid::Uuid::new_v4(),

    });
    return axum::Json(json_response);
}
