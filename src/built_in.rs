use crate::syntax::Expr;
use crate::environment::Env;
use crate::environment::EnvTrait;

pub trait EnvPrimitives {
    fn quote(&mut self, expr: &Vec<Expr>) -> Result<Expr, String>;
    fn car(&mut self, expr: &Vec<Expr>) -> Result<Expr, String>;
    fn cdr(&mut self, expr: &Vec<Expr>) -> Result<Expr, String>;
}

impl EnvPrimitives for Env {
    fn quote(&mut self, expr: &Vec<Expr>) -> Result<Expr, String> {
        if expr.len() != 1 {
            return Err("Incorrect argument count for 'quote' operator.".to_string());
        }
        else {
            return Ok(expr[0].to_owned());
        }
    }

    // Functions that don't require access to environment.
    fn car(&mut self, expr: &Vec<Expr>) -> Result<Expr, String> {
        if expr.len() != 1 {
            return Err("Incorrect argument count for 'car' operator.".to_string());
        }
        else {
            let tree = self.simplify(&expr[0]);

            let result = match tree {
                Ok(v) => car_exp(&v),
                Err(msg) => Err(msg)
            };

            return result;
        }
    }

    fn cdr(&mut self, expr: &Vec<Expr>) -> Result<Expr, String> {
        // If no elements in list throw error.
        // If more than one arg throw error.
        // If first arg is not list throw error.
        if expr.len() != 1 {
            return Err("Incorrect argument count for 'cdr' operator.".to_string());
        }
        else {
            let tree = self.simplify(&expr[0]);
            let result = match tree {
                Ok(v) => cdr_exp(&v),
                Err(msg) => Err(msg)
            };

            return result;
        }
    }
}

// Return first element of list or just empty.
fn car_exp(expr: &Expr) -> Result<Expr, String> {
    match expr {
        Expr::Atom(_) => Result::Err(
            "'car' can only be applied to lists.".to_string()),
        Expr::List(list) => match list.first() {
            Some(element) => Result::Ok(element.to_owned()),
            None => Result::Err(
                "'car' cannot be applied to empty lists.".to_string())
        }
    }
}

// Return elements after first.
fn cdr_exp(expr: &Expr) -> Result<Expr, String> {
    match expr {
        Expr::Atom(_) => Result::Err(
            "'cdr' can only be applied to lists.".to_string()),
        Expr::List(list) => Result::Ok(Expr::List(list[1..].to_vec()))
    }
}
