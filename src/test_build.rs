use std::fs;
use std::path::{Path};
use crate::cbuild::CBuild;

pub struct TestBuild {
    test_file: String,
    working_dir: String,
    sources: Vec<String>,
    includes: Vec<String>,
}

impl TestBuild {
    pub fn new(test_file: String, working_dir: String, sources: Vec<String>, includes: Vec<String>) -> Self {
        Self {
            test_file: test_file.to_string(),
            working_dir: working_dir.to_string(),
            sources,
            includes,
        }
    }

    pub fn build(&self) -> String{
        let name = Path::new(&self.test_file).file_stem().unwrap().to_str().unwrap();
        let build_dir = Path::new(&self.working_dir).join(name).to_str().unwrap().to_string();
        let build_dir2 = build_dir.clone();

        if !Path::new(&build_dir.clone()).exists() {
            fs::create_dir_all(build_dir).expect("Failed to create directory");
        }

        let mut cbuild = CBuild::new(&name, &build_dir2);
        cbuild.add_sources(self.sources.clone());
        cbuild.add_sources(vec![self.test_file.clone()]);
        cbuild.add_includes(self.includes.clone());

        cbuild.build();
        cbuild.compile();

        cbuild.get_lib_path()
    }
}
