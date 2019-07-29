use crate::parser::scope::Scope;
use crate::parser::var_type::*;
use crate::tree_nodes::tree_nodes::ImportExpr;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub struct Importer {
    path_cache: HashMap<String, Vec<Vec<String>>>,
    sheet_cache: HashMap<String, Vec<Vec<String>>>,
    pub current_directory: PathBuf,
}

impl Importer {
    pub fn new(current_directory: PathBuf) -> Importer {
        Importer {
            current_directory,
            path_cache: HashMap::new(),
            sheet_cache: HashMap::new(),
        }
    }

    pub fn update_scope(&mut self, scope: &mut Scope, imports: Vec<ImportExpr>) {
        for import in imports {
            if let Some(name) = import.value_of("name") {
                if let Some(filename) = import.value_of("path") {
                    scope.insert(name, VarType::Table(Var::new(self.get_from_file(filename))));
                } else {
                    if let Some(key) = import.value_of("key") {
                        scope.insert(name, VarType::Table(Var::new(self.get_google_sheet(key))));
                    } else {
                        println!("Unusable import, specify a file path \"path\" or a google sheets key \"key\" (The long gibberish string in the sheet's url)");
                    }
                }
            } else {
                println!("Skipping import because name is not specified");
            }
        }
    }

    pub fn update_with_file(&mut self,scope: &mut Scope, variable_name: String, filename: String) {
        scope.insert(variable_name, VarType::Table(Var::new(self.get_from_file(filename))));
    }

    pub fn update_with_sheet(&mut self,scope: &mut Scope, variable_name:String , sheet_id: String) {
        scope.insert(variable_name, VarType::Table(Var::new(self.get_google_sheet(sheet_id))));
    }

    fn get_from_file(&mut self, filename: String) -> Vec<Vec<String>> {
        let mut records = vec![];
        let path = Path::new(&filename);
        let mut joined;
        let current_path = if path.is_absolute() {
            path
        } else {
            joined = self.current_directory.join(path);
            joined.as_path()
        };
        let file_path = current_path.canonicalize();
        if let Err(_) = file_path {
            println!("Failed to read the file {}", filename);
            return records;
        };

        let file_path = file_path.unwrap();
        let full_file_name = &String::from(file_path.to_string_lossy());

        match self.path_cache.get(full_file_name) {
            Some(collection) => records = collection.clone(),
            None => {
                if let Ok(contents) = fs::read_to_string(full_file_name) {
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
                    self.path_cache
                        .insert(full_file_name.to_string(), records.clone());
                }
            }
        }

        records
    }

    fn get_google_sheet(&mut self, sheet_id: String) -> Vec<Vec<String>> {
        let mut records = vec![];
        let url = format!(
            "https://docs.google.com/spreadsheets/d/{}/export?format=csv",
            sheet_id
        );

        match self.sheet_cache.get(&sheet_id) {
            Some(collection) => records = collection.clone(),
            None => {
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
                    self.sheet_cache
                        .insert(String::from(sheet_id), records.clone());
                }
            }
        }

        records
    }
}
