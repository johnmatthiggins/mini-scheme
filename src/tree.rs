// @file tree.rs
// @brief Bindings for creating an lisp abstract syntax tree.
//
// This file contains the AST data structure that is used
// to parse tokens into a tree. This tree must be mutable as
// it needs to have some symbols substituted for their definitions.
//
// @author johnmatthiggins@gmail.com
//

use std::vec::Vec;
use std::string::ToString;
use bigdecimal::BigDecimal;

// This will be the mutable substitute for "Expr" struct.
pub struct SyntaxTree {
    pub token: Atom,
    pub children: Vec<SyntaxTree>
}

// For printing out syntax tree.
impl ToString for SyntaxTree {
    fn to_string(&self) -> String {
        if self.children.is_empty() {
            token.to_string();
        }
        else {
            let mut output = String::from("(");

            for i in (0..self.children.len()) {
                if i != 0 {
                    output.push(' ');
                }

                output.push(self.children[i].to_string());
            }
    
            output.push(')');
            
            output;
        }
    }
}
