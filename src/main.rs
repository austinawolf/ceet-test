mod clibrary;
use clibrary::CLibrary;


fn main() {
    // Hardcoded shared library path
    let filename = "./libtest.so";
    let lib = CLibrary::new(filename);
    let functions = lib.get_functions();
    println!("{:?}", functions);

    for function in functions {
        if function.starts_with("_ctest") {
            println!("Executing: {:?}", function);
            lib.execute_function(&function);
        }
    }
}
