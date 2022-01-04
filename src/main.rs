mod lexer;
mod syntax;

use std::collections::HashMap;
use std::io::Write;
use std::io;
use crate::syntax::Expr;
use crate::syntax::Atom;

const PROMPT: &str = "λ ";

fn main() {
    // Holds all the predefined functions and values for REPL session.
    let mut env: HashMap<&String, Expr> = HashMap::new();

    loop {
        let mut input = String::new();
        
        // Print prompt than promptly flush output to sync up.
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("User input could not be read...");

        let result = eval(&input, &mut env);

        println!("{}", &result);
    }
}

fn eval(input: &String, mut env: &HashMap<&String, Expr>) -> String {
    let tokens = lexer::lexical_analysis(input);

    let result = match tokens {
        Ok(vec) => print_tree(&lexer::parse_tokens(&vec)),
        Err(e) => e
    };

    return result;
}

fn print_tree(expr_tree: &Expr) -> String {
    match expr_tree {
        Expr::List(v) => v
            .into_iter()
            .fold("(".to_string(), |acc, x| acc + &print_tree(x))
            + ")",
        Expr::Atom(atom) => print_atom(&atom)
    }
}

fn print_atom(expr_atom: &Atom) -> String {
    match expr_atom {
        Atom::Boolean(b) => match b {
            true => "#t".to_string(),
            false => "#f".to_string()
        },
        Atom::StringLiteral(s) => s.to_string(),
        Atom::Number(n) => n.to_string(),
        Atom::Symbol(s) => s.to_string()
    }
}
