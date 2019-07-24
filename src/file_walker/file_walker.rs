pub struct FileWalker;
use crate::interpreter::interpreter::GeneralError;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::interpreter_result::InterpreterResult;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use walkdir::WalkDir;

impl FileWalker {
    pub fn walk_directory(_path: String) {
        for entry in WalkDir::new(".") {
            let file = entry.expect("Read path");
            let path = file.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension != "hamster_wheel" {
                        continue;
                    }
                    if let Err(e) = FileWalker::handle_file(path) {
                        eprintln!("{}", e);
                    }
                }
            }
        }
        println!("------------------------------------");
    }

    fn handle_file(path: &Path) -> Result<(), GeneralError> {
        let file_content = fs::read_to_string(path.canonicalize()?)?;

        let output = Interpreter::new(&file_content).interpret();
        println!("------------------------------------");
        match output {
            Ok(output) => {
                let output_file = FileWalker::write_to_file(&output, path.parent())?;
                println!("Successfully wrote {} to {}", path.display(), output_file,);
            }
            Err(e) => eprintln!("Failed to write to file {}\n{}", path.display(), e.msg),
        };
        Ok(())
    }

    fn write_to_file<'p>(
        interpreter_result: &'p InterpreterResult,
        hamster_wheel_file_path: Option<&'p Path>,
    ) -> Result<String, std::io::Error> {
        let path = Path::new(&interpreter_result.output_file);
        let path_buffer;
        let mut output_path = path;
        if let Some(parent) = hamster_wheel_file_path {
            if path.is_relative() {
                path_buffer = parent.join(path);
                output_path = path_buffer.as_path();
            }
        }

        let mut file = File::create(&output_path)?;
        file.write_all(interpreter_result.text.as_bytes())?;
        Ok(String::from(output_path.to_string_lossy()))
    }
}

impl From<std::io::Error> for GeneralError {
    fn from(error: std::io::Error) -> Self {
        GeneralError {
            msg: format!("{}", error),
        }
    }
}
