mod clibrary;
use clibrary::CLibrary;


fn main() {

    println!("Hello, world1!");

    // Hardcoded shared library path
    let filename = "./libtest.so";
    let lib = CLibrary::new(filename);
    let functions = lib.get_functions();
    println!("{:?}", functions);

    //
    // // Open the shared library file, handling errors manually
    // let mut file = match File::open(filename) {
    //     Ok(f) => f,
    //     Err(e) => {
    //         eprintln!("Error opening file {}: {}", filename, e);
    //         return;
    //     }
    // };
    //
    // let mut buffer = Vec::new();
    //
    // // Read the file into buffer, handling errors manually
    // if let Err(e) = file.read_to_end(&mut buffer) {
    //     eprintln!("Error reading file {}: {}", filename, e);
    //     return;
    // }
    //
    // // Parse the ELF file
    // match Elf::parse(&buffer) {
    //     Ok(elf) => {
    //         println!("Global functions in {}:", filename);
    //
    //         // Load the shared library once (optimization)
    //         let lib = unsafe { Library::new(filename) };
    //         if let Err(e) = lib {
    //             eprintln!("Failed to load library {}: {}", filename, e);
    //             return;
    //         }
    //         let lib = lib.unwrap(); // Safe because we checked above
    //
    //         for sym in &elf.syms {
    //             if !sym.is_function() {
    //                 continue;
    //             }
    //
    //             if let Some(name) = elf.strtab.get_at(sym.st_name) {
    //                 if !name.starts_with("_ctest_hook__") {
    //                     continue;
    //                 }
    //
    //                 println!("-> {}", name);
    //
    //                 // Load and execute the function dynamically
    //                 unsafe {
    //                     let func: Result<Symbol<unsafe extern "C" fn()>, _> = lib.get(name.as_bytes());
    //                     match func {
    //                         Ok(f) => f(),
    //                         Err(e) => eprintln!("Failed to find function '{}': {}", name, e),
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to parse ELF file: {}", e);
    //     }
    // }
}
