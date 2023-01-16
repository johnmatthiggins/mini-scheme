use crate::stack::StackInstruction;

fn encode_ast(ast: &Expr) -> Vec<StackInstruction> {
    match ast {
        List(expressions) => {
            let mut next_instructions = Vec::new();
            let mut list_count = 0;
            let cdr = expressions[1..];
            let car = expressions.first().unwrap();

            for next_expr in cdr {
                match next_expr {
                    List() => {
                        if list_count == 0 {
                            next_instructions.prepend(encode_ast(next_expr));
                        } else {
                            next_instructions.push_front(StackInstruction::Wait);
                            next_instructions.prepend(encode_ast(next_expr));
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
