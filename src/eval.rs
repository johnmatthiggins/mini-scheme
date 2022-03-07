use std::collections::HashMap;
use std::vec::Vec;
use crate::scope::Scope;
use crate::syntax::Atom;

// Split up into phases:
// Phase 1: 
// Phase 2: Substitute symbols for definitions.
// Phase 3: Iterate through constructed tree.

// This will have to be refactored to allow
// for LAMBDA evaluations.
pub struct StackFrame {
    pub name: String,
    pub args: Vec<Expr>,
}

// Tells loop what to do next.
pub enum TraversalCmd {
    Identify,
    ExpandSymbol,
    DefineSymbol,
    ReturnValue,
    ExitLoop
}

fn eval_expr(ast: &SyntaxTree, mut scope: &Scope)
    -> Result<SyntaxTree, String> {
    // Start evaluation with top node.
    let mut tree = Ok(ast.to_owned());
    let mut next_cmd = Identify;
    
    // Create new stackframes and add scopes to them.
    let mut scope_stack = Vec::new();
    scope_stack.push(scope);
    
    // Loop until we get the ExitLoop command.
    loop {
        let current_scope = scope_stack.last();

        match next_cmd {
            TraversalCmd::ExitLoop => {
                break;
            },
            TraversalCmd::Identify => {
            },
            TraversalCmd::ExpandSymbol => {
            },
            TraversalCmd::DefineSymbol => {
            },
            TraversalCmd::ReturnValue => {
            },
        }
    }

    return next_tree;
}
