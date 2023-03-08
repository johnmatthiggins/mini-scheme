use crate::syntax::{Expr, Atom};
use std::collections::VecDeque;

#[derive(Debug)]
pub enum StackInstruction {
    Exec(Expr),
    RawExec(Vec<Expr>),

    // this stack machine instruction pushes an empty
    // arg list to the stack without popping the current args.
    Wait,

    Push(Expr),
}

pub fn is_define_keyword(expr: &Expr) -> bool {
    if let Expr::Atom(atom) = expr {
        if let Atom::Symbol(name) = &**atom {
            name.as_str() == "define"
        } else {
            false
        }
    } else {
        false
    }
}

pub fn is_lambda_keyword(expr: &Expr) -> bool {
    if let Expr::Atom(atom) = expr {
        if let Atom::Symbol(name) = &**atom {
            name.as_str() == "lambda"
        } else {
            false
        }
    } else {
        false
    }
}

pub fn encode_ast(ast: &Expr) -> VecDeque<StackInstruction> {
    match ast {
        Expr::List(expressions) => {
            let mut next_instructions = VecDeque::new();
            let mut list_count = 0;
            let cdr = expressions[1..].to_vec();
            let car = expressions.first().unwrap();

            if is_define_keyword(car) || is_lambda_keyword(car) {
                next_instructions.push_front(
                    StackInstruction::RawExec(expressions.to_owned()));
            }
            else {
                for next_expr in cdr {
                    match next_expr {
                        Expr::List(_) => {
                            if list_count == 0 {
                                let mut prepended = encode_ast(&next_expr);
                                prepended.append(&mut next_instructions);

                                next_instructions = prepended;
                            } else {
                                let mut prepended = encode_ast(&next_expr);
                                prepended.append(&mut next_instructions);

                                next_instructions = prepended;
                                next_instructions.push_back(StackInstruction::Wait);
                            }
                            list_count += 1;
                        },
                        Expr::Atom(_) => {
                            next_instructions.push_back(
                                StackInstruction::Push(next_expr.to_owned()));
                        },
                    }

                }

                next_instructions.push_back(StackInstruction::Exec(car.to_owned()));

            }

            next_instructions
        },
        Expr::Atom(_) => VecDeque::from([StackInstruction::Push(ast.to_owned())]),
    }
}
