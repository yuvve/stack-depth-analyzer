use std::collections::HashSet;
use regex::Regex;
use symex::{
    general_assembly::{
        arch::{
            arm::{
            v6::ArmV6M, 
            v7::ArmV7EM,
        }, Arch},
        RunConfig,
        state::GAState,
        project::PCHook,
    }, 
    run_elf::{run_elf, run_elf_configured}
};

pub fn run_v6(elf_path: &str, entry_point: &str, print: bool) -> Result<(), Box<dyn std::error::Error>> {
    let conf = create_config::<ArmV6M>(elf_path, entry_point, print);
    let elf_results = run_elf_configured(
        &elf_path, 
        &entry_point, 
        ArmV6M {}, 
        conf)?;

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

pub fn run_v7(elf_path: &str, entry_point: &str, print: bool) -> Result<(), Box<dyn std::error::Error>> {
    let conf = create_config::<ArmV7EM>(elf_path, entry_point, print);
    let elf_results = run_elf_configured(
        &elf_path, 
        &entry_point, 
        ArmV7EM {pc_writes: HashSet::new()}, 
        conf)?;

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

fn create_config<A: Arch>(elf_path: &str, entry_point: &str, print: bool) -> RunConfig<A>{
    let regex = Regex::new(r"^__basepri_r$").unwrap();
    let get_prio = |state: &mut GAState<A>| {
        let prio = state.get_register("R0".to_owned()).unwrap();
        state.set_register("basepri".to_owned(), prio)?;
        
        // jump back to where the function was called from
        let lr = state.get_register("LR".to_owned()).unwrap();
        state.set_register("PC".to_owned(), lr)?;
        Ok(())
    };
    RunConfig::<A> {
        show_path_results: print,
        pc_hooks: vec![(regex, PCHook::Intrinsic(get_prio))],
        register_read_hooks: Vec::new(),
        register_write_hooks: Vec::new(),
        memory_write_hooks: Vec::new(),
        memory_read_hooks: Vec::new(),
    }
}