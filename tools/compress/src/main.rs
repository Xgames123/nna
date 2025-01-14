use std::path::PathBuf;

use clap::Parser;

mod ffmpeg;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg()]
    input: PathBuf,
    #[arg(short = 'o', long, default_value = "compressed.bin")]
    output: PathBuf,

    ///Folder where temporary files will be written (frames of video as bmp images)
    #[arg(long, default_value = "/tmp/compress_cache")]
    tmp_dir: PathBuf,

    ///full path to the ffmpeg binary
    #[arg(long, default_value = "ffmpeg")]
    ffmpeg: PathBuf,
}

macro_rules! die {
    ($format:literal,$($arg:expr),+) => {
        const COLOR_RED: &'static str = "\x1b[31m";
        const RESET: &'static str = "\x1b[0m";
        eprintln!(concat!("\n{}", $format,"{}"), COLOR_RED, $($arg),+, RESET);
        std::process::exit(1);
    };
}
pub trait OrDie<T> {
    fn or_die(self) -> T;
}
impl<T, E: std::fmt::Display> OrDie<T> for Result<T, E> {
    fn or_die(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                die!("{}", e);
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    if !cli.input.exists() {
        die!("input file {:?} was not found", &cli.input);
    }
    let meta = cli.input.metadata().or_die();
    let sequence_dir = if meta.is_file() {
        let output_dir = cli.tmp_dir;
        println!("Creating tmp directory...");
        std::fs::create_dir_all(&output_dir)
            .map_err(|e| format!("io error during creation of tmp directory: {}", e))
            .or_die();
        ffmpeg::split_frames(&cli.ffmpeg, &cli.input, &output_dir).or_die();
        output_dir
    } else {
        cli.input
    };

    for frame in std::fs::read_dir(sequence_dir)
        .map_err(|e| format!("io error during listing of sequence directory: {}", e))
        .or_die()
    {
        let frame = match frame {
            Ok(frame) => frame,
            Err(e) => {
                eprintln!("Could not read file: {}", e);
                continue;
            }
        };
    }
}
