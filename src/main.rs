mod lex;
mod math_ops;
mod syntax;
mod boolean;
mod built_in;

use std::io;
use std::io::Write;
use std::collections::HashMap;
use crate::syntax::Expr;
use crate::syntax::Atom;
use crate::syntax::LambdaDef;

// Get string from actual config file.
const VERSION: &str = env!("CARGO_PKG_VERSION");

const PROMPT: &str = "> ";

fn main() {
    print!("Mini-Scheme Version {}\n\n", VERSION);

    // Holds all the predefined functions and values for REPL session.
    // let mut scope: Scope = HashMap::new();

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
                    // Convert to syntax tree.
                    // Evaluate syntax tree using traversal.
                    let output = String::from("THIS PROGRAM DOESNT DO ANYTHING");
                    
                    // Print out expression.
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

    return acc;
}
