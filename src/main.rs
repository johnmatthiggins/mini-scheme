mod lex;
mod syntax;
mod env;
mod built_in;
mod math;
mod boolean;

use std::collections::HashMap;
use std::io::Write;
use std::io;
use ansi_term::Colour::{Red, Green, Blue, Cyan, Yellow};
use crate::syntax::*;
use crate::env::EnvTrait;
use crate::env::Env;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PROMPT: &str = "~> ";

fn main() {
    print!("Mini-Scheme Version {}\n", VERSION);

    // Holds all the predefined functions and values for REPL session.
    let mut env: Env = Env {
        scopes: Vec::new(),
    };

    env.begin_scope();
    let mut failed = false;

    loop {
        let mut input = String::new();
        
        // Print prompt than promptly flush output to sync up.
        if !failed {
            print!("{}", PROMPT);
        }
        else {
            print!("{}", Red.paint(PROMPT).to_string());
        }
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

                    match result {
                        Ok(expr) => {
                            failed = false;
                            println!("{}", print_tree(&expr));
                        },
                        Err(msg) => 
                        {
                            failed = true;
                            println!("{}", Red.paint(&msg).to_string());
                        },
                    };
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
            true => Red.paint(syntax::TRUE_LIT).to_string(),
            false => Red.paint(syntax::FALSE_LIT).to_string(),
        },
        Atom::StringLiteral(s) =>
            Yellow.paint(s.to_string().replace("\\'", "'")).to_string(),
        Atom::Number(n) => Cyan.paint(n.to_string()).to_string(),
        Atom::Symbol(s) => s.to_string(),
        Atom::Nil => Red.paint(syntax::NIL_LIT.to_string()).to_string(),
        Atom::Lambda(ld) => print_lambda(ld)
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
        }
        else {
            acc.push_str(&print_tree(exp));
        }
        
        if i == expr_list.len() - 1 {
            acc.push(')');
        }
        else {
            acc.push(' ');
        }
    }

    acc
}
