use hamster_wheel::interpreter::importer::Importer;
use hamster_wheel::parser::scope::Scope;
use std::env;

extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("HamsterWheel")
        .version("0.2.0")
        .author("OsamaElHariri <yamsandbread@gmail.com>")
        .about("Looping code generator that allows for generating code over a collection")
        .subcommand(
            SubCommand::with_name("file")
            .help("Import a collection from a file")
                .about("Import a collection from a file")
                .arg(Arg::with_name("name").help("The name of the variable"))
                .arg(Arg::with_name("file_path").help("The file name")),
        )
        .subcommand(
            SubCommand::with_name("sheet")
            .help("Import a collection from Google Sheets")
                .about("Import a collection from Google Sheets")
                .arg(Arg::with_name("name").help("The name of the variable"))
                .arg(Arg::with_name("key").help("The sheet's ID. This is the long string in the URL. The sheet MUST have a shareable link for this to work")),
        )
        .get_matches();

    let mut base_scope = Scope::new();
    if let Some(matches) = matches.subcommand_matches("sheet") {
        let path = env::current_dir().expect("Could not read current directory");
        let name = matches
            .value_of("name")
            .expect("The sheet subcommand expects a name variable after it");
        let key = matches
            .value_of("key")
            .expect("The sheet subcommand expects a key variable after the name (this is the sheet ID in the url)");
        let mut importer = Importer::new(path);
        importer.update_with_sheet(&mut base_scope, String::from(name), String::from(key));
    }
    if let Some(matches) = matches.subcommand_matches("file") {
        let path = env::current_dir().expect("Could not read current directory");
        let name = matches
            .value_of("name")
            .expect("The file subcommand expects a name variable after it");
        let file_path = matches
            .value_of("file_path")
            .expect("The file subcommand expects a file path after the name");
        let mut importer = Importer::new(path);
        importer.update_with_file(&mut base_scope, String::from(name), String::from(file_path));
    }
    hamster_wheel::generate_with_scope(&mut base_scope);
    println!("Hamster Wheel Done!");
}
