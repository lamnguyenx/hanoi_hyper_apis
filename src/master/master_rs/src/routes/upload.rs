// TRAITS
use tokio::io::AsyncWriteExt;

// OVERRIDES
use anyhow::Result;
use anyhow::Error;
use anyhow::anyhow;

// GLOBAL
type ArcSemp = std::sync::Arc<tokio::sync::Semaphore>;
type Ext<T> = axum::extract::Extension<T>;
const UPLOADS_DIRECTORY: &str = "exp/uploads";

// FUNCTIONS
#[axum::debug_handler]
pub async fn upload(
    sync_arc_ext: Ext<ArcSemp>,
    multipart: axum::extract::Multipart
) -> impl axum::response::IntoResponse {
    let _permit = sync_arc_ext.0.acquire().await.unwrap();

    match _upload(multipart).await {
        Ok(result) => result,
        Err(e) =>
            axum::Json(
                serde_json::json!({
                "error" : e.to_string(),
                "error_traceback" : format!("{}", e.backtrace())
            })
            ),
    }
}

pub async fn _upload(
    mut multipart: axum::extract::Multipart
) -> Result<axum::Json<serde_json::Value>, Error> {
    //

    let mut unknown_fields: Vec<String> = Vec::new();
    let mut saved_file = std::path::PathBuf::new();

    while let Some(mut field) = multipart.next_field().await? {
        match field.name() {
            None => {
                return Err(anyhow!("field name cant be empty"));
            }
            Some("file") => {
                // prep file
                saved_file = get_saved_file();
                tokio::fs::create_dir_all(saved_file.parent().unwrap()).await?;

                // load ffmpeg
                let mut ffmpeg_subprocess = new_ffmpeg_subprocess(&saved_file);
                let mut ffmpeg_subprocess_stdin = ffmpeg_subprocess.stdin.take().unwrap();

                // iter through the stream
                while let Some(chunk) = field.chunk().await? {
                    println!("chunk: {}", chunk.len());
                    ffmpeg_subprocess_stdin.write_all(&chunk).await?;
                }

                // close
                drop(ffmpeg_subprocess_stdin);
                let _ = ffmpeg_subprocess.wait().await?;
                println!("done streaming");
            }

            Some(name) => unknown_fields.push(name.to_string()),
        }
    }

    return Ok(
        axum::Json(
            serde_json::json!({
                "request_id": uuid::Uuid::new_v4(),
                "uploading": "SUCCESS",
                "unknown_fields" : unknown_fields,
                "saved_file" : saved_file,

            })
        )
    );
}

fn get_saved_file() -> std::path::PathBuf {
    let saved_file = std::path::PathBuf
        ::new()
        .join(UPLOADS_DIRECTORY)
        .join(utils_rs::get_timedir_hourly())
        .join(utils_rs::get_timeslug())
        .with_extension("wav");
    return std::path::absolute(saved_file).unwrap();
}

fn new_ffmpeg_subprocess(out_file: &std::path::PathBuf) -> tokio::process::Child {
    let args_01: Vec<&str> = "-i pipe:0 -hide_banner -loglevel error".split_whitespace().collect();
    let args_02: Vec<&str> = "-ac 1 -ar 16000 -sample_fmt s16".split_whitespace().collect();

    return tokio::process::Command
        ::new("ffmpeg")
        .args(&args_01)
        .args(&args_02)
        .arg(out_file.to_str().unwrap())
        .stdin(std::process::Stdio::piped())
        .spawn()
        .unwrap();
}
