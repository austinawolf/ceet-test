use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct CBuild {
    name: String,
    build_dir: String,
    sources: Vec<String>,
    includes: Vec<String>,
}

impl CBuild {
    pub fn new(name: &str, build_dir: &str) -> Self {
        Self {
            name: name.to_string(),
            build_dir: build_dir.to_string(),
            sources: vec![],
            includes: vec![]
        }
    }

    pub fn add_sources(&mut self, sources: Vec<String>) {
        self.sources.extend(sources.into_iter().map(String::from));
    }

    pub fn add_includes(&mut self, includes: Vec<String>) {
        self.includes.extend(includes.into_iter().map(String::from));
    }

    pub fn clean_build_dir(&self) {
        let build_dir_path = PathBuf::from(&self.build_dir);
        if build_dir_path.exists() {
            fs::remove_dir_all(build_dir_path).unwrap();
        }
    }

    fn write_meson_build(&self) {
        let build_file_path = Path::new(self.build_dir.as_str()).join("meson.build");
        let build_file_contents = self.meson_build_string();
        let mut file = File::create(build_file_path).unwrap(); // Opens file in overwrite mode
        file.write_all(build_file_contents.as_bytes()).unwrap(); // Write bytes to file
    }

    pub fn generate(&self) {
        self.write_meson_build();
    }

    pub fn build(&self) {

        let _ = Command::new("meson") // Replace with "dir" on Windows
            .args(&["setup", "build"]) // Equivalent to `ls` on Windows
            .current_dir(&self.build_dir) // Set the working directory
            .output()
            .expect("Failed to execute command");

        // println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    pub fn compile(&self) {
        let _ = Command::new("meson")
            .args(&["compile", "-C", "build"])
            .current_dir(&self.build_dir)
            .output()
            .expect("Failed to execute command");
        // println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    fn meson_build_string(&self) -> String {
        const TEMPLATE_STRING: &str = include_str!("resources/template_meson.build");

        let includes_string = self.includes.iter()
            .map(|s| format!("'{}',", s))
            .collect::<Vec<String>>()
            .join("\n");

        let sources_string = self.sources.iter()
            .map(|s| format!("'{}',", s))
            .collect::<Vec<String>>()
            .join("\n");

        let output = TEMPLATE_STRING.to_string()
            .replace("REPLACE_PROJECT_NAME", &self.name)
            .replace("REPLACE_SOURCES", &sources_string)
            .replace("REPLACE_INCLUDES", &includes_string);

        output
    }

    pub fn get_lib_path(&self) -> String {
        let libname = format!("lib{}.so", self.name);
        let lib_path = Path::new(self.build_dir.as_str())
            .join("build")
            .join(libname);
        lib_path.to_str().unwrap().to_string()
    }


}
