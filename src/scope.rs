use std::collections::HashMap;
use crate::tree::SyntaxTree;

// Every symbol maps to an expression tree.
type Scope = HashMap<String, SyntaxTree>;
