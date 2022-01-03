mod lexer;
mod syntax;

use std::collections::HashMap;
use std::io::Write;
use std::io;
use crate::syntax::Expr;

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
        Ok(vec) => vec.iter()
            .fold(String::new(), |acc, x| acc + " " + x),
        Err(e) => e
    };

    return result;
}
