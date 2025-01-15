use symex::run_elf;

pub fn analyze_entry_point(elf_path: &str, entry_point: &str, print: bool) -> Vec<u64>{
    let elf_results = run_elf::run_elf(&elf_path, &entry_point, print).unwrap();
    let mut stack_depths: Vec<u64> = Vec::new();

    // Example of how to access the stack usage results
    for result in elf_results {
        let initial_sp = result.initial_sp;
        let stack_hashet = result.stack_usage.unwrap();
        let min_stack = stack_hashet.iter().min().unwrap();
        let max_stack_depth = initial_sp - *min_stack;
        println!("Initial SP for {}: 0x{:x}", entry_point, initial_sp);
        println!("Lowest address in accessed by {}: 0x{:x}", entry_point, min_stack);
        println!("Maximal stack depth for {}: {} bytes", entry_point, max_stack_depth);

        stack_depths.push(max_stack_depth);
    }

    stack_depths
}