mod elf_parser;
mod symex_adapter;

use clap::Parser;
use elf_parser::{ParseElf, ParsedElf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the ELF file to analyze
    #[arg(short='f', long)]
    elf: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let elf_path = args.elf;
    let entry_points = ParsedElf.get_entry_points(&elf_path);

    match entry_points {
        Ok(entry_points) => {
            for entry_point in entry_points {
                println!("{}", entry_point);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
