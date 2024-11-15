use crate::{builder::build_step::BuildStep, logger};
use json::JsonValue;
use std::io::Write;
use std::path::PathBuf;

const REQUIRED_MANIFEST_FIELDS: [&str; 2] = ["fx_version", "game"];

#[derive(Debug)]
pub struct ManifestGenerationStep {
    fx_version: String,
    game: String,
    author: Option<String>,
    description: Option<String>,
    version: Option<String>,
    client_scripts: Vec<String>,
    server_scripts: Vec<String>,
    shared_scripts: Vec<String>,
    dependencies: Vec<String>,
    files: Vec<String>,
    loadscreen: Option<String>,
    ui_page: Option<String>,
    is_a_map: bool,
    lua54: bool,
    rdr3_warning: Option<String>,
}

impl ManifestGenerationStep {
    pub fn new(_base_path: &PathBuf, raw_json: &JsonValue) -> Result<ManifestGenerationStep, ()> {
        if !raw_json.is_object() {
            logger::log_error("├───• Wrong format for manifest config file (Expected JSON object)");
            return Err(());
        }

        for field in REQUIRED_MANIFEST_FIELDS.iter() {
            if !raw_json.has_key(field) || !raw_json[*field].is_string() {
                logger::log_error(
                    format!("├───• Missing field '{}' in manifest config", field).as_str(),
                );
                return Err(());
            }
        }

        let fx_version = raw_json["fx_version"].to_string();
        let game = raw_json["game"].to_string();

        let author = match raw_json["author"].as_str() {
            Some(author) => Some(author.to_string()),
            None => None,
        };

        let description = match raw_json["description"].as_str() {
            Some(description) => Some(description.to_string()),
            None => None,
        };

        let version = match raw_json["version"].as_str() {
            Some(version) => Some(version.to_string()),
            None => None,
        };

        let client_scripts = match raw_json["client_scripts"].is_array() {
            true => raw_json["client_scripts"]
                .members()
                .map(|script| script.to_string())
                .collect(),
            false => Vec::new(),
        };

        let server_scripts = match raw_json["server_scripts"].is_array() {
            true => raw_json["server_scripts"]
                .members()
                .map(|script| script.to_string())
                .collect(),
            false => Vec::new(),
        };

        let shared_scripts = match raw_json["shared_scripts"].is_array() {
            true => raw_json["shared_scripts"]
                .members()
                .map(|script| script.to_string())
                .collect(),
            false => Vec::new(),
        };

        let dependencies = match raw_json["dependencies"].is_array() {
            true => raw_json["dependencies"]
                .members()
                .map(|script| script.to_string())
                .collect(),
            false => Vec::new(),
        };

        let files = match raw_json["files"].is_array() {
            true => raw_json["files"]
                .members()
                .map(|script| script.to_string())
                .collect(),
            false => Vec::new(),
        };

        let loadscreen = match raw_json["loadscreen"].as_str() {
            Some(loadscreen) => Some(loadscreen.to_string()),
            None => None,
        };

        let ui_page = match raw_json["ui_page"].as_str() {
            Some(ui_page) => Some(ui_page.to_string()),
            None => None,
        };

        let is_a_map = match raw_json["is_a_map"].as_bool() {
            Some(is_a_map) => is_a_map,
            None => false,
        };

        let lua54 = match raw_json["lua54"].as_bool() {
            Some(lua54) => lua54,
            None => false,
        };

        let rdr3_warning = match raw_json["rdr3_warning"].as_str() {
            Some(rdr3_warning) => Some(rdr3_warning.to_string()),
            None => None,
        };

        Ok(ManifestGenerationStep {
            fx_version,
            game,
            author,
            description,
            version,
            client_scripts,
            server_scripts,
            shared_scripts,
            dependencies,
            files,
            loadscreen,
            ui_page,
            is_a_map,
            lua54,
            rdr3_warning,
        })
    }
}

impl BuildStep for ManifestGenerationStep {
    fn build(&self, base_path: &PathBuf) {
        logger::log_info("├───• Generating fxmanifest.lua");
        let file_path = base_path.join("fxmanifest.lua");
        let mut file = std::fs::File::create(file_path).unwrap();

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
