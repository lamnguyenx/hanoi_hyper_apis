use axum::{http::StatusCode, routing::post, Router };
use tracing_subscriber::{ layer::SubscriberExt, util::SubscriberInitExt };
use tokio::io::AsyncWriteExt;

const UPLOADS_DIRECTORY: &str = "exp/uploads";

#[tokio::main]
pub async fn main() {
    tracing_subscriber
        ::registry()
        .with(
            tracing_subscriber::EnvFilter
                ::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into())
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tokio::fs
        ::create_dir_all(UPLOADS_DIRECTORY).await
        .expect("failed to create `uploads` directory");

    let app = Router::new().route("/file/:file_name", post(save_multipart_file));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// Handler that processes multipart form-data and saves the file.
async fn save_multipart_file(
    axum::extract::Path(file_name): axum::extract::Path<String>,
    mut multipart: axum::extract::Multipart
) -> Result<(), (StatusCode, String)> {
    if !path_is_valid(&file_name) {
        return Err((StatusCode::BAD_REQUEST, "Invalid file name".to_owned()));
    }

    while
        let Some(mut field) = multipart
            .next_field().await
            .map_err(|err| {
                (StatusCode::BAD_REQUEST, format!("Failed to process multipart field: {}", err))
            })?
    {
        if field.name() == Some("file") {
            let path = std::path::Path::new(UPLOADS_DIRECTORY).join(&file_name);

            let mut child = tokio::process::Command
                ::new("ffmpeg")
                .args(
                    &[
                        "-i",
                        "pipe:0",
                        "-ac",
                        "1",
                        "-ar",
                        "16000",
                        "-sample_fmt",
                        "s16",
                        path.to_str().unwrap(),
                    ]
                )
                .stdin(std::process::Stdio::piped())
                .spawn()
                .unwrap();

            let mut child_stdin = child.stdin.take().unwrap();

            while
                let Some(chunk) = field
                    .chunk().await
                    .map_err(|err| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to read chunk: {}", err),
                        )
                    })?
            {
                println!("chunk: {}", chunk.len());
                child_stdin
                    .write_all(&chunk).await
                    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
            }

            // Close the writer to signal EOF
            drop(child_stdin);

            println!("Flushed");

            // Wait for the ffmpeg process to finish
            let _ = child
                .wait().await
                .map_err(|err| { (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()) })?;

            return Ok(());
        }
    }

    Err((StatusCode::BAD_REQUEST, "No file field found".to_owned()))
}

// to prevent directory traversal attacks we ensure the path consists of exactly one normal
// component
fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}
