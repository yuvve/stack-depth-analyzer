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

    /// List entries in the ELF file
    #[arg(
        short='l', 
        long, 
        conflicts_with="entry_point",
        conflicts_with="verbose",
        default_value="false"
    )]
    list_entries: bool,

    // Analyze specific entry point in the ELF file (otherwise do all of them)
    #[arg(short='e', long)]
    entry_point: Option<String>,

    // Verbose (prints Symex output)
    #[arg(short='v', long, default_value="false")]
    verbose: bool,
}

fn list_entries(elf_path: &str) -> Result<(), Box<dyn std::error::Error>> {
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

fn analyze_one_point(elf_path: &str, entry_point: &str, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let parsed_entry_point = analyze_entry_point(&elf_path, &entry_point, verbose);
    match parsed_entry_point {
        Ok(parsed_entry_point) => {
            println!("Entry Point: {}", entry_point);
            println!("{}", parsed_entry_point);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}

fn analyze_all_entry_points(elf_path: &str, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let entry_points = ParsedElf.get_entry_points(&elf_path);
    
    match entry_points {
        Ok(entry_points) => {
            for entry_point in entry_points {
                analyze_one_point(&elf_path, &entry_point, verbose)?;
                println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let elf_path = args.elf;
    if args.entry_point.is_some() {
        analyze_one_point(&elf_path, &args.entry_point.unwrap(), args.verbose)?;
    } else if args.list_entries {
        list_entries(&elf_path)?;
    } else {
        analyze_all_entry_points(&elf_path, args.verbose)?;
    }
    Ok(())
}
