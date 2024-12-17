use super::bundler::Bundler;
use crate::{builder::build_step::BuildStep, logger};
use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

#[derive(Debug)]
pub struct BundleStep {
    pub name: String,
    pub output: PathBuf,
    pub source_dir: PathBuf,
    pub entrypoint: PathBuf,
}

impl BuildStep for BundleStep {
    fn build(&self, resource_name: &String) {
        logger::log_info(format!("[{}/{}] Bundling lua", resource_name, &self.name).as_str());
        let bundler = Bundler::new(self);

        create_dir_all(&self.output.parent().unwrap()).unwrap();
        let mut out_file = File::create(&self.output).unwrap();

        bundler.write_bundle(&mut out_file);
    }
}

unsafe impl Sync for BundleStep {}
unsafe impl Send for BundleStep {}
