pub mod file_walker;
pub mod interpreter;
pub mod parser;
pub mod tokenizer;
pub mod tree_nodes;

use file_walker::file_walker::FileWalker;
use parser::scope::Scope;
use std::env;

/// Walk through the current directory and interpret all hamster_wheel files
pub fn generate() {
    let path = env::current_dir().expect("Could not read current directory");
    FileWalker::walk_directory(&path);
}

/// Walk through the current directory and interpret all hamster_wheel files
/// Uses the given `Scope` as the base and exposes it to all files that will be interpreted
pub fn generate_with_scope(scope: &mut Scope) {
    let path = env::current_dir().expect("Could not read current directory");
    FileWalker::walk_directory_with_scope(&path, scope);
}
