use std::collections::HashMap;
use core::ops::Add;
use core::ops::Sub;
use crate::lexer;
use crate::syntax::Expr;
use crate::syntax::Atom;
use crate::ops::EnvOps;

pub type Env = HashMap<String, Expr>;

// pub struct SymbolMapping {
//     name: String,
//     expr: Atom
// }

pub trait EnvTrait {
    fn map_symbol(&self, symbol: &String) -> Option<&Expr>;
    fn eval(&mut self, input: &String) -> Result<Expr, String>;
    fn eval_list(&mut self, list: &Vec<Expr>) -> Result<Expr, String>;
    fn eval_car_cdr(&mut self, car: Atom, cdr: &Vec<Expr>) -> Result<Expr, String>;
    fn apply(&mut self, func: &String, args: &Vec<Expr>) -> Result<Expr, String>;
    fn simplify(&mut self, expr: &Expr) -> Result<Expr, String>;
}

impl EnvTrait for Env {
    fn map_symbol(&self, symbol: &String) -> Option<&Expr> {
        return self.get(symbol);
    }

    fn eval(&mut self, input: &String) -> Result<Expr, String> {
        let ast = lexer::lexical_analysis(input)
            .map(|x| lexer::parse_tokens(&x))
            .and_then(|x| self.simplify(&x));

        return ast;
    }

    fn eval_list(&mut self, list: &Vec<Expr>) -> Result<Expr, String> {
        let car = list.first();

        match car {
            Some(expr) => match expr {
                Expr::Atom(atom)
                    => self.eval_car_cdr(atom.to_owned(), &list[1..].to_vec()),
                Expr::List(list)
                    => Result::Err("First token in list must be function name.".to_string())
            },
            None => Result::Err("Empty list is not a valid token.".to_string())
        }
    }

    fn eval_car_cdr(&mut self, car: Atom, cdr: &Vec<Expr>) -> Result<Expr, String> {
        match car {
            Atom::Symbol(name) => self.apply(&name, cdr),
            _ => Result::Err("First token in list must be function name.".to_string())
        }
    }

    fn apply(&mut self, func: &String, args: &Vec<Expr>) -> Result<Expr, String> {
        // Match functions to their name and return a function not found error
        // if it doesn't exist in the environment or in built in functions.
        // return Result::Err("Good job! You reached the end of the level!".to_string());
        
        match func.as_str() {
            "+" => self.add(args),
            "-" => self.sub(args),
            _ => Result::Err("Function name not recognized.".to_string())
        }
    }

    fn simplify(&mut self, expr: &Expr) -> Result<Expr, String> {
        match expr {
            Expr::List(list) => self.eval_list(list),
            // Don't worry about atoms right now.
            Expr::Atom(atom) => Result::Ok(Expr::Atom(atom.to_owned()))
        }
    }
}
