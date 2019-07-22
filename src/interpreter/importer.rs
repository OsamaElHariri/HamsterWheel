use crate::parser::scope::Scope;
use crate::parser::var_type::*;
use crate::tree_nodes::tree_nodes::ImportExpr;
use std::fs;

pub struct Importer {}

impl Importer {
    pub fn update_scope(scope: &mut Scope, imports: Vec<ImportExpr>) {
        for import in imports {
            if let Some(name) = import.value_of("name") {
                if let Some(filename) = import.value_of("path") {
                    scope.insert(
                        name,
                        VarType::Table(Var::new(Importer::get_from_file(filename))),
                    );
                } else {
                    if let Some(key) = import.value_of("key") {
                        scope.insert(
                            name,
                            VarType::Table(Var::new(Importer::get_google_sheet(key))),
                        );
                    } else {
                        panic!("Unusable config, specify a file path \"path\" or a google sheets key \"key\" (The long gibberish string in the url)");
                    }
                }
            } else {
                panic!("Name must be specified in the configs");
            }
        }
    }

    fn get_from_file(filename: String) -> Vec<Vec<String>> {
        let error_msg = format!("Could not read the file {}", &filename);
        let contents = fs::read_to_string(filename).expect(&error_msg);

        let mut records = vec![];
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(contents.as_bytes());
        for result in rdr.records() {
            match result {
                Ok(record) => {
                    let mut values = vec![];
                    for val in record.iter() {
                        values.push(val.to_string());
                    }
                    records.push(values);
                }
                Err(err) => {
                    println!("Failed to read csv from file: {}", err);
                }
            }
        }
        records
    }

    fn get_google_sheet(sheet_id: String) -> Vec<Vec<String>> {
        let mut records = vec![];
        let url = format!(
            "https://docs.google.com/spreadsheets/d/{}/export?format=csv",
            sheet_id
        );
        if let Ok(val) = reqwest::get(&url) {
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(val);
            for result in rdr.records() {
                match result {
                    Ok(record) => {
                        let mut values = vec![];
                        for val in record.iter() {
                            values.push(val.to_string());
                        }
                        records.push(values);
                    }
                    Err(err) => {
                        println!("Failed to read csv from Google Sheets: {}", err);
                    }
                }
            }
        }
        records
    }
}
