use std::path::PathBuf;
use crate::logger;
use crate::builder::build_step::BuildStep;
use crate::bundler::bundle_step::BundleStep;
use crate::js_builder::JSBuildStep;

pub const BUILD_CONFIG_FILE: &str = "jade.json";

pub struct BuildOptions {
    pub bundle: bool,
    pub js_build: bool
}

#[derive(Debug)]
pub struct BuildConfig {
    pub name: String,
    base_path: PathBuf,
    steps: Vec<Box<dyn BuildStep>>
}

impl Default for BuildOptions {
    fn default() -> Self {
        BuildOptions {
            bundle: true,
            js_build: true
        }
    }
}

impl BuildConfig {
    pub fn new(name: String, path: PathBuf, options: &BuildOptions) -> Result<BuildConfig, ()> {
        let build_config_string = match std::fs::read_to_string(path.join(BUILD_CONFIG_FILE)) {
            Ok(string) => string,
            Err(_) => {
                logger::log_error("Failed to read build config file");
                return Err(());
            }
        };

        let build_config_object = match json::parse(build_config_string.as_str()) {
            Ok(object) => object,
            Err(_) => {
                logger::log_error("Failed to parse build config file");
                return Err(());
            }
        };

        let mut steps: Vec<Box<dyn BuildStep>> = Vec::new();

        if options.bundle {
            match build_config_object["bundle"].is_array() {
                true => {
                    for bundle in build_config_object["bundle"].members() {
                        match BundleStep::new(&path, bundle) {
                            Ok(config) => {
                                steps.push(Box::new(config));
                            }
                            Err(_) => continue
                        }
                    }
                }
                false => ()
            };
        }

        if options.js_build {
            match build_config_object["js_build"].is_array() {
                true => {
                    for ui_build in build_config_object["js_build"].members() {
                        match JSBuildStep::new(&path, ui_build) {
                            Ok(config) => {
                                steps.push(Box::new(config));
                            }
                            Err(_) => continue
                        }
                    }
                }
                false => ()
            };
        }

        if steps.is_empty() {
            logger::log_warn(format!("'{}' does not contain any build steps!", name).as_str());
        }

        Ok(BuildConfig {
            name,
            base_path: path,
            steps
        })
    }

    pub fn build(&self) {
        logger::log_info(format!("┌ Starting build for '{}'", self.name).as_str());
        for step in &self.steps {
            step.build(&self.base_path);
        }
        logger::log_success(format!("└ Successfully built resource '{}'!\n", self.name).as_str());
    }
}
