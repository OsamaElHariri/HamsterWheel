use hamster_wheel::interpreter::interpreter::Interpreter;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let output = Interpreter::new(
        "{{ output ok.txt }}
        {{import}}
        name: something;
        key: 1-hbZd6LH3153gVZi6CGq6n1HWv36Omt2k_OMc_6w3CE;
        {{end}}
        {{Loop(i) something as row}}
            Cell: ({{ row[0] }}, {{ row[1] }})
        {{END}}",
    )
    .interpret();
    println!("{}\n output to: {}", output.text, output.output_file);
    let path = Path::new(&output.output_file);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(output.text.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
