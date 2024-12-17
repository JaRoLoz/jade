use crate::builder::build_config::BUILD_CONFIG_FILE;
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

pub fn is_dir_a_buildable_resource(path: &PathBuf, build_config_file: &String) -> bool {
    let contains_build_config = path.read_dir().unwrap().into_iter().any(|dir_entry| {
        let entry_name = dir_entry.unwrap().file_name();
        entry_name.into_string().unwrap() == *build_config_file
    });
    contains_build_config
}

pub fn get_build_config_file(env: Option<&String>) -> String {
    match env {
        Some(env) => format!("{}.{}", env, BUILD_CONFIG_FILE),
        None => BUILD_CONFIG_FILE.to_string(),
    }
}

pub fn enumerate_buildable_resources(
    resources_path: &PathBuf,
    build_env: Option<&String>,
) -> HashMap<String, PathBuf> {
    let mut resources = HashMap::new();
    let build_config_file = get_build_config_file(build_env);

    for entry in resources_path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let name = entry.file_name().into_string().unwrap();
        if name.starts_with("[") && name.ends_with("]") {
            resources.extend(enumerate_buildable_resources(&path, build_env));
        } else {
            if is_dir_a_buildable_resource(&path, &build_config_file) {
                resources.insert(name, path.join(&build_config_file));
            }
        }
    }

    resources
}
