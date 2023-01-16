use crate::syntax::Expr;
use std::collections::VecDeque;

pub enum StackInstruction {
    Exec(Expr),

    // this stack machine instruction pushes an empty
    // arg list to the stack without popping the current args.
    Wait,

    Push(Expr),
}

fn encode_ast(ast: &Expr) -> VecDeque<StackInstruction> {
    match ast {
        Expr::List(expressions) => {
            let mut next_instructions = VecDeque::new();
            let mut list_count = 0;
            let cdr = expressions[1..].to_vec();
            let car = expressions.first().unwrap();

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
                            next_instructions.push_front(StackInstruction::Wait);
                        }
                        list_count += 1;
                    },
                    Expr::Atom(_) => {
                        next_instructions.push_front(StackInstruction::Push(ast.to_owned()));
                    },
                }
            }

            next_instructions.push_back(StackInstruction::Exec(car.to_owned()));

            next_instructions
        },
        Expr::Atom(_) => VecDeque::from([StackInstruction::Push(ast.to_owned())]),
    }
}
