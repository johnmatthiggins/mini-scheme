use crate::env::Env;
use crate::env::Eval;
use crate::syntax::Expr;
use crate::stack_encode::StackInstruction;
use std::collections::VecDeque;
use std::vec::Vec;

pub struct StackMachine {
    arg_stack: VecDeque<Vec<Expr>>,
    environment: Env,
    instructions: VecDeque<StackInstruction>,
}

pub impl StackMachine {
    // Output exit code if error occurs...
    fn next_instruction(&mut self) -> i32 {
        let next = instructions.pop_front();

        if let Some(line) = next {
            match line {
                StackInstruction::Exec(instr) => {
                    let mut args = arg_stack.pop_front();
                    
                    if let mut Some(arg_list) = args {
                        args_list.push_front(line);
                        let result = environment.eval_list(arg_list);

                        if let Ok(v) = result {
                            let next_args = self.arg_stack.pop_front();

                            if let Some(args) = next_args {
                                args.push(v);
                                self.arg_stack.push_front(args);
                                0
                            } else {
                                self.arg_stack.push_front(vec![v]);
                                0
                            }
                        } else {
                            // an error has occurred so return non zero.
                            1
                        }
                    } else {
                        let all_args = line;
                        let result = environment.eval_list(all_args);
                        
                        if let Ok(v) = result {
                            let next_args = self.arg_stack.pop_front();

                            if let Some(args) = next_args {
                                args.push(v);
                                self.arg_stack.push_front(args);
                                0
                            } else {
                                self.arg_stack.push_front(vec![v]);
                                0
                            }
                        } else {
                            // an error has occurred so return non zero.
                            1
                        }
                    }
                },
                StackInstruction::Wait => {
                    self.wait_instr();
                    0
                },
            }
        } else {
            // Return a non zero exit code if there aren't any more instructions.
            1
        }
    }

    fn wait_instr(&mut self) {
        let empty_vec = Vec::new();
        self.arg_stack.push_front(empty_vec);
    }

    fn push_instr(&mut self, expr: Expr) {
        let next_args = self.arg_stack.pop_front();

        if let Some(args) = next_args {
            args.push(v);
            self.arg_stack.push_front(args);
        } else {
            self.arg_stack.push_front(vec![v]);
        }
    }
}
