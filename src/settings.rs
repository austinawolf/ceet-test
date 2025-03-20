use std::fs;
use toml_edit::{Array, DocumentMut, Item, Value};


pub struct TestSettings {
    pub name: String,
    pub test_path: String,
}

pub struct Settings {
    pub name: String,
    pub working_dir: String,
    pub sources: Vec<String>,
    pub includes: Vec<String>,
    pub tests: Vec<TestSettings>,
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
        Self::from_string(&config_str)
    }

    pub fn from_string(string: &str) -> Self {
        let doc = string.parse::<DocumentMut>().expect("Invalid config file");

        let title = doc["title"].as_str().unwrap();
        let working_dir = doc["working_dir"].as_str().unwrap();
        let sources = doc["sources"].as_array().unwrap();
        let includes = doc["includes"].as_array().unwrap();

        let test_names: Vec<String>  = doc.iter()
            .filter_map(|(key, _value)| {
                if key.starts_with("test_") {
                    Some(key.to_string())
                } else {
                    None
                }
            }).collect();

        let mut test_modules: Vec<TestSettings> = Vec::new();
        for name in test_names {
            let module = &doc[&name];

            if let Item::Table(table) = module {

                // Find "test" filename
                let item = table.get("test").unwrap();

                if let Item::Value(value) = item {
                    if let Value::String(message) = value {
                        let test_path = message.value();
                        println!("{}", string);
                        let module_settings = TestSettings {
                            name,
                            test_path: test_path.to_string(),
                        };
                        test_modules.push(module_settings);

                    }
                }
            }
        }

        Self {
            name: String::from(title),
            working_dir: String::from(working_dir),
            tests: test_modules,
            sources: array_to_vec(sources),
            includes: array_to_vec(includes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let toml_data = r#"
            title = "Test Project"
            working_dir = "/home/user/project"
            sources = ["src/main.rs", "src/lib.rs"]
            includes = ["include/", "deps/"]

            [test_sensor]
            test = "test_sensor.c"
        "#;

        let settings = Settings::from_string(toml_data);

        assert_eq!(settings.name, "Test Project");
        assert_eq!(settings.working_dir, "/home/user/project");
        assert_eq!(settings.sources, vec!["src/main.rs", "src/lib.rs"]);
        assert_eq!(settings.includes, vec!["include/", "deps/"]);
        assert_eq!(settings.tests.len(), 1);
        assert_eq!(settings.tests[0].name, "test_sensor");
        assert_eq!(settings.tests[0].test_path, "test_sensor.c");
    }
}