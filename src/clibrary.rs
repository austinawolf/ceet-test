use std::fs::File;
use libloading::{Library, Symbol};
use goblin::elf::Elf;
use std::io::Read;


pub struct CLibrary {
    filename: String,
    lib: Library,
    buffer: Vec<u8>,
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

        // Load the shared library before creating elf
        let lib = unsafe { Library::new(&filename).unwrap_or_else(|e| {
            panic!("Failed to load library {}: {}", &filename, e);
        }) };

        Self {
            filename: filename.to_string(),
            lib,
            buffer,
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

    pub fn test(&self, function_name: &str) {

        #[repr(C)]
        struct RainTestResults {
            magic: i32,
            assert: bool
        }

        type CtypeTestFunction = unsafe extern "C" fn();
        type RainTestRunFunction = unsafe extern "C" fn(CtypeTestFunction, *mut RainTestResults) -> i32;

        unsafe {
            // Load the test function symbol from the library
            let test_func: Symbol<CtypeTestFunction> =
                self.lib.get(&function_name.as_bytes()).expect("Failed to load test function");

            let runner_func: Symbol<RainTestRunFunction> =
                self.lib.get(b"rain_test_run").expect("Failed to load rain_test_run function");

            // Allocate the test results struct
            let mut results = RainTestResults {
                magic: 0xAA,
                assert: false
            };

            // Execute the function
            let return_code = runner_func(*test_func, &mut results);

            if (results.assert)
            {
                println!("test failed!");
            }
            else
            {
                println!("test success!");
            }

            println!("rain_test_run returned: {}", return_code);
        }
    }
}
