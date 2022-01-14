mod lex;
mod syntax;
mod env;
mod built_in;
mod math;
mod boolean;

use std::collections::HashMap;
use std::io::Write;
use std::io;
use crate::syntax::Expr;
use crate::syntax::Atom;
use crate::env::EnvTrait;
use crate::env::Env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const PROMPT: &str = "> ";

fn main() {
    print!("Mini-Scheme Version {}\n\n", VERSION);

    // Holds all the predefined functions and values for REPL session.
    let mut env: Env = HashMap::new();

    loop {
        let mut input = String::new();
        
        // Print prompt than promptly flush output to sync up.
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("User input could not be read...");

        if input.len() > 0 {
            if input != "\n" {
                let first_char = input
                    .chars()
                    .next()
                    .unwrap();

                if first_char != ';' {
                    let result = env.eval(&input);

                    let output = match result {
                        Ok(expr) => print_tree(&expr),
                        Err(msg) => msg
                    };

                    print!("{}\n", &output);
                }
            }
        }
        else {
            print!("\n");
            break;
        }
    }
}

fn print_tree(expr_tree: &Expr) -> String {
    match expr_tree {
        Expr::List(v) => print_list(v),
        Expr::Atom(atom) => print_atom(&atom)
    }
}

fn print_atom(expr_atom: &Atom) -> String {
    let result = match expr_atom {
        Atom::Boolean(b) => match b {
            true => crate::syntax::TRUE_LIT.to_string(),
            false => crate::syntax::FALSE_LIT.to_string()
        },
        Atom::StringLiteral(s) => s.to_string(),
        Atom::Number(n) => n.to_string(),
        Atom::Symbol(s) => s.to_string(),
        Atom::Nil => crate::syntax::NIL_LIT.to_string(),
        // TODO: Fix this and add compatibility for LAMBDA
        _ => "I guess we're just gonna blow up now.".to_string()
    };

    return result;
}

fn print_list(expr_list: &Vec<Expr>) -> String {
    let mut acc = String::new();

    for (i, exp) in expr_list.iter().enumerate() {
        if i == 0 {
            acc.push('(');
            acc.push_str(&print_tree(exp));
            acc.push(' ');
        }
        else if i == expr_list.len() - 1 {
            acc.push_str(&print_tree(exp));
            acc.push(')');
        }
        else {
            acc.push_str(&print_tree(exp));
            acc.push(' ');
        }
    }

    return acc;
}
