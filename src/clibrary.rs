use std::fs::File;
use libloading::Library;
use goblin::elf::Elf;
use std::io::Read;


pub struct CLibrary {
    filename: String,
}

impl CLibrary {

    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
        }
    }

    pub fn get_functions(&self) -> Vec<String> {
        // Open the shared library file
        let mut file = File::open(&self.filename).unwrap_or_else(|e| {
            panic!("Failed to open file {}: {}", &self.filename, e);
        });

        // Load the shared library
        let _lib = unsafe { Library::new(&self.filename).unwrap_or_else(|e| {
            panic!("Failed to load library {}: {}", &self.filename, e);
        }) };

        // Read the file contents into a buffer
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap_or_else(|e| {
            panic!("Failed to read file {}: {}", &self.filename, e);
        });

        // Parse the ELF structure from the buffer
        let elf = Elf::parse(&buffer).unwrap_or_else(|e| {
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
}
