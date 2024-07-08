mod logger;
mod path_resolver;
mod builder;
mod bundler;
mod js_builder;
mod manifest_generator;

use std::path::PathBuf;
use clap::{arg, command, value_parser};
use std::time::SystemTime;
use builder::{build_config::BuildOptions, builder::Builder};

const ASCII_LOGO: &str = r#"
⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⣿⣿⣦⣄⠀⠀⠀⠀⣠⡞⠀⠀⠀⣠⣴⣶⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⡛⢿⣿⣷⣄⢀⣾⠏⣀⣤⣶⣿⢿⣿⡏⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠑⢤⣀⡀⠀⢻⣷⡄⠻⣿⣿⣿⣿⣿⣿⡿⢋⣵⣿⠟⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠉⠛⢷⣿⣿⣿⣤⣼⠿⢿⣿⡟⢁⣴⣾⡿⠋⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢀⣤⣾⣿⣿⣿⠿⣿⠋⠀⠀⠀⢹⣿⣿⣿⣿⣿⠶⠶⠤⣄⡀⠀⠀
⠀⠀⠀⣠⣾⡿⠿⢛⣋⣉⣤⣤⣽⣷⣤⣤⣶⣟⣁⠉⠛⠿⣿⣷⣦⡀⠀⠀⠀⠀
⠀⠐⠻⠿⠿⠿⠿⠿⠿⠿⣿⣿⣿⣿⠃⢰⣿⣿⣿⣿⣷⣦⣬⣙⣿⣿⣄⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣼⢿⣿⣿⠀⣿⣿⣿⣿⡉⠉⠉⠛⠻⠿⣿⣿⣦⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⢠⡾⠁⢸⣿⡏⢸⣿⣿⠇⠙⢿⡄⠀⠀⠀⠀⠀⠀⠉⠁⠀
⠀⠀⠀⠀⠀⠀⠀⢀⠏⠀⠀⢸⣿⣧⣿⣿⠏⠀⠀⠈⠻⣆⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣿⡿⠃⠀⠀⠀⠀⠀⠈⠄⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⡟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"#;


fn build_all_resources(options: BuildOptions) {
    let current_path = PathBuf::from(".").canonicalize().unwrap();
    let resources_dir = path_resolver::find_resources_dir(&current_path).unwrap();
    let resources = path_resolver::enumerate_resources(&resources_dir);
    let builder = Builder::new(resources, options);
    logger::log_info(format!("Found {} buildable resources!", builder.len()).as_str());
    logger::log_info("Starting build process...\n");
    builder.build();
}

fn build_resource(resource: &String, options: BuildOptions) {
    let current_path = PathBuf::from(".").canonicalize().unwrap();
    let resources_dir = path_resolver::find_resources_dir(&current_path).unwrap();
    let resources = path_resolver::enumerate_resources(&resources_dir);
    let builder = Builder::new(resources, options);
    builder.build_resource(resource);
}

fn main() {
    let matches = command!()
        .arg(arg!(<RESOURCE> "Resource(s) to build").required(false))
        .arg(arg!(--env <ENVIRONMENT> "Selects the type of config file to build").required(false).value_parser(value_parser!(String)))
        .arg(arg!(--bundle "Only executes the bundle step for every build").required(false))
        .arg(arg!(--js_build "Only executes the JS build step for every build").required(false))
        .arg(arg!(--manifest "Only executes the fxmanifest generation build step").required(false))
        .get_matches();

    let mut options = BuildOptions::default();
    options.env = matches.get_one::<String>("env");
    let only_bundle = matches.get_flag("bundle");
    let only_js_build = matches.get_flag("js_build");
    let only_manifest = matches.get_flag("manifest");

    if only_bundle || only_js_build || only_manifest {
        options.bundle = only_bundle;
        options.js_build = only_js_build;
        options.manifest = only_manifest;
    }

    println!("{}", ASCII_LOGO);
    let start = SystemTime::now();

    let build_target = matches.get_one::<String>("RESOURCE");
    match build_target {
        Some(resource) => {
            if (resource == "all") || (resource == "*") {
                build_all_resources(options);
            } else if resource == "." {
                let current_path = PathBuf::from(".").canonicalize().unwrap();
                let current_dir = current_path.file_name().unwrap().to_str().unwrap().to_string();
                build_resource(&current_dir, options);
            } else {
                build_resource(&resource, options);
            }
        }
        None => build_all_resources(options)
    }

    logger::log_success(format!("Build finished in {:.2}s!", start.elapsed().unwrap().as_secs_f32()).as_str());
}
