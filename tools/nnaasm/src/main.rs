#![allow(dead_code)]
use asm::Bank;
use clap::{Parser, ValueEnum};
use libnna::Architecture;
use std::{
    ffi::OsStr,
    fs,
    io::{self, Read},
    path::Path,
    process,
};
use stderrlog::LogLevelNum;
mod asm;

#[derive(ValueEnum, Clone)]
enum OutputFormat {
    Auto,
    Bin,
    Hex,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Input file or - to read from stdin
    #[arg(default_value = "-")]
    input: String,

    /// The instruction set to compile for
    #[arg(short = 'a', long, default_value = "nna8v1")]
    arch: Architecture,

    /// Output file
    #[arg(short = 'o', long, default_value = "out.bin")]
    output: String,

    /// Prints the amount of space the program uses
    #[arg(short = 'z', long)]
    size: bool,

    /// The minimum log level
    #[arg(long, default_value = "0")]
    log_level: usize,

    /// The format of the output file
    #[arg(short = 'f', long, default_value = "auto")]
    format: OutputFormat,
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

#[inline]
fn u4_to_hex(val: u8) -> char {
    let lower = val & 0x0F;
    if lower > 9 {
        (lower + 55) as char
    } else {
        (lower + 48) as char
    }
}

#[inline]
fn write_hexline(str: &mut String, byte: u8, repeat_count: usize) {
    if repeat_count > 1 {
        str.push_str(&repeat_count.to_string());
        str.push('*');
    }
    str.push(u4_to_hex(byte >> 4));
    str.push(u4_to_hex(byte));
    str.push('\n');
}

fn write_hex(input: Vec<Bank>) -> Vec<u8> {
    let mut output = "v2.0 raw\n".to_string();
    let mut prev_byte = input[0][0];
    let mut repeat_count = 0;
    for byte in input.iter().flat_map(|b| b.iter()) {
        if *byte == prev_byte {
            repeat_count += 1;
            continue;
        }
        write_hexline(&mut output, prev_byte, repeat_count);
        prev_byte = *byte;
        repeat_count = 1;
    }
    write_hexline(&mut output, prev_byte, repeat_count);
    output.into_bytes()
}
fn write_bin(input: Vec<Bank>) -> Vec<u8> {
    input.into_iter().flat_map(|b| b.into_iter()).collect()
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

    let output = match asm::assemble(filename.into(), &input_data, cli.arch) {
        Ok(out) => out,
        Err(err) => {
            err.print();
            process::exit(1);
        }
    };

    if cli.size {
        for (i, bank) in asm::codegen::calc_mem_usage(&output, 256)
            .iter()
            .enumerate()
        {
            println!("bank {:#x} Using {}/{} bytes", i, bank.start, bank.end);
        }
    }

    let output = match cli.format {
        OutputFormat::Bin => write_bin(output),
        OutputFormat::Hex => write_hex(output),
        OutputFormat::Auto => {
            if output_file.extension() == Some(OsStr::new("hex")) {
                write_hex(output)
            } else {
                write_bin(output)
            }
        }
    };

    fs::write(output_file, output).unwrap_or_else(|err| {
        die!("Failed to write output file:\n{}", err);
    });
}
