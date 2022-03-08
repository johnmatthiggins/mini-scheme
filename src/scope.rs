use std::collections::HashMap;
use crate::tree::SyntaxTree;

// Every symbol maps to an expression tree.
type Scope = HashMap<String, SyntaxTree>;

pub trait Variables {
    fn define_var(&mut self, name: &String, value: &SyntaxTree);
    fn var_definition(&mut self, name: &String) -> Option<SyntaxTree>;
}

impl Variables for Scope {
    fn define_var(&mut self, name: &String, value: &SyntaxTree) {
        self.insert(name.to_owned(), value.to_owned());
    }

    fn var_definition(&mut self, name: &String) -> Option<SyntaxTree> {
        self.get(name);
    }
}
