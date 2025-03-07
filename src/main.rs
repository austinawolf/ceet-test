mod clibrary;
mod cbuild;

use clibrary::CLibrary;
use crate::cbuild::CBuild;

fn main() {
    // Hardcoded shared library path
    let name = "test_sensor";
    let build_dir = "/home/awolf/dev/rain-test/example-project/.working/test_sensor";
    let sources = vec![
        "/home/awolf/dev/rain-test/example-project/tests/test_sensor.c",
        "/home/awolf/dev/rain-test/c/rain.c"
    ];
    let includes = vec![
        "/home/awolf/dev/rain-test/c/"
    ];

    let mut cbuild = CBuild::new(name, build_dir);
    cbuild.add_sources(sources);
    cbuild.add_includes(includes);

    cbuild.write_meson_build();
    cbuild.build();
    cbuild.compile();

    let filename = cbuild.get_lib_path();
    let lib = CLibrary::new(&filename);

    let functions = lib.get_functions();
    for function in functions {
        if function.starts_with("_ctest") {
            println!("Executing: {:?}", function);
            lib.execute_function(&function);
        }
    }
}
