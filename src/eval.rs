use std::collections::HashMap;
use std::vec::Vec;

// This will have to be refactored to allow
// for LAMBDA evaluations.
pub struct StackFrame {
    pub name: String,
    pub args: Vec<Expr>,
}

fn eval_expr(expr: &Expr, mut env: &HashMap<String, Expr>)
    -> Result<Expr, String> {
    // Create stack to store the AST path.
    let mut path: Vec<u32> = vec::Vec();
    let mut frames: Vec<StackFrame> = vec::Vec();
    let mut current = Ok(expr);
    let mut level: u32 = 0;
    
    // loop until we are done walking
    // through every part of the tree.
    loop {
        match expr {
            Expr::List(v) =>
                v.first().and_then(|x| x match {
                    Expr::Atom(a1) => {
                        // Create new stack frame with name of function.
                        let mut frame = StackFrame {
                            name: a1,
                            args: vec::Vec()
                        };
                        frames.push(frame);

                        // Select next current expression.
                    },
                    Expr::List(v1) => {
                    }
                }),
            Expr::Atom(a) => {
                if frames.is_empty() {
                    // End loop if you are done evaluating.
                    current = Ok(Expr::Atom(a));
                    break;
                }
                else {
                    // evaluate 
                }
            }
        }
    }

    return ;
}

fn walk_tree(expr: &Expr, path: &Vec<u32>) -> Option<Expr> {
    let mut end_node = Some(expr.to_owned());

    for i in path {
        end_node = grab_node(&end_node, i);
    }

    return end_node;
}

fn grab_node(expr: &Expr, node_num: u32) -> Option<Expr> {
    match expr {
        Expr::Atom(a) => {
            if node_num == 0 {
                Some(Expr::Atom(a));
            }
            else {
                None;
            }
        },
        Expr::List(v) => v.get(node_num);
    }
}
