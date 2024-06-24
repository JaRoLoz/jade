use std::fmt::Debug;
use std::path::PathBuf;

pub trait BuildStep: Debug {
    fn build(&self, base_path: &PathBuf);
}