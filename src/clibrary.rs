use std::fs::File;
use libloading::{Library, Symbol};
use goblin::elf::Elf;
use std::io::Read;

pub struct CLibrary {
    filename: String,
    buffer: Box<[u8]>,
    lib: Library,
}

impl CLibrary {
    pub fn new(filename: &str) -> Self {
        let mut file = File::open(filename).unwrap_or_else(|e| {
            panic!("Failed to open file {}: {}", &filename, e);
        });

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap_or_else(|e| {
            panic!("Failed to read file {}: {}", &filename, e);
        });

        let buffer = buffer.into_boxed_slice(); // Convert to heap storage first

        // Load the shared library before creating elf
        let lib = unsafe { Library::new(&filename).unwrap_or_else(|e| {
            panic!("Failed to load library {}: {}", &filename, e);
        }) };

        Self {
            filename: filename.to_string(),
            buffer,
            lib,
        }
    }

    pub fn get_functions(&self) -> Vec<String> {
        // Parse the ELF structure from the buffer
        let elf = Elf::parse(&self.buffer).unwrap_or_else(|e| {
            panic!("Failed to parse ELF for file {}: {:?}", &self.filename, e);
        });

        // Collect all function names into a vector
        let mut function_names = Vec::new();

        for sym in &elf.dynsyms {
            if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
                if sym.is_function() {
                    function_names.push(name.to_string());
                }
            }
        }

        function_names
    }

    pub fn execute_function(&self, function_name: &str) {
        unsafe {
            let func: Result<Symbol<unsafe extern "C" fn()>, _>
                = self.lib.get(function_name.as_bytes());
            match func {
                Ok(f) => f(),
                Err(e) => eprintln!("Failed to find function '{}': {}", function_name, e),
            }
        }
    }
}
