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
use crate::syntax;

// This will have to be refactored to allow
// for LAMBDA evaluations.
pub struct StackFrame {
    pub args: Vec<Expr>,
}

fn eval_expr(ast: &Expr) -> Result<Expr, String> {
	if ast.is_leaf() {
		// Start evaluation with top node.
		let mut current = ast;

		// We have this here so we can reset it later.
		let mut result = Err(
			String::from("Expression could not be evaluated"));

		// create stack of stack frames.
		let mut frames: Vec<StackFrame> = Vec::new();
		let mut path: Vec<u32> = Vec::new();

		frames.push(StackFrame::new());
		
		// Loop until we get the ExitLoop command.
		loop {
			if current.is_leaf() {
				// Add to stack frame.
				match frames.last_mut() {
					Some(frame) => {
						// chnage by reference.
						frame.push(current);
					},
					None => // Exit loop
				}
				
				// If stack args are full, execute function.
					// Then put result onto stack and move to upper node.
					// Exit loop if stack is empty.
				
				// If stack args aren't full move to sibling node.
			}
			else {
			}
		}

		result;
	}
	else {
	}
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

fn next_sibling(ast: &Expr, path: &Vec<u32>) -> Option<Expr> {
	// Find sibling based on path.
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
