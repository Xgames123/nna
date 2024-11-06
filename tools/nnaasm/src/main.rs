use asm::AsmError;
use clap::{Parser, ValueEnum};
use std::{
    fs,
    io::{self, Read},
    path::Path,
    process,
};
use stderrlog::LogLevelNum;

//use crate::{emiting::*, util::count_nonzero_banks};
mod asm;
//mod emiting;
mod util;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Input file or - to read from stdin
    #[arg(default_value = "-")]
    input: String,

    /// Output file
    #[arg(short = 'o', long, default_value = "out.bin")]
    output: String,

    /// The format of the output
    #[arg(short = 'f', long, default_value = "auto")]
    format: Format,

    /// Prints the amount of space the program uses
    #[arg(short = 'm', long)]
    memory_usage: bool,

    /// The minimum log level
    #[arg(long, default_value = "0")]
    log_level: usize,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
#[value()]
enum Format {
    ///Choose format based on file extension
    Auto,
    Hex,
    Bin,
}
fn get_input_data(path: &str) -> io::Result<(Box<str>, String)> {
    if path == "-" {
        let mut str = String::new();
        let mut stdin = io::stdin();
        stdin.read_to_string(&mut str)?;
        Ok(("stdin".into(), str))
    } else {
        let can_path = std::fs::canonicalize(&path)?;
        let filename = can_path
            .file_name()
            .map(|f| f.to_str())
            .flatten()
            .unwrap_or("unknown_file")
            .into();
        let str = fs::read_to_string(can_path)?;
        Ok((filename, str))
    }
}

macro_rules! die {
    ($($arg:tt)*) => {
        eprintln!($($arg)*);
        process::exit(-1);
    };
}

fn main() {
    let cli = Cli::parse();
    stderrlog::new()
        .verbosity(LogLevelNum::from(cli.log_level))
        .module(module_path!())
        .show_module_names(true)
        .init()
        .unwrap();

    let output_file = Path::new(&cli.output);

    let (filename, input_data) = get_input_data(&cli.input).unwrap_or_else(|err: io::Error| {
        die!("Failed to read '{}'\n{}", cli.input, err);
    });

    let err = AsmError {
        filename: filename.into(),
        file: &input_data,
        location: (10, 2..4),
        message: "test error".into(),
    };
    err.print();
    // let out = match asm::assemble(input_data) {
    //     Ok(out) => out,
    //     Err(err) => {
    //         die(&err.to_string());
    //         return;
    //     }
    // };
    //
    // let content = emit(
    //     cli.format,
    //     output_file.extension().map(|ext| ext.to_str()).flatten(),
    //     out,
    // );
    //
    // fs::write(cli.output, content).unwrap_or_else(|err| {
    //     die(&format!("Failed to write output file\n\n {}", err));
    // });
    //
    // if cli.memory_usage {
    //     println!("Using {}/16 banks", count_nonzero_banks(&out));
    // }
}
