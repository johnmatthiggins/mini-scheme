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
use crate::syntax::Expr;
use crate::syntax::ExprOps;
use crate::tree::Traversal;
use crate::tree::TraversalOps;
use crate::syntax;

fn eval_expr(ast: &Expr) -> Result<Expr, String> {
    // initialize traversal.
    let mut traverse = init_traversal(ast.to_owned()); 
    
    // Run traversal and get results.
    let result = traverse.run();
    
    return result;
}

fn exec_stackframe(line: Vec<Expr>) -> Result<Expr, String> {
    let operator = line.first();

    match operator {
        Some(exp) =>
            match exp {
                Expr::Atom(value) => 
                    match value {
                        Atom::Symbol(name) => {
                            // Grab all args after operator
                            let args = line[1..].to_vec();
                                                    
                            // execute function with newly created name.	
                            exec_fn(&name, &args);
                        },
                        _ => Err(String::from("First symbol must be operator!"))
                    },
                Expr::List(_) => Err("First symbol must be atomic!")
            },
        None => Err("Cannot execute blank expression!")
    }
}

fn next_sibling(root: &Expr, path: &Vec<u32>) -> Option<Expr> {
    // Find sibling based on path.
    let mut result = Option::None; 
    let mut sibling_path = path.clone();
    let mut current_exp = root.to_owned();
    
    let mut last = sibling_path.last_mut();
    // Add one to it to get next sibling.
    last += 1;

    // Loop until you find a row that has a larger node count
    // than the current path index.
    // Then return that expression and the new path.
}

// Maps function names to their real world operations.
fn exec_fn(name: &String, args: &Vec<Expr>) -> Result<Expr, String> {
    match name {
        syntax::EQ_OP => math_ops::eq_op(args),
        syntax::ADD_OP => math_ops::add_op(args),
        syntax::SUB_OP => math_ops::sub_op(args),
        syntax::MUL_OP => math_ops::mul_op(args),
        syntax::DIV_OP => math_ops::div_op(args),
        syntax::MOD_OP => math_ops::mod_op(args),
        
        // Just print error for stuff we don't support yet.
        _ => Err(String::from(
                format!("Operator '{}' not recognized!", name)))
    }
}

fn child_count(exp: &Expr) -> u32 {
    match exp {
        Expr::Atom(_) => 0,
        Expr::List(l) => (u32)l.len()
    }
}
