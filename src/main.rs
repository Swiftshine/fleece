use anyhow::Result;
use clap::{Parser, ValueEnum};
use gfbson::*;
use std::fs;
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Mode {
    /// Convert BSON to JSON
    Decode,
    /// Convert JSON to BSON
    Encode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum CLIEndianness {
    Auto,
    Big,
    Little,
}

impl From<CLIEndianness> for Endianness {
    fn from(cli: CLIEndianness) -> Self {
        match cli {
            CLIEndianness::Auto => Endianness::Auto,
            CLIEndianness::Big => Endianness::Big,
            CLIEndianness::Little => Endianness::Little,
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    /// Action to perform
    #[arg(value_enum)]
    mode: Mode,

    /// Input file path
    #[arg(short, long)]
    input: PathBuf,

    /// Output file path
    #[arg(short, long)]
    output: PathBuf,

    /// Endianness for BSON files (Default: Auto for reading, Big for writing)
    #[arg(short, long, value_enum, default_value_t = CLIEndianness::Auto)]
    endian: CLIEndianness,

    /// BSON version to use when writing
    #[arg(short, long, default_value_t = 3)]
    version: u32,

    /// Pretty-print JSON output
    #[arg(short, long)]
    pretty: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.mode {
        Mode::Decode => {
            // read BSON file
            let data = fs::read(&args.input)?;
            let root = read(&data, args.endian.into())
                .map_err(|e| anyhow::anyhow!("Failed to read BSON: {}", e))?;

            // convert to JSON string
            let json_out = to_json(&root, args.pretty)
                .map_err(|e| anyhow::anyhow!("Failed to convert to JSON: {}", e))?;

            fs::write(&args.output, json_out)?;
            println!(
                "Successfully decoded {} to {}",
                args.input.display(),
                args.output.display()
            );
        }
        Mode::Encode => {
            // read JSON file
            let json_str = fs::read_to_string(&args.input)?;
            let root =
                from_json(&json_str).map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;

            // write to BSON bytes
            let bson_bytes = write(&root, args.version, args.endian.into())
                .map_err(|e| anyhow::anyhow!("Failed to write BSON: {}", e))?;

            fs::write(&args.output, bson_bytes)?;
            println!(
                "Successfully encoded {} to {}",
                args.input.display(),
                args.output.display()
            );
        }
    }

    Ok(())
}
