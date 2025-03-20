use std::fs;
use toml_edit::{Array, DocumentMut, Value};



pub struct Settings {
    pub name: String,
    pub working_dir: String,
    pub tests: Vec<String>,
    pub sources: Vec<String>,
    pub includes: Vec<String>,
}


fn array_to_vec(arr: &Array) -> Vec<String> {
    arr.iter()
        .filter_map(|value| match value {
            Value::String(s) => Some(s.value().to_string()),
            _ => None, // Ignore non-string values
        })
        .collect()
}

impl Settings {
    pub fn from_file(filename: &str) -> Self {
        let config_str = fs::read_to_string(filename).expect("Failed to read config file");
        let doc = config_str.parse::<DocumentMut>().expect("Invalid config file");

        let title = doc["title"].as_str().unwrap();
        let working_dir = doc["working_dir"].as_str().unwrap();
        let tests = doc["tests"].as_array().unwrap();
        let sources = doc["sources"].as_array().unwrap();
        let includes = doc["includes"].as_array().unwrap();

        Self {
            name: String::from(title),
            working_dir: String::from(working_dir),
            tests: array_to_vec(tests),
            sources: array_to_vec(sources),
            includes: array_to_vec(includes),
        }
    }
}