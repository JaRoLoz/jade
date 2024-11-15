use super::bundler::Bundler;
use crate::{builder::build_step::BuildStep, logger};
use json::JsonValue;
use relative_path::RelativePathBuf;
use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

#[derive(Debug)]
pub struct BundleStep {
    pub name: String,
    pub encrypt: bool,
    pub output: PathBuf,
    pub source_dir: PathBuf,
    pub entrypoint: PathBuf,
}

impl BundleStep {
    pub fn new(base_path: &PathBuf, raw_json: &JsonValue) -> Result<BundleStep, ()> {
        let name = match raw_json["name"].as_str() {
            Some(name) => name.to_string(),
            None => {
                logger::log_error(
                    format!(
                        "Name not found for bundle config! ({})",
                        base_path.display()
                    )
                    .as_str(),
                );
                return Err(());
            }
        };

        let output = match raw_json["output"].as_str() {
            Some(path) => RelativePathBuf::from(path)
                .normalize()
                .to_logical_path(base_path),
            None => {
                logger::log_error(format!("Output path not found in '{}' config", name).as_str());
                return Err(());
            }
        };

        let source_dir = match raw_json["source_dir"].as_str() {
            Some(path) => RelativePathBuf::from(path)
                .normalize()
                .to_logical_path(base_path),
            None => {
                logger::log_error("Source directory not found in bundle config");
                return Err(());
            }
        };

        let entrypoint = match raw_json["entrypoint"].as_str() {
            Some(path) => RelativePathBuf::from(path)
                .normalize()
                .to_logical_path(base_path),
            None => {
                logger::log_error("Entrypoint not found in bundle config");
                return Err(());
            }
        };

        let encrypt = match raw_json["encrypt"].as_bool() {
            Some(encrypt) => encrypt,
            None => false,
        };

        Ok(BundleStep {
            name,
            encrypt,
            output,
            source_dir,
            entrypoint,
        })
    }
}

impl BuildStep for BundleStep {
    fn build(&self, base_path: &PathBuf) {
        logger::log_info(format!("├───• Bundling '{}'", self.name).as_str());
        let mut bundler = Bundler::new(self);
        bundler.bundle(&base_path);

        let out_file_path = base_path.join(&self.output);
        create_dir_all(out_file_path.parent().unwrap()).unwrap();
        let mut out_file = File::create(out_file_path).unwrap();

        bundler.write_bundle(&mut out_file);
    }
}
