use std::{collections::HashMap, path::PathBuf};

pub fn find_resources_dir(current_path: &PathBuf) -> Option<PathBuf> {
    let mut current_path = current_path.clone();
    loop {
        if current_path.ends_with("resources") {
            return Some(current_path);
        }
        if !current_path.pop() {
            return None;
        }
    }
}

pub fn enumerate_resources(resources_path: &PathBuf) -> HashMap<String, PathBuf> {
    let mut resources = HashMap::new();

    for entry in resources_path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let name = entry.file_name().into_string().unwrap();
        if name.starts_with("[") && name.ends_with("]") {
            resources.extend(enumerate_resources(&path));
        } else {
            let contains_manifest = path
                .read_dir()
                .unwrap()
                .into_iter()
                .any(|dir_entry| {
                    let entry_name = dir_entry.unwrap().file_name();
                    entry_name == "fxmanifest.lua" || entry_name == "__resource.lua"
                });
            if contains_manifest {
                resources.insert(name, path);
            }
        }
    }

    resources
}