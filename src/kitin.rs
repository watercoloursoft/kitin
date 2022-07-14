use serde::{Deserialize, Serialize};
use std::{
    fmt,
    io::{Read, Write},
    str::FromStr,
};

const KITIN_PROJECT_FILE: &str = "kitin.yaml";

#[derive(Deserialize, Debug)]
pub enum KitinModuleSourceType {
    Git,
    Local,
}

impl FromStr for KitinModuleSourceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Git" => Ok(KitinModuleSourceType::Git),
            "Local" => Ok(KitinModuleSourceType::Local),
            _ => Err(format!("Unknown source type: {}", s)),
        }
    }
}

impl fmt::Display for KitinModuleSourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KitinModuleSourceType::Git => write!(f, "Git"),
            KitinModuleSourceType::Local => write!(f, "Local"),
        }
    }
}

impl Serialize for KitinModuleSourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Clone for KitinModuleSourceType {
    fn clone(&self) -> Self {
        match self {
            KitinModuleSourceType::Git => KitinModuleSourceType::Git,
            KitinModuleSourceType::Local => KitinModuleSourceType::Local,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KitinModule {
    source: Option<String>,
    source_type: Option<KitinModuleSourceType>,
}

impl KitinModule {
    pub fn new(source: String, source_type: KitinModuleSourceType) -> Self {
        KitinModule {
            source: Some(source),
            source_type: Some(source_type),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KitinProject {
    name: Option<String>,
    modules: Option<std::collections::HashMap<String, KitinModule>>,
}

impl KitinProject {
    pub fn new() -> Self {
        KitinProject {
            name: None,
            modules: Some(std::collections::HashMap::new()),
        }
    }

    // loads a kitin.yaml file from the current working directory
    pub fn load_from_file(&mut self) {
        let path = std::path::Path::new(KITIN_PROJECT_FILE);
        if !path.exists() {
            return;
        }
        let mut file = std::fs::File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let data: KitinProject = serde_yaml::from_str(&contents).unwrap();
        self.name = data.name;
        self.modules = data
            .modules
            .or_else(|| Some(std::collections::HashMap::new()));
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

    pub fn add_module(&mut self, path: String, module: KitinModule) {
        self.modules.as_mut().unwrap().insert(path, module);
    }
}
