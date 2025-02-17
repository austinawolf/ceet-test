use goblin::elf::Elf;
use std::fs::File;
use std::io::{self, Read};


// extern "C" {
//     fn sensor_init();
//     fn sensor_get_value() -> i32;
// }

fn main() {

    println!("Hello, world1!");

    // Hardcoded shared library path
    let filename = "./libtest.so";

    // Open the shared library file, handling errors manually
    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file {}: {}", filename, e);
            return;
        }
    };

    let mut buffer = Vec::new();

    // Read the file into buffer, handling errors manually
    if let Err(e) = file.read_to_end(&mut buffer) {
        eprintln!("Error reading file {}: {}", filename, e);
        return;
    }

    // Parse the ELF file
    match Elf::parse(&buffer) {
        Ok(elf) => {
            println!("Global functions in {}:", filename);
            for sym in &elf.syms {
                if sym.is_function() {
                    if let Some(Ok(name)) = elf.strtab.get(sym.st_name) {
                        if !name.is_empty() {
                            println!("\t{}", name);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to parse ELF file: {}", e);
        }
    }

    // unsafe {
    //     let lib = Library::new("./libtest.so").expect("Failed to load library");
    //     let f: Symbol<unsafe extern "C" fn()> = lib.get(b"test_asdf").expect("Failed to find function");
    //     f()
    // }

}
