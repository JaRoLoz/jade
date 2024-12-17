use std::{path::PathBuf, process::Command};

use crate::{builder::build_step::BuildStep, logger};

pub const DEFAULT_PACKAGE_MANAGER: &str = "npm";

#[derive(Debug)]
pub struct JSBuildStep {
    pub name: String,
    pub package_manager: String,
    pub folder: PathBuf,
    pub build_script: String,
    pub install_packages: bool,
}

impl BuildStep for JSBuildStep {
    fn build(&self, resource_name: &String) {
        let path = dunce::canonicalize(&self.folder).unwrap();

        if self.install_packages {
            logger::log_info(
                format!(
                    "[{}/{}] Installing dependencies with {}",
                    resource_name, &self.name, &self.package_manager
                )
                .as_str(),
            );
            match Command::new(&self.package_manager)
                .current_dir(&path)
                .arg("install")
                .output()
            {
                Ok(_) => {}
                Err(error) => {
                    logger::log_error(
                        format!(
                            "[{}/{}] Failed to install dependencies with {}: {}",
                            resource_name, &self.name, &self.package_manager, error
                        )
                        .as_str(),
                    );
                    return;
                }
            }
        }

        logger::log_info(
            format!(
                "[{}/{}] Running \"{}\" with {}",
                resource_name, &self.name, &self.build_script, &self.package_manager
            )
            .as_str(),
        );

        match Command::new(&self.package_manager)
            .current_dir(&path)
            .arg("run")
            .arg(&self.build_script)
            .output()
        {
            Ok(_) => {}
            Err(error) => {
                logger::log_error(
                    format!(
                        "[{}/{}] Failed to run \"{}\" with {}: {}",
                        resource_name, &self.name, &self.build_script, &self.package_manager, error
                    )
                    .as_str(),
                );
                return;
            }
        }
    }
}

unsafe impl Sync for JSBuildStep {}
unsafe impl Send for JSBuildStep {}
