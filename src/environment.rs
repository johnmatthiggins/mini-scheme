use std::collections::HashMap;
use crate::lexer;
use crate::syntax::Expr;
use crate::syntax::Atom;

pub type Env = HashMap<String, Expr>;

pub struct SymbolMapping {
    name: String,
    expr: Atom
}

pub trait EnvTrait {
    fn map_symbol(&self, symbol: &String) -> Option<&Expr>;
    fn eval(&mut self, input: &String) -> Result<Expr, String>;
    fn eval_list(list: &Vec<Expr>) -> Result<Expr, String>;
    fn eval_car_cdr(&mut self, car: Atom, cdr: &Vec<Expr>) -> Result<Expr, String>;
    fn apply(&mut self, name: &String, args: &Vec<Expr>) -> Result<Expr, String>;
    fn simplify(&mut self, expr: &Expr) -> Result<Expr, String>;
}

impl EnvTrait for Env {
    fn map_symbol(&self, symbol: &String) -> Option<&Expr> {
        return self.get(symbol);
    }

    fn eval(&mut self, input: &String) -> Result<Expr, String>{
        let tokens = lexer::lexical_analysis(input);
        let ast = lexer::parse_tokens(&tokens);

        let result = self.simplify(&ast);
        
        return result;
    }

    fn eval_list(list: &Vec<Expr>) -> Result<Expr, String> {
        // Evaluate list expression.
        let mut simplified_list:
            Vec<Result<Expr, String>> = Vec::new();

        let car = list.first();

        match car {
            Some(expr) => match expr {
                Expr::Atom(atom)
                    => eval_car_cdr(atom, list[1..].to_vec()),
                Expr::List(list)
                    => Result::Err("First token in list must be function name.".to_string()
            }
            None => Result::Err("Empty list is not a valid token.".to_string())
        }
    }

    fn eval_car_cdr(&mut self, car: Atom, cdr: &Vec<Expr>) -> Result<Expr, String> {
        match car {
            Atom::Symbol(name) => apply(car, cdr),
            _ => Result::Err("First token in list must be function name.".to_string())
        }
    }

    fn apply(&mut self, name: &String, args: &Vec<Expr>) -> Result<Expr, String> {
        // Match functions to their name and return a function not found error
        // if it doesn't exist in the environment or in built in functions.
        return Result::Err("Good job! You reached the end of the level!".to_string());
    }

    fn simplify(&mut self, expr: &Expr) -> Result<Expr, String> {
        match expr {
            Expr::List(list) => eval_list(self, list),
            // Don't worry about atoms right now.
            Expr::Atom(atom) => Result::Ok(Expr::Atom(atom))
        }
    }
}
