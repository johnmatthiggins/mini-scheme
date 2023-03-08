use crate::env::{Env, Eval};
use crate::syntax::Expr;
use crate::stack_encode::StackInstruction;
use std::collections::VecDeque;
use std::vec::Vec;

pub struct StackMachine {
    pub arg_stack: VecDeque<VecDeque<Expr>>,
    pub environment: Env,
    pub instructions: VecDeque<StackInstruction>,
}

impl StackMachine {
    pub fn create(env: Env) -> StackMachine {
        StackMachine {
            arg_stack: VecDeque::new(),
            environment: env,
            instructions: VecDeque::new(),
        }
    }

    pub fn run_instructions(&mut self,
                            stream: VecDeque<StackInstruction>) -> Expr {
        self.instructions = stream;

        while self.next_instruction() == 0 { }

        self.get_result()
    }

    // Output exit code if error occurs...
    pub fn next_instruction(&mut self) -> i32 {
        let next = self.instructions.pop_front();

        if let Some(line) = next {
            match line {
                StackInstruction::Exec(cmd) => {
                    self.exec_instr(cmd);
                    0
                },
                StackInstruction::RawExec(cmds) => {
                    self.raw_exec_instr(cmds);
                    0
                },
                StackInstruction::Push(expr) => {
                    self.push_instr(expr);
                    0
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
        let empty_vec = VecDeque::new();
        self.arg_stack.push_front(empty_vec);
    }

    fn push_instr(&mut self, expr: Expr) {
        let next_args = self.arg_stack.pop_front();

        if let Some(mut args) = next_args {
            args.push_back(expr);
            self.arg_stack.push_front(args);
        } else {
            self.arg_stack.push_front(VecDeque::from([expr]));
        }
    }

    fn exec_instr(&mut self, expr: Expr) {
        let mut next_args = self.arg_stack.pop_front();

        if let Some(mut args) = next_args {
            args.push_front(expr);
            let result = self
                .environment
                .eval_list(&Vec::from(args)).unwrap();

            self.push_return(result);
        } else {
            let args = vec![expr];
            let result = self.environment.eval_list(&args).unwrap();

            self.push_return(result);
        }
    }

    fn raw_exec_instr(&mut self, expr: Vec<Expr>) {
        let mut next_args = self.arg_stack.pop_front();

        if let Some(mut args) = next_args {
            let mut prepended = VecDeque::from(expr);

            prepended.append(&mut args);

            args = prepended;

            let result = self
                .environment
                .eval_list(&Vec::from(args)).unwrap();

            self.push_return(result);
        } else {
            let result = self.environment.eval_list(&expr).unwrap();

            self.push_return(result);
        }
    }

    fn push_return(&mut self, expr: Expr) {
        if let Some(mut args) = self.arg_stack.pop_front() {
            args.push_back(expr);

            self.arg_stack.push_front(args);
        } else {
            self.arg_stack.push_front(VecDeque::from([expr]));
        }
    }

    pub fn get_result(&mut self) -> Expr {
        self.arg_stack.pop_front().unwrap().pop_front().unwrap()
    }
}
