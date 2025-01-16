mod elf_parser;
mod symex_adapter;

use clap::Parser;
use elf_parser::{ParseElf, ParsedElf};
use symex_adapter::analyze_entry_point;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the ELF file to analyze
    #[arg(short='f', long)]
    elf: String,

    // Analyze specific entry point in the ELF file (otherwise do all of them)
    #[arg(short='e', long)]
    entry_point: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let elf_path = args.elf;
    if args.entry_point.is_some() {
        let entry_point = args.entry_point.unwrap();
        let parsed_entry_point = analyze_entry_point(&elf_path, &entry_point, false);
        match parsed_entry_point {
            Ok(parsed_entry_point) => {
                println!("Entry Point: {}", entry_point);
                println!("{}", parsed_entry_point);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
        return Ok(());
    }

    let entry_points = ParsedElf.get_entry_points(&elf_path);

    match entry_points {
        Ok(entry_points) => {
            for entry_point in entry_points {
                let parsed_entry_point = analyze_entry_point(&elf_path, &entry_point, false);
                match parsed_entry_point {
                    Ok(parsed_entry_point) => {
                        println!("Entry Point: {}", entry_point);
                        println!("{}", parsed_entry_point);
                        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
