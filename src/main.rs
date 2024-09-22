use clap::Parser;
use std::path::Path;
use std::process::Command;

/// Simple YouTube video downloader using yt-dlp
#[derive(Parser, Debug)]
#[command(name = "youtube_downloader")]
#[command(author = "Your Name <your.email@example.com>")]
#[command(version = "1.0")]
#[command(about = "Downloads videos from YouTube", long_about = None)]
struct Args {
    /// The URL of the YouTube video
    #[arg(short, long)]
    url: String,

    /// The directory to save the video in
    #[arg(short, long, default_value = ".")]
    output_dir: String,

    /// Download audio only
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    audio_only: bool,
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Check if output directory exists
    let output_path = Path::new(&args.output_dir);
    if !output_path.exists() {
        eprintln!("Error: Output directory does not exist");
        return;
    }

    // Build the yt-dlp command
    let mut command = Command::new("yt-dlp");

    command
        .arg(&args.url)
        .arg("-o")
        .arg(format!("{}/%(title)s.%(ext)s", args.output_dir));

    if args.audio_only {
        command.arg("-x").arg("--audio-format").arg("mp3");
    }

    // Execute the yt-dlp command
    let status = command.status().expect("Failed to execute yt-dlp");

    if status.success() {
        println!("Download completed successfully!");
    } else {
        // Fix: Use `{:?}` to print the Option<i32> or handle it using `unwrap_or`
        eprintln!(
            "Error: Download failed with status code: {:?}",
            status.code().unwrap_or(-1) // Provide a default error code if it's `None`
        );
    }
}
