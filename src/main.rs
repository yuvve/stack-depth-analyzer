mod symex_adapter;

use clap::Parser;
use symex_adapter::{run_v6, run_v7};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the ELF file to analyze
    #[arg(short='f', long)]
    elf: String,

    /// Architecture of the ELF file
    #[arg(short='a', long, default_value = "v7")]
    arch: String,

    /// Entry point to analyze
    #[arg(short='e', long)]
    entry: String,

    // Print path results
    #[arg(short='p', long)]
    print: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let elf_path = args.elf;
    let entry_point = args.entry;
    let print = args.print;

    match args.arch.as_str() {
        "v6" => run_v6(&elf_path, &entry_point, print)?,
        "v7" => run_v7(&elf_path, &entry_point, print)?,
        _ => panic!("Unsupported architecture"),
    }

    Ok(())
}