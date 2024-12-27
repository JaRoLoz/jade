use crate::{builder::build_step::BuildStep, logger};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ManifestGenerationStep {
    pub path: PathBuf,
    pub fx_version: String,
    pub game: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub client_scripts: Vec<String>,
    pub server_scripts: Vec<String>,
    pub shared_scripts: Vec<String>,
    pub dependencies: Vec<String>,
    pub files: Vec<String>,
    pub loadscreen: Option<String>,
    pub ui_page: Option<String>,
    pub is_a_map: bool,
    pub lua54: bool,
    pub rdr3_warning: Option<String>,
}

impl BuildStep for ManifestGenerationStep {
    fn build(&self, resource_name: &String) {
        logger::log_info(
            format!("[{}/fx_manifest] Generating fxmanifest.lua", resource_name).as_str(),
        );
        let mut file = std::fs::File::create(&self.path).unwrap();

        writeln!(file, r#"fx_version "{}""#, self.fx_version).unwrap();
        writeln!(file, r#"game "{}""#, self.game).unwrap();

        if let Some(author) = &self.author {
            writeln!(file, r#"author "{}""#, author).unwrap();
        }
        if let Some(description) = &self.description {
            writeln!(file, r#"description "{}""#, description).unwrap();
        }
        if let Some(version) = &self.version {
            writeln!(file, r#"version "{}""#, version).unwrap();
        }

        if !self.client_scripts.is_empty() {
            writeln!(file, "client_scripts {{").unwrap();
            for script in &self.client_scripts {
                writeln!(file, r#"    "{}","#, script).unwrap();
            }
            writeln!(file, "}}").unwrap();
        }

        if !self.server_scripts.is_empty() {
            writeln!(file, "server_scripts {{").unwrap();
            for script in &self.server_scripts {
                writeln!(file, r#"    "{}","#, script).unwrap();
            }
            writeln!(file, "}}").unwrap();
        }

        if !self.shared_scripts.is_empty() {
            writeln!(file, "shared_scripts {{").unwrap();
            for script in &self.shared_scripts {
                writeln!(file, r#"    "{}","#, script).unwrap();
            }
            writeln!(file, "}}").unwrap();
        }

        if let Some(ui_page) = &self.ui_page {
            writeln!(file, r#"ui_page "{}""#, ui_page).unwrap();
        }

        if !self.files.is_empty() {
            writeln!(file, "files {{").unwrap();
            for file_name in &self.files {
                writeln!(file, r#"    "{}","#, file_name).unwrap();
            }
            writeln!(file, "}}").unwrap();
        }

        if let Some(loadscreen) = &self.loadscreen {
            writeln!(file, r#"loadscreen "{}""#, loadscreen).unwrap();
        }

        if !self.dependencies.is_empty() {
            writeln!(file, "dependencies {{").unwrap();
            for dependency in &self.dependencies {
                writeln!(file, r#"    "{}","#, dependency).unwrap();
            }
            writeln!(file, "}}").unwrap();
        }

        if self.is_a_map {
            writeln!(file, r#"this_is_a_map "yes""#).unwrap();
        }

        if self.lua54 {
            writeln!(file, r#"lua54 "yes""#).unwrap();
        }

        if let Some(rdr3_warning) = &self.rdr3_warning {
            writeln!(file, r#"rdr3_warning "{}""#, rdr3_warning).unwrap();
        }

        writeln!(file).unwrap();
    }
}
