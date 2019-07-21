use crate::parser::var_type::VarType;
use std::collections::HashMap;

pub struct Scope<'a> {
    pub parent: Option<&'a Scope<'a>>,
    pub vars: HashMap<String, VarType>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Scope<'a> {
        Scope {
            parent: None,
            vars: HashMap::new(),
        }
    }

    pub fn with_parent(parent: &'a Scope) -> Scope<'a> {
        Scope {
            parent: Some(parent),
            vars: HashMap::new(),
        }
    }

    pub fn set_parent(&mut self, parent: &'a Scope) {
        self.parent = Some(parent);
    }

    pub fn insert(&mut self, key: String, var: VarType) -> Option<VarType> {
        self.vars.insert(key, var)
    }

    pub fn lookup(&self, key: &String) -> &VarType {
        let query = self.query(key);
        query.expect(&format!("Attempted to use undeclared variable {}", key))
    }

    pub fn query(&self, key: &String) -> Option<&VarType> {
        if let Some(var) = self.vars.get(key) {
            Some(var)
        } else {
            if let Some(parent) = &self.parent {
                parent.query(key)
            } else {
                None
            }
        }
    }
}
