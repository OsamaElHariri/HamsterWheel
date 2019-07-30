use crate::parser::var_type::VarType;
use std::collections::HashMap;
use std::fmt;

pub struct Scope<'a> {
    pub parent: Option<&'a Scope<'a>>,
    pub vars: HashMap<String, VarType>,
}

impl<'a> Scope<'a> {
    /// Construct a new `Scope`
    pub fn new() -> Scope<'a> {
        Scope {
            parent: None,
            vars: HashMap::new(),
        }
    }

    /// Construct a new `Scope` that has a parent
    pub fn with_parent(parent: &'a Scope) -> Scope<'a> {
        Scope {
            parent: Some(parent),
            vars: HashMap::new(),
        }
    }

    /// Set this `Scope`'s parent
    pub fn set_parent(&mut self, parent: &'a Scope) {
        self.parent = Some(parent);
    }

    /// Insert a new value into this scope
    pub fn insert(&mut self, key: String, var: VarType) -> Option<VarType> {
        self.vars.insert(key, var)
    }

    /// Lookup a value in this `Scope`, if not found keep looking in this
    /// `Scope`'s ancestors for the value.
    /// Return Err if the value is not found
    pub fn lookup(&self, key: &String) -> Result<&VarType, ScopeError> {
        let query = self.query(key);
        match query {
            Some(val) => Ok(val),
            None => Err(ScopeError {
                msg: format!("Attempted to use undeclared variable {}", key),
            }),
        }
    }

    fn query(&self, key: &String) -> Option<&VarType> {
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

pub struct ScopeError {
    msg: String,
}

impl fmt::Display for ScopeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl fmt::Debug for ScopeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}
