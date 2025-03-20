mod clibrary;
mod cbuild;
mod test_build;
mod run_config;
mod settings;

use clibrary::CLibrary;
use crate::run_config::RunConfig;
use crate::test_build::TestBuild;
use crate::settings::Settings;


fn test() {
    let settings = Settings::from_file("settings/settings.toml");

    for test in &settings.tests {
        let run_config = RunConfig {
            test: test.clone(),
            working_dir: settings.working_dir.clone(),
            sources: settings.sources.clone(),
            includes: settings.includes.clone(),
        };

        let test_build = TestBuild::from_run_config(run_config);
        test_build.build();
        test_build.compile();

        let lib_path = test_build.get_lib_path();

        println!("Got: {:?}", lib_path);

        let lib = CLibrary::new(&lib_path);

        let functions = lib.get_functions();
        for function in functions {
            if function.starts_with("_ctest") {
                println!("Executing: {:?}", function);
                lib.test(&function);
            }
        }
    }
}

fn main() {
    test();
}
