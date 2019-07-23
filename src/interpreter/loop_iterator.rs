use crate::parser::scope::Scope;
use crate::parser::var_type::Var;
use crate::parser::var_type::VarType;

pub struct LoopIterator<'a> {
    _start: usize,
    end: usize,
    scope: &'a Scope<'a>,
    loop_index: usize,
    collection_index: usize,
    collection: VarType,
    loop_variable_name: Option<String>,
    collection_variable_name: Option<String>,
    as_variable_name: Option<String>,
}

impl<'a> LoopIterator<'a> {
    pub fn new(
        scope: &'a Scope,
        collection: VarType,
        min: usize,
        max: usize,
        loop_variable_name: Option<String>,
        collection_variable_name: Option<String>,
        as_variable_name: Option<String>,
    ) -> LoopIterator<'a> {
        LoopIterator {
            scope,
            collection,
            _start: min,
            end: max,
            loop_variable_name,
            collection_variable_name,
            as_variable_name,
            collection_index: min,
            loop_index: 0,
        }
    }
}

impl<'a> Iterator for LoopIterator<'a> {
    type Item = Scope<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.collection_index < self.end {
            let mut scope = Scope::with_parent(self.scope);

            scope.insert(
                String::from("loop_index"),
                VarType::Number(Var::new(self.loop_index)),
            );
            scope.insert(
                String::from("collection_index"),
                VarType::Number(Var::new(self.collection_index)),
            );

            if let Some(variable) = &self.loop_variable_name {
                scope.insert(variable.clone(), VarType::Number(Var::new(self.loop_index)));
            }

            if let Some(variable) = &self.collection_variable_name {
                scope.insert(
                    variable.clone(),
                    VarType::Number(Var::new(self.collection_index)),
                );
            }

            match &self.collection {
                VarType::Table(var) => {
                    if let Some(variable) = &self.as_variable_name {
                        scope.insert(
                            variable.clone(),
                            VarType::Row(Var::new(var.data[self.collection_index].clone())),
                        );
                    }
                }
                VarType::Row(var) => {
                    if let Some(variable) = &self.as_variable_name {
                        scope.insert(
                            variable.clone(),
                            VarType::Value(Var::new(var.data[self.collection_index].clone())),
                        );
                    }
                }
                _ => panic!("Attempt to loop on a non-iterable"),
            };

            self.loop_index += 1;
            self.collection_index += 1;

            Some(scope)
        } else {
            None
        }
    }
}
