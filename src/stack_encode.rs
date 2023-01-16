use crate::stack::StackInstruction;
use std::collections::VecDeque;

fn encode_ast(ast: &Expr) -> VecDeque<StackInstruction> {
    match ast {
        List(expressions) => {
            let mut next_instructions = VecDeque::new();
            let mut list_count = 0;
            let cdr = expressions[1..];
            let car = expressions.first().unwrap();

            for next_expr in cdr {
                match next_expr {
                    List(expressions) => {
                        if list_count == 0 {
                            let mut prepended = encode_ast(next_expr);
                            prepended.append(next_instructions);

                            next_instructions = prepended;
                        } else {
                            let mut prepended = encode_ast(next_expr);
                            prepended.append(next_instructions);

                            next_instructions = prepended;
                            next_instructions.push_front(StackInstruction::Wait);
                        }
                        list_count += 1;
                    },
                    Atom(_) => {
                        next_instructions.append(StackInstruction::Push(ast));
                    },
                }
            }

            next_instructions.push(StackInstruction::Exec(car));
        },
        Atom(_) => vec![StackInstruction::Push(ast)],
    }
}
