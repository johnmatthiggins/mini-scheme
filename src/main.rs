mod stack_encode;
mod stack;
mod boolean;
mod built_in;
mod env;
mod lex;
mod math;
mod syntax;
mod sys;

use crate::env::Env;
use crate::env::Eval;
use crate::syntax::*;
use crate::stack::StackMachine;
use ansi_term::Colour::{Blue, Green, Purple, Red, Yellow};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;
use std::io::Write;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::env::args;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PROMPT: &str = "~> ";

fn main() {
    let args: Vec<String> = args().collect();

    if let Some(arg) = args.get(1) {
        if arg.as_str() == "--repl" {
            repl_mode();
        } else {
            interpreter_mode(&arg);
        }
    } else {
        panic!("Invalid arguments...");
    }
}

fn repl_mode() {
    print!("Mini-Scheme Version {}\n", VERSION);

    // Holds all the predefined functions and values for REPL session.
    let mut env: Env = HashMap::new();
    let mut sm = stack::StackMachine::create(env);
    let mut failed = false;

    loop {
        let mut input = String::new();

        // Print prompt than promptly flush output to sync up.
        if !failed {
            print!("{}", PROMPT);
        } else {
            print!("{}", Red.paint(PROMPT).to_string());
        }

        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("User input could not be read...");

        input = trim_newline(&mut input);

        if input.len() > 0 {
            match input.as_str() {
                "exit" => {
                    print!("\n");
                    break;
                },
                _ => {
                    let first_char = input.chars().next().unwrap();

                    if first_char != ';' {
                        // let result = env.eval(&input);
                        let instructions = lex::lexical_analysis(&input)
                            .map(|x| lex::parse_tokens(&x))
                            .map(|x| stack_encode::encode_ast(&x))
                            .unwrap();

                        // dbg!(&instructions);

                        let result = sm.run_instructions(instructions);

                        println!("{}", print_tree(&result));

                        // match result {
                        //     Ok(expr) => {
                        //         failed = false;
                        //         println!("{}", print_tree(&expr));
                        //     }
                        //     Err(msg) => {
                        //         failed = true;
                        //         println!("{}", &msg);
                        //     }
                        // };
                    }
                }
            }
        }
    }
}

fn interpret_line(input: &String, sm: &mut StackMachine) {
    if input.len() > 0 {
        let first_char = input.chars().next().unwrap();

        if first_char != ';' {
            // let result = env.eval(&input);
            let instructions = lex::lexical_analysis(&input)
                .map(|x| lex::parse_tokens(&x))
                .map(|x| stack_encode::encode_ast(&x))
                .unwrap();

            // dbg!(&instructions);

            let result = sm.run_instructions(instructions);

            // match result {
            //     Ok(expr) => {
            //         failed = false;
            //         println!("{}", print_tree(&expr));
            //     }
            //     Err(msg) => {
            //         failed = true;
            //         println!("{}", &msg);
            //     }
            // };
        }
    }
}

fn interpreter_mode(path: &String) {
    let mut env: Env = HashMap::new();
    let mut sm = stack::StackMachine::create(env);
    let mut failed = false;

    if let Ok(lines) = read_lines(path.as_str()) {
        for line in lines {
            if let Ok(ip) = line {
                interpret_line(&ip, &mut sm);
            }
        }
    }
}

fn print_tree(expr_tree: &Expr) -> String {
    match expr_tree {
        Expr::List(v) => print_list(v),
        Expr::Atom(atom) => print_atom(&atom),
    }
}

fn print_atom(expr_atom: &Atom) -> String {
    let result = match expr_atom {
        Atom::Boolean(b) => match b {
            true => Red.paint(syntax::TRUE_LIT).to_string(),
            false => Red.paint(syntax::FALSE_LIT).to_string(),
        },
        Atom::StringLiteral(s) => Yellow.paint(s.to_string()).to_string(),
        Atom::Number(n) => Red.paint(n.to_string()).to_string(),
        Atom::Symbol(s) => s.to_string(),
        Atom::Nil => Red.paint(syntax::NIL_LIT.to_string()).to_string(),
        Atom::Lambda(ld) => print_lambda(ld),
    };

    result
}

fn print_lambda(lambda: &LambdaDef) -> String {
    // create new string with lambda at start.
    let mut acc = String::new();

    acc.push('(');
    acc.push_str(syntax::FUN_OP);
    acc.push(' ');
    acc.push_str(&print_list(&lambda.params));
    acc.push(' ');
    acc.push_str(&print_tree(&*lambda.body));
    acc.push(')');

    acc
}

fn print_list(expr_list: &Vec<Expr>) -> String {
    let mut acc = String::new();

    for (i, exp) in expr_list.iter().enumerate() {
        if i == 0 {
            acc.push('(');
            acc.push_str(&print_tree(exp));
        } else {
            acc.push_str(&print_tree(exp));
        }

        if i == expr_list.len() - 1 {
            acc.push(')');
        } else {
            acc.push(' ');
        }
    }

    acc
}

fn trim_newline(s: &mut String) -> String {
    let mut trimmed = s.clone();

    if trimmed.ends_with('\n') {
        trimmed.pop();
        if trimmed.ends_with('\r') {
            trimmed.pop();
        }
    }

    trimmed
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
