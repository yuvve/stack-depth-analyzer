use gimli::{
    AttributeValue,
    DW_AT_name,
    DW_TAG_subprogram,
    DebugAbbrev,
    DebugInfo,
    DebugStr,
    RunTimeEndian,
};
use object::{Object, ObjectSection};
use std::fs;
use regex::Regex;

pub trait ParseElf{
    fn get_entry_points(&self, elf_path: &str) -> Result<Vec<String>, String>;
}

pub struct ParsedElf;

impl ParseElf for ParsedElf {
    fn get_entry_points(&self, elf_path: &str) -> Result<Vec<String>, String> {
        get_entry_points(elf_path)
    }
}

// Hacky solution to remove main entry points
const KNOWN_MAIN_ENTRY_POINTS: [&str; 3] = [
    "main",
    "__cortex_m_rt_main",
    "__cortex_m_rt_main_trampoline"
];

fn get_entry_points(elf_path: &str) -> Result<Vec<String>, String> {
    let crate_namespace = elf_path.split("/").last().unwrap().split(".").next().unwrap();
    let mut entry_points = get_subprograms_in_namespace(elf_path, crate_namespace).unwrap();

    // Remove entry points matching known main entry points
    for main_entry_point in KNOWN_MAIN_ENTRY_POINTS.iter() {
        let re = Regex::new(&format!("^{}$", main_entry_point)).unwrap();
        entry_points = entry_points.iter().filter(|entry_point| !re.is_match(entry_point)).map(|x| x.to_string()).collect();
    }
    
    Ok(entry_points)
}

fn get_subprograms_in_namespace(elf_path: &str, crate_namespace: &str) -> Result<Vec<String>, String> {
    let file = fs::read(elf_path).expect("Unable to open file.");
    let data: &[u8] = file.as_ref();
    let obj_file = match object::File::parse(data) {
        Ok(x) => x,
        Err(e) => {
            return Err(format!("Failed to parse ELF file: {}", e));
        }
    };
    let gimli_endian = if obj_file.is_little_endian() {
        RunTimeEndian::Little
    } else {
        RunTimeEndian::Big
    };

    let debug_info = obj_file.section_by_name(".debug_info").unwrap();
    let debug_info = DebugInfo::new(debug_info.data().unwrap(), gimli_endian);

    let debug_abbrev = obj_file.section_by_name(".debug_abbrev").unwrap();
    let debug_abbrev = DebugAbbrev::new(debug_abbrev.data().unwrap(), gimli_endian);

    let debug_str = obj_file.section_by_name(".debug_str").unwrap();
    let debug_str = DebugStr::new(debug_str.data().unwrap(), gimli_endian);

    let mut subprograms = Vec::new();

    let mut units = debug_info.units();
    while let Some(unit) = units.next().unwrap() {
        let abbrev = unit.abbreviations(&debug_abbrev).unwrap();
        let mut cursor = unit.entries(&abbrev);

        while let Some((_dept, entry)) = cursor.next_dfs().unwrap() {
            let tag = entry.tag();
            if tag != gimli::DW_TAG_namespace {
                continue;
            }
            let attr = match entry.attr_value(DW_AT_name).unwrap() {
                Some(a) => a,
                None => continue,
            };
            let entry_name = match attr {
                AttributeValue::DebugStrRef(s) => s,
                _ => continue,
            };

            let entry_name = debug_str.get_str(entry_name).unwrap();
            let name_str = entry_name.to_string().unwrap();
            
            if name_str != crate_namespace {
                continue;
            }
            
            search_children(debug_str, &mut subprograms, &abbrev, entry, unit);
        }
    }
    Ok(subprograms)
}

fn search_children(debug_str: DebugStr<gimli::EndianSlice<'_, RunTimeEndian>>, subprograms: &mut Vec<String>, abbrev: &gimli::Abbreviations, entry: &gimli::DebuggingInformationEntry<'_, '_, gimli::EndianSlice<'_, RunTimeEndian>, usize>, unit: gimli::UnitHeader<gimli::EndianSlice<'_, RunTimeEndian>, usize>) {
    let mut child_entries = unit.entries_tree(abbrev,Some(entry.offset())).unwrap();
    let root = child_entries.root().unwrap();
    let mut children = root.children();
    while let Some(child) = children.next().unwrap() {
        let child_entry = child.entry();
        if child_entry.tag() != DW_TAG_subprogram {
            continue;
        }

        // Skip internal functions
        match child_entry.attr_value(gimli::DW_AT_external) {
            Ok(Some(AttributeValue::Flag(true))) => (),
            _ => continue,
        }

        if let Some(name_attr) = child_entry.attr(DW_AT_name).unwrap() {
            if let AttributeValue::DebugStrRef(str_ref) = name_attr.value() {
                let function_name = debug_str.get_str(str_ref).unwrap().to_string_lossy();
                subprograms.push(function_name.to_string());
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::get_entry_points;
    use std::path::PathBuf;

    struct ElfTestData {
        path: String, 
        entries: Vec<&'static str>, 
    }
    
    fn get_test_data() -> Vec<ElfTestData> {
        let root_path = env!("CARGO_MANIFEST_DIR").to_owned();
        let assets_path = PathBuf::from(root_path).join("tests").join("assets");
    
        vec![
            ElfTestData {
                path: assets_path.join("program1").to_str().unwrap().to_string(), 
                entries: vec![
                    "function1",
                    "function2",
                    "function3",
                    "function4"
                ],
            },
            ElfTestData {
                path: assets_path.join("ex4").to_str().unwrap().to_string(), 
                entries: vec![
                    "equal_formula_rec",
                    "equal_iter_rec",
                    "complexity_sum_recursive",
                    "complexity_sum_iterative",
                    "complexity_sum_formula"
                ],
            },
        ]
    }

    #[test]
    fn test_get_entry_points() {
        let test_elf = get_test_data();
        for elf in test_elf {
            let entries = get_entry_points(&elf.path);
            assert!(entries.is_ok());
            let entries = entries.unwrap();
            assert_eq!(entries.len(), elf.entries.len());
            for entry in elf.entries {
                assert!(entries.contains(&entry.to_string()), "Entry {} not found", entry);
            }
        }
    }
}
