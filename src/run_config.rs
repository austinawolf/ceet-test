use std::fs;
use toml_edit::DocumentMut;


pub struct RunConfig {
    pub test: String,
    pub working_dir: String,
    pub sources: Vec<String>,
    pub includes: Vec<String>,
}
