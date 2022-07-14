use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const KITIN_PROJECT_FILE: &str = "kitin.yaml";

#[derive(Serialize, Deserialize, Debug)]
struct KitinModule {
    source: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KitinProject {
    name: Option<String>,
    modules: Option<std::collections::HashMap<String, KitinModule>>,
}

impl KitinProject {
    pub fn new() -> KitinProject {
        KitinProject {
            name: None,
            modules: None,
        }
    }

    // loads a .kit file from the current working directory
    pub fn load_from_file(&mut self) {
        let path = std::path::Path::new(KITIN_PROJECT_FILE);
        if path.exists() {
            let mut file = std::fs::File::open(path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let data: KitinProject = serde_yaml::from_str(&contents).unwrap();
            self.name = data.name;
            self.modules = data.modules;
        }
    }

    // checks if a directory has a .kit file, and subsequently a kit project.
    pub fn directory_has_project(directory: &std::path::Path) -> bool {
        return directory
            .join(std::path::Path::new(KITIN_PROJECT_FILE))
            .exists();
    }

    pub fn save_to_file(&self, directory: &std::path::Path) {
        let mut file = std::fs::File::create(directory.join(KITIN_PROJECT_FILE)).unwrap();
        let data = serde_yaml::to_string(&self).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }
}
