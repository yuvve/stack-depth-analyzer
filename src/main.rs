use clap::Parser;
use symex::run_elf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the ELF file to analyze
    #[arg(short='f', long)]
    elf: String,

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

    let elf_results = run_elf::run_elf(&elf_path, &entry_point, print)?;

    // Example of how to access the stack usage results
    for result in elf_results {
        let initial_sp = result.initial_sp;
        let stack_hashet = result.stack_usage.unwrap();
        let min_stack = stack_hashet.iter().min().unwrap();
        let max_stack_depth = initial_sp - *min_stack;
        println!("Initial SP for {}: 0x{:x}", entry_point, initial_sp);
        println!("Lowest address in accessed by {}: 0x{:x}", entry_point, min_stack);
        println!("Maximal stack depth for {}: {} bytes", entry_point, max_stack_depth);
    }

    Ok(())
}
