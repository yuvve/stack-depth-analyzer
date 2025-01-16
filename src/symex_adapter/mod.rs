use std::fmt;
use symex::run_elf;

pub struct ParsedEntryPoint {
    initial_sp: u64,
    lowest_address: u64,
    max_stack_depth: u64,
}

impl fmt::Display for ParsedEntryPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Initial SP: {:#x}\nLowest Address: {:#x}\nMax Stack Depth: {} bytes", self.initial_sp, self.lowest_address, self.max_stack_depth)
    }
}

pub fn analyze_entry_point(elf_path: &str, entry_point: &str, print: bool) -> Result<ParsedEntryPoint, String>{
    let elf_results = match run_elf::run_elf(&elf_path, &entry_point, print) {
        Ok(x) => x,
        Err(e) => return Err(format!("Failed to run ELF: {}", e))
    };

    let mut deepest_stack = 0;
    let mut best_path_result: Option<ParsedEntryPoint> = None;

    for result in elf_results {
        let path_result = get_path_result(result);
        if path_result.max_stack_depth > deepest_stack {
            deepest_stack = path_result.max_stack_depth;
            best_path_result = Some(path_result);
        }
    }

    match Some(best_path_result) {
        Some(result) => Ok(result.unwrap()),
        None => Err("No results found".to_string())
    }
}

fn get_path_result(result: symex::elf_util::VisualPathResult) -> ParsedEntryPoint{
    let initial_sp = result.initial_sp;
    let stack_hashset = result.stack_usage.unwrap();
    let min_stack = stack_hashset.iter().min().unwrap();
    let max_stack_depth = initial_sp - *min_stack;

    ParsedEntryPoint {
        initial_sp,
        lowest_address: *min_stack,
        max_stack_depth,
    }
}

#[cfg(test)]
mod tests {
    use super::analyze_entry_point;
    use std::path::PathBuf;

    struct EntryPointsTestData {
        path: String, 
        entries_with_depth: Vec<(&'static str, u64)>, 
    }
    
    fn get_test_data() -> Vec<EntryPointsTestData> {
        let root_path = env!("CARGO_MANIFEST_DIR").to_owned();
        let assets_path = PathBuf::from(root_path).join("tests").join("assets");
        let test_file_path = assets_path.join("ex4");
    
        vec![
            EntryPointsTestData {
                path: test_file_path.to_str().unwrap().to_string(), 
                entries_with_depth: vec![
                    ("equal_formula_rec", 176),
                    ("equal_iter_rec", 184),
                    ("complexity_sum_recursive", 160),
                    ("complexity_sum_iterative", 32),
                    ("complexity_sum_formula", 16),
                ],
            },
        ]
    }

    #[test]
    fn test_entry_points_analysis() {
        let test_elf = get_test_data();
        for elf in test_elf {
            for entry in elf.entries_with_depth {
                let entry_point = analyze_entry_point(&elf.path, entry.0, false);
                assert!(entry_point.is_ok());
                let entry_point = entry_point.unwrap();
                assert_eq!(entry_point.max_stack_depth, entry.1);
            }
        }
    }
}