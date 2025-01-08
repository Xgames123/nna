use clap::Parser;
use std::{
    fs,
    io::{self, Read},
    path::Path,
    process,
};
use stderrlog::LogLevelNum;
mod asm;

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

    /// Prints the amount of space the program uses
    #[arg(short = 'm', long)]
    memory_usage: bool,

    /// The minimum log level
    #[arg(long, default_value = "0")]
    log_level: usize,
}

fn get_input_data(path: &str) -> io::Result<(Box<str>, String)> {
    if path == "-" {
        println!("Reading program from stdin...");
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
        process::exit(1);
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

    let output = match asm::assemble(filename.into(), &input_data) {
        Ok(out) => out,
        Err(err) => {
            err.print();
            process::exit(1);
        }
    };

    if cli.memory_usage {
        let mem_usage = asm::codegen::calc_mem_usage(&output);
        println!("Using {}/{} bytes", mem_usage.start, mem_usage.end);
    }
    fs::write(output_file, output).unwrap_or_else(|err| {
        die!("Failed to write output file:\n{}", err);
    });
}
