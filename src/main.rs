mod clibrary;
mod cbuild;
mod test_build;
use clibrary::CLibrary;
use crate::test_build::TestBuild;


// fn test() {
//     let build_dir = String::from("/home/awolf/dev/rain-test/example-project/.working/");
//
//     let tests = vec![
//         String::from("/home/awolf/dev/rain-test/example-project/tests/test_sensor.c"),
//         String::from("/home/awolf/dev/rain-test/example-project/tests/test_sensor2.c"),
//         String::from("/home/awolf/dev/rain-test/example-project/tests/test_sensor3.c"),
//     ];
//
//     for test in &tests {
//         let other_sources = vec![
//             String::from("/home/awolf/dev/rain-test/c/rain.c"),
//         ];
//         let other_includes = vec![
//             String::from("/home/awolf/dev/rain-test/c/")
//         ];
//
//         let test_build = TestBuild::new(test.clone(),
//                                         build_dir.clone(),
//                                         other_sources.clone(),
//                                         other_includes.clone());
//         test_build.build();
//         test_build.compile();
//
//         let lib_path = test_build.get_lib_path();
//
//         println!("Got: {:?}", lib_path);
//
//         let lib = CLibrary::new(&lib_path);
//
//
//         let functions = lib.get_functions();
//         for function in functions {
//             if function.starts_with("_ctest") {
//                 println!("Executing: {:?}", function);
//                 lib.execute_function(&function);
//             }
//         }
//     }
// }

fn test2() {
    let build_dir = String::from("/home/awolf/dev/rain-test/example-project/.working/");

    let test_file = String::from("/home/awolf/dev/rain-test/example-project/tests/test_sensor.c");

    let other_sources = vec![
        String::from("/home/awolf/dev/rain-test/c/rain.c"),
    ];
    let other_includes = vec![
        String::from("/home/awolf/dev/rain-test/c/")
    ];

    let test_build = TestBuild::new(test_file.clone(),
                                    build_dir.clone(),
                                    other_sources.clone(),
                                    other_includes.clone());
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

fn main() {
    test2();
}
