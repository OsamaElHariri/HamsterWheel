pub mod tokenizer;
pub mod tree_nodes;
pub mod parser;
pub mod interpreter;
pub mod file_walker;

use std::env;
use file_walker::file_walker::FileWalker;
use parser::scope::Scope;

pub fn generate() {
    let path = env::current_dir().expect("Could not read current directory");
    FileWalker::walk_directory(&path);
}

pub fn generate_with_scope(scope: &mut Scope) {
    let path = env::current_dir().expect("Could not read current directory");
    FileWalker::walk_directory_with_scope(&path, scope);
}