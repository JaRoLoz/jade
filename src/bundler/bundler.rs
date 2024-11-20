use super::bundle_step::BundleStep;
use core::panic;
use regex::Regex;
use relative_path::RelativePathBuf;
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

const FILE_SEPARATOR: &str = ".";
const BUNDLE_BOILERPLATE: &str = "--[[Generated with Jade (https://github.com/JaRoLoz)]]
local ____bundle__dict, ____bundle__cache = {}, {}
require = function(module)
    if ____bundle__cache[module] then
        return ____bundle__cache[module]
    end
    local module_func = ____bundle__dict[module]()
    ____bundle__cache[module] = module_func
    return module_func
end
";

#[derive(Debug, Clone)]
struct DependencyNode {
    path: PathBuf,
}

pub struct Bundler<'a> {
    config: &'a BundleStep,
    main_node: Option<DependencyNode>,
    modules: HashMap<String, DependencyNode>,
}

impl DependencyNode {
    pub fn new(path: PathBuf) -> DependencyNode {
        DependencyNode { path }
    }

    pub fn get_contents(&self) -> String {
        std::fs::read_to_string(&self.path).unwrap()
    }

    pub fn scan_dependencies(
        &self,
        base_path: &PathBuf,
        dependencies: &mut HashMap<String, DependencyNode>,
    ) {
        let require_regex = Regex::new(r#"require\("([^"]+)"\)"#).unwrap();
        for capture in require_regex.captures_iter(self.get_contents().as_str()) {
            let module_name = capture.get(1).unwrap().as_str();
            if dependencies.contains_key(module_name) {
                continue;
            }

            let module_path = RelativePathBuf::from(module_name.replace(FILE_SEPARATOR, "/"))
                .normalize()
                .to_logical_path(base_path)
                .with_extension("lua");

            if !module_path.exists() {
                panic!(
                    "Module not found: {} ({})",
                    module_name,
                    module_path.display()
                );
            }

            let module = DependencyNode::new(module_path);
            dependencies.insert(module_name.to_string(), module.clone());
            module.scan_dependencies(base_path, dependencies);
        }
    }
}

impl<'a> Bundler<'a> {
    pub fn new(config: &BundleStep) -> Bundler {
        Bundler {
            config,
            main_node: None,
            modules: HashMap::new(),
        }
    }

    pub fn bundle(&mut self, base_path: &PathBuf) {
        let main_node = DependencyNode::new(self.config.entrypoint.clone());
        main_node.scan_dependencies(&self.config.source_dir, &mut self.modules);
        self.main_node = Some(main_node);
    }

    pub fn write_bundle(&self, out_file: &mut File) {
        out_file.write_all(BUNDLE_BOILERPLATE.as_bytes()).unwrap();
        for (name, module) in &self.modules {
            let content = format!(
                "____bundle__dict[\"{}\"] = function()\n{}\nend\n",
                name,
                module.get_contents()
            );
            out_file.write_all(content.as_bytes()).unwrap();
        }
        out_file
            .write_all(&self.main_node.as_ref().unwrap().get_contents().as_bytes())
            .unwrap();
    }
}
