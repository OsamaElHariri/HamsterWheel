use crate::parser::var_type::VarType;
use std::collections::HashMap;

pub struct Scope {
    pub parent: Option<Box<Scope>>,
    pub vars: HashMap<String, VarType>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            parent: None,
            vars: HashMap::new(),
        }
    }

    pub fn with_parent(parent: Box<Scope>) -> Scope {
        Scope {
            parent: Some(parent),
            vars: HashMap::new(),
        }
    }

    pub fn set_parent(&mut self, parent: Box<Scope>) {
        self.parent = Some(parent);
    }

    pub fn insert(&mut self, key: String, var: VarType) -> Option<VarType> {
        self.vars.insert(key, var)
    }

    pub fn lookup(&self, key: &String) -> Option<&VarType> {
        if let Some(var) = self.vars.get(key) {
            Some(var)
        } else {
            if let Some(parent) = &self.parent {
                parent.lookup(key)
            } else {
                None
            }
        }
    }
}
