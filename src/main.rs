use hamster_wheel::interpreter::interpreter::Interpreter;
use hamster_wheel::interpreter::interpreter_result::InterpreterResult;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    //     let output = Interpreter::new(
    //         "{{ output localization.dart }}
    // {{import}}
    // name: localizations;
    // key: 1JqduyjCwmjKmmyBT0eM7YuD_El2Z_5grafi2hLV8OX4;
    // {{end}}

    // class Localized {
    //     static String languageCode = \"en\";
    // {{Loop localizations[1, ..] as localizationRow}}
    // static String get {{localizationRow[0]}} {
    //   {{Loop(i, collectionIndex)  localizations[0][1, ..] as languageCode }}
    //     if (languageCode == \"{{languageCode}}\") return \"{{localizationRow[collectionIndex]}}\";
    //   {{End}}
    //     return \"{{localizationRow[3]}}\";
    //   }
    // {{END}}
    // }
    // ",
    //     )
    //     .interpret();
        // println!("{}\n output to: {}", output.text, output.output_file);

    for entry in WalkDir::new(".") {
        let file = entry.expect("Read path");
        let path = file.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension != "hamster_wheel" {
                    continue;
                }
                println!("start read {}", path.display());
                let file_content = fs::read_to_string(path.canonicalize().unwrap()).expect(
                    &format!("Something went wrong reading the file {}", path.display()),
                );
                println!("found content: {}", file_content);
                let output = Interpreter::new(&file_content).interpret();
                // println!("{:?}", output);
                write_to_file(output, path.parent());
            }
        }
    }
    println!("Hamster Wheel Done!");
}

fn write_to_file(interpreter_result: InterpreterResult, hamster_wheel_file_path: Option<&Path>) {
    let path = Path::new(&interpreter_result.output_file);
    let path_buffer;
    let mut output_path = path;
    if let Some(parent) = hamster_wheel_file_path {
        if path.is_relative() {
            path_buffer = parent.join(path);
            output_path = path_buffer.as_path();
        }
    }

    let display = path.display();
    let mut file = match File::create(&output_path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };
    match file.write_all(interpreter_result.text.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    };
}
