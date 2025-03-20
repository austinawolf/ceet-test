use std::fs;
use std::path::{Path};
use crate::cbuild::CBuild;
use crate::run_config::RunConfig;

pub struct TestBuild {
    test_file: String,
    working_dir: String,
    cbuild: CBuild,
}

impl TestBuild {
    pub fn from_run_config(run_config: RunConfig) -> Self {
        Self::new(
            run_config.test,
            run_config.working_dir,
            run_config.sources,
            run_config.includes,
        )
    }

    pub fn new(test_file: String, working_dir: String, sources: Vec<String>, includes: Vec<String>) -> Self {
        let name = Path::new(&test_file).file_stem().unwrap().to_str().unwrap();
        let build_dir = Path::new(&working_dir).join(name).to_str().unwrap().to_string();

        if !Path::new(&build_dir.clone()).exists() {
            fs::create_dir_all(&build_dir).expect("Failed to create directory");
        }

        let mut cbuild = CBuild::new(&name, &build_dir);
        cbuild.add_sources(sources.clone());
        cbuild.add_sources(vec![test_file.clone()]);
        cbuild.add_includes(includes.clone());

        Self {
            test_file: test_file.to_string(),
            working_dir: working_dir.to_string(),
            cbuild,
        }
    }

    pub fn build(&self) {
        self.cbuild.generate();
        self.cbuild.build();
    }

    pub fn compile(&self) {
        self.cbuild.compile();
    }

    pub fn get_lib_path(&self) -> String{
        self.cbuild.get_lib_path()
    }
}
