// TRAITS
use tokio::io::AsyncWriteExt;

// GLOBAL
type ArcSemp = std::sync::Arc<tokio::sync::Semaphore>;
type Ext<T> = axum::extract::Extension<T>;

// FUNCTIONS
#[axum::debug_handler]
pub async fn upload(
    sync_arc_ext: Ext<ArcSemp>,
    multipart: axum::extract::Multipart,
) -> impl axum::response::IntoResponse {
    let _permit = sync_arc_ext.0.acquire().await.unwrap();

    match _upload(multipart).await {
        Ok(result) => result,
        Err(e) => axum::Json(serde_json::json!({
            "error" : e.to_string(),
            "error_traceback" : format!("{}", e.backtrace())
        })),
    }
}

pub async fn _upload(
    mut multipart: axum::extract::Multipart,
) -> anyhow::Result<axum::Json<serde_json::Value>, anyhow::Error> {
    //
    let mut file_bytes = axum::body::Bytes::new();
    let mut file_name = String::new();
    let mut unknown_fields: Vec<String> = Vec::new();

    while let Some(field) = multipart.next_field().await? {
        match field.name() {
            None => return Err(anyhow::anyhow!("field name cant be empty")),
            Some("file") => file_bytes = field.bytes().await?,
            Some("file_name") => file_name = field.text().await?,
            Some(name) => unknown_fields.push(name.to_string()),
        }
    }

    match file_bytes.is_empty() {
        true => return Err(anyhow::anyhow!("file is empty")),
        false => {
            // prep dir
            let disk_file_path = std::path::Path::new("exp").join(&file_name);
            let disk_file_path_parent = disk_file_path
                .parent()
                .ok_or_else(|| anyhow::anyhow!("disk_file_path_parent is None"))?;

            tokio::fs::create_dir_all(disk_file_path_parent).await?;

            // save file
            let mut disk_file_obj = tokio::fs::File::create(&disk_file_path).await?;
            disk_file_obj.write_all(&file_bytes).await?;

            let _dfp = disk_file_path.clone();
            let audio_info =
                tokio::task::spawn_blocking(move || crate::utils::probe_audio(&_dfp)).await?;

            // conclude
            return Ok(axum::Json(serde_json::json!({
                "request_id": uuid::Uuid::new_v4(),
                "uploading": "SUCCESS",
                "disk_file_path" : disk_file_path,
                "unknown_fields" : unknown_fields,
                "audio_info" : audio_info,
            })));
        }
    }
}
