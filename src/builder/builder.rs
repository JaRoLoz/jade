use std::{collections::HashMap, ffi::OsString, path::PathBuf};
use crate::builder::build_config::{BuildConfig, BUILD_CONFIG_FILE};
use super::build_config::BuildOptions;

#[derive(Debug)]
pub struct Builder {
    configs: Vec<BuildConfig>
}

impl Builder {
    pub fn new(resources: HashMap<String, PathBuf>, options: BuildOptions) -> Builder {
        let mut configs = Vec::new();

        let config_file_name = match options.env {
            Some(env) => format!("{}.{}", env, BUILD_CONFIG_FILE),
            None => BUILD_CONFIG_FILE.to_string()
        };

        for (name, path) in resources {
            let is_buildable = path
                .read_dir()
                .unwrap()
                .into_iter()
                .any(|dir_entry| {
                    let entry_name: OsString = dir_entry.unwrap().file_name();
                    entry_name.to_str().unwrap() == config_file_name || entry_name.to_str().unwrap() == BUILD_CONFIG_FILE
                });
            if is_buildable {
                let config = match BuildConfig::new(name, path, &options) {
                    Ok(config) => config,
                    Err(_) => continue
                };
                configs.push(config);
            }
        }

        Builder { configs }
    }

    pub fn build(&self) {
        for config in &self.configs {
            config.build();
        }
    }

    pub fn build_resource(&self, resource: &String) {
        for config in &self.configs {
            if config.name == *resource {
                config.build();
                return;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.configs.len()
    }
}