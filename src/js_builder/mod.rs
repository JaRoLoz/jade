use std::{path::PathBuf, process::Command};

use json::JsonValue;
use crate::{builder::build_step::BuildStep, logger};

pub const DEFAULT_PACKAGE_MANAGER: &str = "npm";

#[derive(Debug)]
pub struct JSBuildStep {
    name: String,
    package_manager: String,
    folder: PathBuf
}

impl JSBuildStep {
    pub fn new(base_path: &PathBuf, raw_json: &JsonValue) -> Result<JSBuildStep, ()> {
        let name = match raw_json["name"].as_str() {
            Some(name) => name.to_string(),
            None => {
                logger::log_error(format!("Name not found for build config! ({})", base_path.display()).as_str());
                return Err(());
            }
        };

        let folder = match raw_json["folder"].as_str() {
            Some(folder) => PathBuf::from(folder),
            None => {
                logger::log_error(format!("Folder not found in build config '{}'", name).as_str());
                return Err(());
            }
        };

        let package_manager = match raw_json["package_manager"].as_str() {
            Some(package_manager) => package_manager.to_string(),
            None => {
                logger::log_warn(format!("Package manager not found in build config '{}', defaulting to '{}'", name, DEFAULT_PACKAGE_MANAGER).as_str());
                DEFAULT_PACKAGE_MANAGER.to_string()
            }
        };

        Ok(JSBuildStep {
            name,
            package_manager,
            folder: base_path.join(folder)
        })
    }
}

impl BuildStep for JSBuildStep {
    fn build(&self, base_path: &PathBuf) {
        logger::log_info(format!("├───• Building '{}' using '{}'", self.name, self.package_manager).as_str());
        let dir = base_path.join(&self.folder);
        let dir = match dunce::canonicalize(dir) {
            Ok(dir) => dir,
            Err(error) => {
                logger::log_error(format!("├───• Failed to resolve path for '{}': {}", self.name, error).as_str());
                return;
            }
        };
        match Command::new(&self.package_manager)
            .current_dir(&dir)
            .arg("install")
            .output() {
                Ok(_) => {},
                Err(error) => {
                    logger::log_error(format!("├───• Failed to install dependencies for '{}': {}", self.name, error).as_str());
                    return;
                }
            }
        
        match Command::new(&self.package_manager)
            .current_dir(&dir)
            .arg("run")
            .arg("build")
            .output() {
                Ok(_) => {},
                Err(error) => {
                    logger::log_error(format!("├───• Failed to build '{}': {}", self.name, error).as_str());
                    return;
                }
            }
    }
}