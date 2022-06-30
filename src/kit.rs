use serde::{Deserialize, Serialize};
use std::io::Read;

const KIT_PROJECT_FILE: &str = ".kit";

#[derive(Serialize, Deserialize, Debug)]
struct KitModule {
    source: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KitProject {
    name: Option<String>,
    modules: Option<std::collections::HashMap<String, KitModule>>,
}

impl KitProject {
    pub fn new() -> KitProject {
        KitProject {
            name: None,
            modules: None,
        }
    }

    // loads a .kit file from the current woking directory
    pub fn load_from_file(&mut self) {
        let path = std::path::Path::new(KIT_PROJECT_FILE);
        if path.exists() {
            let mut file = std::fs::File::open(path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let data: KitProject = serde_json::from_str(&contents).unwrap();
            self.name = data.name;
            self.modules = data.modules;
        }
    }

    // checks if a directory has a .kit file, and subsequently a kit project.
    pub fn directory_has_project(directory: &std::path::Path) -> bool {
        return directory
            .join(std::path::Path::new(KIT_PROJECT_FILE))
            .exists();
    }
}
