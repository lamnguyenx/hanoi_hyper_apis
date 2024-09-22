use std::fs::File;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

pub fn convert() -> io::Result<()> {
    // Open the WAV file
    let file_path = "/data/data_sets/THOI_SU/Thoi_Su-15s-trio.wav"; // Change this to your WAV file path
    let mut file = File::open(file_path)?;

    // Start the ffmpeg process to convert WAV to MP3
    let output_file_path = "/data/data_sets/THOI_SU/Thoi_Su-15s-trio.mp3"; // Change this to your desired output file path
    let mut child = Command::new("ffmpeg")
        .arg("-y") // Overwrite output file if it exists
        .arg("-i") // Input option
        .arg("pipe:0") // Input from stdin
        .arg("-codec:a") // Audio codec option
        .arg("libmp3lame") // Use the LAME MP3 encoder
        .arg(output_file_path) // Direct output to a file
        .stdin(Stdio::piped())
        .spawn()?;

    // Get the stdin of the ffmpeg process
    let mut stdin = child.stdin.take().unwrap();

    // Buffer for reading the WAV file
    let mut buffer = vec![0; 4096]; // Adjust buffer size as needed

    // Simulate streaming the WAV file
    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break; // End of file
        }

        // Write the data to ffmpeg's stdin
        stdin.write_all(&buffer[..bytes_read])?;

        // Simulate slow streaming
        thread::sleep(Duration::from_millis(100)); // Adjust delay as needed
    }

    // Close stdin to indicate EOF to ffmpeg
    drop(stdin);

    // Wait for ffmpeg to finish
    let _ = child.wait()?;

    Ok(())
}
