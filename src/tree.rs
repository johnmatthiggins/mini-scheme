/**
 * @file tree.rs
 * @brief Bindings for creating a lisp abstract syntax tree.
 *
 * This file contains the AST data structure that is used
 * to parse tokens into a tree. This tree must be mutable as
 * it needs to have some symbols substituted for their definitions.
 *
 * @author John M. Higgins (johnmatthiggins@gmail.com)
 */

use std::vec::Vec;
use std::string::ToString;
use bigdecimal::BigDecimal;

// This will have to be refactored to allow
// for LAMBDA evaluations.
pub struct StackFrame {
    pub args: Vec<Expr>,
}

pub struct Traversal {
    pub call_stack: Vec<StackFrame>,
    pub path: Vec<u32>,
    pub root: Expr
    pub current: &Expr
}

pub fn init_traversal(root: Expr) -> Traversal {
    let traversal = Traversal {
        call_stack: Vec::new(),
        path: vec![0],
        root: root,
        current: &root
    };

    return traversal;
}

pub trait TraversalOps {
    pub fn run(&mut self) -> Result<Expr, String>;
}

impl Traversal for TraversalOps {
    fn run(&mut self) -> Result<Expr, String> { 
        // Loop until we get the ExitLoop command.
        loop {
            if current.is_leaf() {
                // Add to stack frame.
                match frames.last_mut() {
                    Some(frame) => {
                            // chnage by reference.
                            frame.args.push(current);
                    },
                    None => // Exit loop
                }
                    
                // If stack args are full, execute function.
                    // Then put result onto stack and move to upper node.
                    // Exit loop if stack is empty.
                let sibling_count = get_parent(current)
                        .and_then(|x| (u32)x.len())
                        .or(0);

                let curr_frame = frames.last().unwrap();
                let curr_arg_count = curr_frame.args.len();
            
                if sibling_count == curr_arg_count {
                    result = exec_stackframe(curr_frame);
                
                    // grab path excluding last turn.
                    let rest_of_path = path.split_last()
                        .and_then(|x| x.1)
                        .or(Vec::new());
                    
                    // If the list isn't long enough, just exit and return our result. 
                    match rest_of_path {
                        Some(v) => {
                            path = v.to_vec();
                        },
                        None => {
                            exit = true;
                        }
                    }
                }
                else {  
                }

                // If stack args aren't full move to sibling node.
            }
            else {
                
            }
            
            if exit {
                break;
            }
        }

        result;
    }
    else {
    }
}
