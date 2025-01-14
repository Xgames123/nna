use std::{io::ErrorKind, path::Path, process::Command};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FFmpegError {
    #[error(
        "ffmpeg not found. Please install it in the default PATH or use the --ffmpeg argument"
    )]
    FfmpegNotFound,

    #[error("ffmpeg returned a non 0 exit code")]
    NonZeroExit,
    #[error("Io error when running ffmpeg: {0}")]
    Io(std::io::Error),
}
impl From<std::io::Error> for FFmpegError {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            ErrorKind::NotFound => Self::FfmpegNotFound,
            _ => Self::Io(e),
        }
    }
}

pub fn split_frames(
    ffmpeg_path: &Path,
    video: &Path,
    output_dir: &Path,
) -> Result<(), FFmpegError> {
    println!("splitting video into frames...");
    let output_files = output_dir.join("frame%d.bmp");
    let status = Command::new(ffmpeg_path)
        .arg("-i")
        .arg(video)
        .arg(output_files)
        .status()?;
    if !status.success() {
        return Err(FFmpegError::NonZeroExit);
    }
    Ok(())
}
