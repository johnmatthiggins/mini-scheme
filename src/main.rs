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

const WELCOME: &str = "Mini-Scheme Version 0.1.0\n";

const PROMPT: &str = "> ";

fn main() {
    print!("{}\n", WELCOME);

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
            let is_comment = input
                .chars()
                .next()
                .map(|x| x == ';')
                .unwrap_or(false);
            
            if !is_comment {
                let result = env.eval(&input);

                let output = match result {
                    Ok(expr) => print_tree(&expr),
                    Err(msg) => msg
                };

                print!("{}\n", &output);
            }
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
            true => "#t".to_string(),
            false => "#f".to_string()
        },
        Atom::StringLiteral(s) => s.to_string(),
        Atom::Number(n) => n.to_string(),
        Atom::Symbol(s) => s.to_string(),
        Atom::Nil => "()".to_string()
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
