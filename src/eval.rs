/**
 * @file eval.rs
 * @brief Primary evaluation algorithm used in interperter.
 *
 * Evaluates abstract syntax tree.
 *
 * @author John M. Higgins (johnmatthiggins@gmail.com)
 */

use std::collections::HashMap;
use std::vec::Vec;
use crate::scope::Scope;
use crate::syntax::Atom;

// This will have to be refactored to allow
// for LAMBDA evaluations.
pub struct StackFrame {
    pub name: String,
    pub args: Vec<Expr>,
}

// Tells loop what to do next.
pub enum TraversalCmd {
    Right,
    Down,
    Up
}

fn eval_expr(ast: &SyntaxTree, mut scope: &Scope)
    -> Result<SyntaxTree, String> {
    // Start evaluation with top node.
    let mut tree = ast.to_owned();
    let mut result = Err("Expression could not be evaluated".to_string());

    // create stack of stack frames.
    let mut stack_frame: Vec<StackFrame> = Vec::new();
    
    // Loop until we get the ExitLoop command.
    loop {
        if stack_frames.is_empty() {
            // We don't have anything else left to evaluate.
            if tree.children.is_empty() {
                // Set as result and end loop.
            }
            // We have more to evaluate so we have to step down.
            else {
                // Add Symbol to stackframe
                // add stackframe to stack.
                // Grab first child and set as next tree.
            }
        }
        else {
            // Give message to go right if this is not the last child,
            // or go up if it is the last child.
            if tree.children.is_empty() {
                // add token to stack frame above.
                // then move right to sibling node.
            }
            // Going downwards.
            else {
                // add token to new stack frame.
                // Then give the command to move up.
            }
        }
    }

    return result;
}
