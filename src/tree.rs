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
    fn find_next(&mut self);
    pub fn run(&mut self) -> Result<Expr, String>;
}

// Primary evaluation algorithm for interpreter.
impl Traversal for TraversalOps {
    fn find_next(&mut self) {
        loop {
            // Stackframe should have new Expr added to it from current iteration.
            // Path should be one less element because we are going to search for siblings.
            // 
        }
    }

    fn run(&mut self) -> Result<Expr, String> { 
        // Loop until we get the ExitLoop command.
        loop {
            if self.current.is_leaf() {
                // Add to stack frame.
                match self.call_stack.last_mut() {
                    Some(stack_frame) => {
                            // change by reference.
                            stack_frame.args.push(current);
                    },
                    None => {
                        // If arg stack is empty and first node is leaf,
                        // just return it.
                        result = Ok(self.current.to_owned());
                        break;
                    }
                }
                    
                // If stack args are full, execute function.
                    // Then put result onto stack and move to upper node.
                    // Exit loop if stack is empty.
                let sibling_count = get_parent(current)
                        .and_then(|x| match x {
                            Expr::List(l) => (u32)l.len(),
                            _ => None
                        })
                        .or(0);

                let curr_frame = self.call_stack.last_mut().unwrap();
                let curr_arg_count = curr_frame.args.len();
            
                if sibling_count == curr_arg_count {
                    // Execute stack frame once we have enough arguments in it.
                    let exec_frame = self.call_stack.args.pop();
                    result = exec_stackframe(exec_frame).unwrap();

                    match result {
                        Ok(v) => {
                            self.call_stack.args.push(v);
                    
                            // Seek next node in tree.
                            self.find_next();
                        },
                        Err(err) => {
                            // Show error and exit loop.
                            result = Result::Err(err.to_owned());
                            break;
                        }
                    }
                }
                // If stack args aren't full, move directly to sibling node.
                else {
                    // Add one to it to get next sibling.
                    let mut last = sibling_path.last_mut();
                    last += 1;
                    
                    // then we assign current as next child in line.
                    self.current = get_child(self.root, self.path);
                }
            }
            else {
                // Move down.
                self.path.push(0);
                self.current = get_child(&self.root, &self.path).unwrap();
                
                // Add new stack frames as we move down. 
                let next_frame = StackFrame {
                    args = Vec::new()
                };

                self.call_stack.push();
            }
            
            if exit {
                break;
            }
        }

        return result;
    }
}
