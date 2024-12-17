mod builder;
mod bundler;
mod js_builder;
mod logger;
mod manifest_generator;
mod parallel_builder;
mod path_resolver;

use builder::build_config::BuildConfig;
use clap::{arg, command, Arg};
use js_builder::DEFAULT_PACKAGE_MANAGER;
use path_resolver::{
    enumerate_buildable_resources, find_resources_dir, get_build_config_file,
    is_dir_a_buildable_resource,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
// use std::time::SystemTime;

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

fn main() {
    let matches = command!()
        .arg(arg!(<RESOURCE> "Resource(s) to build").required(false))
        .arg(
            Arg::new("ENVIRONMENT")
                .long("env")
                .required(false)
                .value_name("ENVIRONMENT")
                .help("Selects the type of config file to build"),
        )
        .arg(
            Arg::new("PACKAGE_MANAGER")
                .long("package-manager")
                .required(false)
                .value_name("PACKAGE_MANAGER")
                .help(format!(
                    "Selects the package manager to use (default is '{}')",
                    DEFAULT_PACKAGE_MANAGER
                )),
        )
        .get_matches();

    println!("{}", ASCII_LOGO);

    let resource = matches.get_one::<String>("RESOURCE");
    let environment = matches.get_one::<String>("ENVIRONMENT");
    let package_manager: &String = match matches.get_one::<String>("PACKAGE_MANAGER") {
        Some(package_manager) => package_manager,
        None => &DEFAULT_PACKAGE_MANAGER.to_string(),
    };

    let current_path = PathBuf::from(".").canonicalize().unwrap();
    let resources_path = find_resources_dir(&current_path);
    if let None = resources_path {
        logger::log_error("Could not find resources directory!");
        exit(1);
    }

    let resources = match resource {
        // If no resource is specified, build all resources
        None => enumerate_buildable_resources(&resources_path.unwrap(), environment),
        Some(resource) => {
            if resource != "." {
                // If the resource to be build is not the current directory, build it from the resources directory
                let build_resource =
                    enumerate_buildable_resources(&resources_path.unwrap(), environment)
                        .into_iter()
                        .find(|(name, _)| name == resource)
                        .unwrap();
                let mut hash_map = HashMap::new();
                hash_map.insert(build_resource.0.clone(), build_resource.1.clone());
                hash_map
            } else {
                // If the resource to be build is the current directory, build it from the current directory
                let build_config_file = get_build_config_file(environment);
                if is_dir_a_buildable_resource(&current_path, &build_config_file) {
                    let mut hash_map = HashMap::new();
                    let resource_name = current_path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    hash_map.insert(resource_name, current_path.join(&build_config_file));
                    hash_map
                } else {
                    logger::log_error("Could not find buildable resource!");
                    exit(1);
                }
            }
        }
    };

    let buildable_resources: Vec<Arc<BuildConfig>> = resources
        .iter()
        .filter_map(|(resource_name, resource_path)| {
            match BuildConfig::new(
                resource_name.clone(),
                resource_path.clone(),
                package_manager,
            ) {
                Ok(config) => Some(Arc::new(config)),
                Err(_) => {
                    logger::log_warn(
                        format!(
                            "Failed to parse build file for resource '{}'",
                            resource_name
                        )
                        .as_str(),
                    );
                    None
                }
            }
        })
        .collect();

    logger::log_info(format!("Found {} resource(s) to build", buildable_resources.len()).as_str());

    let start_time = std::time::Instant::now();

    let threads: Vec<JoinHandle<()>> = buildable_resources
        .into_iter()
        .map(|resource| thread::spawn(move || resource.build()))
        .collect();

    threads
        .into_iter()
        .for_each(|handle| handle.join().unwrap());

    let duration = start_time.elapsed().as_secs_f64();
    logger::log_success(format!("Build finished in {:.2}s!", duration).as_str());
}
