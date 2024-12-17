use crate::builder::build_step::BuildStep;
use crate::bundler::bundle_step::BundleStep;
use crate::js_builder::JSBuildStep;
use crate::logger;
use crate::manifest_generator::ManifestGenerationStep;
use crate::parallel_builder::ParallelBuildStep;
use relative_path::RelativePathBuf;
use std::path::PathBuf;
use std::sync::Arc;

pub const BUILD_CONFIG_FILE: &str = "jade.xml";

#[derive(Debug)]
pub struct BuildConfig {
    pub name: String,
    steps: Vec<Box<dyn BuildStep + Send + Sync>>,
}

fn parse_js_build(
    node: &roxmltree::Node,
    path: &PathBuf,
    default_package_manager: &String,
) -> Box<dyn BuildStep + Send + Sync> {
    let name = node.attribute("name").unwrap();
    let folder = node
        .children()
        .find(|n| n.tag_name().name() == "folder")
        .unwrap()
        .text()
        .unwrap();
    let build_script = node
        .children()
        .find(|n| n.tag_name().name() == "build_script")
        .unwrap()
        .text()
        .unwrap();
    let install_packages = match node
        .children()
        .find(|n| n.tag_name().name() == "install_packages")
    {
        None => true,
        Some(n) => n.text().unwrap() == "true",
    };
    let package_manager = match node
        .children()
        .find(|n| n.tag_name().name() == "package_manager")
    {
        None => default_package_manager,
        Some(n) => n.text().unwrap(),
    };

    Box::new(JSBuildStep {
        name: name.to_string(),
        build_script: build_script.to_string(),
        package_manager: package_manager.to_string(),
        install_packages,
        folder: RelativePathBuf::from(folder)
            .normalize()
            .to_logical_path(path),
    })
}

fn parse_bundle(node: &roxmltree::Node, path: &PathBuf) -> Box<dyn BuildStep + Send + Sync> {
    let name = node.attribute("name").unwrap();
    let entrypoint = node
        .children()
        .find(|n| n.tag_name().name() == "entrypoint")
        .unwrap()
        .text()
        .unwrap();
    let source_dir = node
        .children()
        .find(|n| n.tag_name().name() == "source_dir")
        .unwrap()
        .text()
        .unwrap();
    let output = node
        .children()
        .find(|n| n.tag_name().name() == "output")
        .unwrap()
        .text()
        .unwrap();

    Box::new(BundleStep {
        name: name.to_string(),
        entrypoint: RelativePathBuf::from(entrypoint)
            .normalize()
            .to_logical_path(path)
            .with_extension("lua"),
        source_dir: RelativePathBuf::from(source_dir)
            .normalize()
            .to_logical_path(path),
        output: RelativePathBuf::from(output)
            .normalize()
            .to_logical_path(path)
            .with_extension("lua"),
    })
}

fn parse_manifest(node: &roxmltree::Node, path: &PathBuf) -> Box<dyn BuildStep + Send + Sync> {
    Box::new(ManifestGenerationStep {
        path: RelativePathBuf::from("./fxmanifest.lua")
            .normalize()
            .to_logical_path(path)
            .with_extension("lua"),
        fx_version: node
            .children()
            .find(|n| n.tag_name().name() == "fx_version")
            .unwrap()
            .text()
            .unwrap()
            .to_string(),
        game: node
            .children()
            .find(|n| n.tag_name().name() == "game")
            .unwrap()
            .text()
            .unwrap()
            .to_string(),
        author: match node.children().find(|n| n.tag_name().name() == "author") {
            None => None,
            Some(n) => Some(n.text().unwrap().to_string()),
        },
        description: match node
            .children()
            .find(|n| n.tag_name().name() == "description")
        {
            None => None,
            Some(n) => Some(n.text().unwrap().to_string()),
        },
        version: match node.children().find(|n| n.tag_name().name() == "version") {
            None => None,
            Some(n) => Some(n.text().unwrap().to_string()),
        },
        client_scripts: match node
            .children()
            .find(|n| n.tag_name().name() == "client_scripts")
        {
            None => Vec::new(),
            Some(n) => n
                .children()
                .filter(|n| n.tag_name().name() == "client_script")
                .map(|n| n.text().unwrap().to_string())
                .collect(),
        },
        server_scripts: match node
            .children()
            .find(|n| n.tag_name().name() == "server_scripts")
        {
            None => Vec::new(),
            Some(n) => n
                .children()
                .filter(|n| n.tag_name().name() == "server_script")
                .map(|n| n.text().unwrap().to_string())
                .collect(),
        },
        shared_scripts: match node
            .children()
            .find(|n| n.tag_name().name() == "shared_scripts")
        {
            None => Vec::new(),
            Some(n) => n
                .children()
                .filter(|n| n.tag_name().name() == "shared_script")
                .map(|n| n.text().unwrap().to_string())
                .collect(),
        },
        dependencies: match node
            .children()
            .find(|n| n.tag_name().name() == "dependencies")
        {
            None => Vec::new(),
            Some(n) => n
                .children()
                .filter(|n| n.tag_name().name() == "dependency")
                .map(|n| n.text().unwrap().to_string())
                .collect(),
        },
        files: match node.children().find(|n| n.tag_name().name() == "files") {
            None => Vec::new(),
            Some(n) => n
                .children()
                .filter(|n| n.tag_name().name() == "file")
                .map(|n| n.text().unwrap().to_string())
                .collect(),
        },
        loadscreen: match node
            .children()
            .find(|n| n.tag_name().name() == "loadscreen")
        {
            None => None,
            Some(n) => Some(n.text().unwrap().to_string()),
        },
        ui_page: match node.children().find(|n| n.tag_name().name() == "ui_page") {
            None => None,
            Some(n) => Some(n.text().unwrap().to_string()),
        },
        is_a_map: match node.children().find(|n| n.tag_name().name() == "is_a_map") {
            None => false,
            Some(n) => n.attribute("enable").unwrap() == "true",
        },
        lua54: match node.children().find(|n| n.tag_name().name() == "lua54") {
            None => false,
            Some(n) => n.attribute("enable").unwrap() == "true",
        },
        rdr3_warning: match node
            .children()
            .find(|n| n.tag_name().name() == "rdr3_warning")
        {
            None => None,
            Some(n) => Some(n.text().unwrap().to_string()),
        },
    })
}

fn parse_steps(
    root: &roxmltree::Node,
    path: &PathBuf,
    package_manager: &String,
) -> Vec<Box<dyn BuildStep + Send + Sync>> {
    root.children()
        .filter_map(|node| match node.tag_name().name() {
            "js_build" => Some(parse_js_build(&node, path, package_manager)),
            "bundle" => Some(parse_bundle(&node, path)),
            "manifest" => Some(parse_manifest(&node, path)),
            "parallel" => Some(Box::new(ParallelBuildStep {
                steps: parse_steps(&node, path, package_manager)
                    .into_iter()
                    .map(|step| Arc::new(step))
                    .collect(),
            })),
            _ => None,
        })
        .collect()
}

impl BuildConfig {
    pub fn new(
        name: String,
        build_file_path: PathBuf,
        package_manager: &String,
    ) -> Result<BuildConfig, ()> {
        let resource_path = PathBuf::from(build_file_path.parent().unwrap());

        let build_config_string = match std::fs::read_to_string(&build_file_path) {
            Ok(string) => string,
            Err(_) => match std::fs::read_to_string(resource_path.join(BUILD_CONFIG_FILE)) {
                Ok(string) => string,
                Err(_) => {
                    logger::log_error(
                        format!(
                            "Failed to read build config file {}",
                            build_file_path.display()
                        )
                        .as_str(),
                    );
                    return Err(());
                }
            },
        };

        let build_config = match roxmltree::Document::parse(&build_config_string) {
            Ok(config) => config,
            Err(_) => {
                logger::log_error(
                    format!(
                        "Failed to parse build config file {}",
                        build_file_path.display()
                    )
                    .as_str(),
                );
                return Err(());
            }
        };

        let steps: Vec<Box<dyn BuildStep + Send + Sync>> = parse_steps(
            &build_config.root_element(),
            &resource_path,
            package_manager,
        );

        if steps.is_empty() {
            logger::log_warn(
                format!("[{}] Build config does not contain any build steps!", name).as_str(),
            );
        }

        Ok(BuildConfig { name, steps })
    }

    pub fn build(&self) {
        let start_time = std::time::Instant::now();
        logger::log_info(format!("[{}] Starting build", &self.name).as_str());
        let _ = &self.steps.iter().for_each(|step| step.build(&self.name));
        let duration = start_time.elapsed().as_secs_f64();
        logger::log_success(
            format!("[{}] Built successfully in {:.2}s", &self.name, duration).as_str(),
        );
    }
}

unsafe impl Send for BuildConfig {}
unsafe impl Sync for BuildConfig {}
