use crate::env::Env;
use crate::env::EnvTrait;
use crate::syntax::Atom;
use crate::syntax::Expr;
use crate::syntax::LambdaDef;

pub trait EnvPrimitives {
    fn define(&mut self, expr: &Vec<Expr>) -> Result<Expr, String>;
    fn quote(&mut self, expr: &Vec<Expr>) -> Result<Expr, String>;
    fn car(&mut self, expr: &Vec<Expr>) -> Result<Expr, String>;
    fn cdr(&mut self, expr: &Vec<Expr>) -> Result<Expr, String>;
    fn lambda(&mut self, expr: &Vec<Expr>) -> Result<Expr, String>;
}

impl EnvPrimitives for Env {
    fn define(&mut self, expr: &Vec<Expr>) -> Result<Expr, String> {
        if expr.len() != 2 {
            return Err("Incorrect number of arguments for 'define' operator.".to_string());
        }
        else {
            // Add new symbol definition to environment.
            let current_expr = &expr[0];
            let symbol_def = &expr[1];
            let symbol = try_get_symbol_string(current_expr);

            let result = symbol.map(|x| {
                self.insert(x.to_owned(), symbol_def.to_owned());

                return Expr::Atom(Atom::Symbol(x));
            });

            return result;
        }
    }

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

    fn lambda(&mut self, expr: &Vec<Expr>) -> Result<Expr, String> { 
        if expr.len() != 2 {
            return Err("Incorrect argument count for 'lambda' operator.".to_string());
        }
        else {
            let param_expr = self.simplify(&expr[0]);
            let params = param_expr
                .map(|x| match x {
                    Expr::Atom(v) => vec![Expr::Atom(v.to_owned())],
                    Expr::List(l) => l
                });

            let body = Box::new(expr[1].to_owned());

            let result = params
                .map(|p| LambdaDef {
                    params: p,
                    body: body
                })
                .map(|x| Expr::Atom(Atom::Lambda(x)));

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

fn try_get_symbol_string(expr: &Expr) -> Result<String, String> {
    match expr {
        Expr::Atom(atom) => match atom {
            Atom::Symbol(s) => Ok(s.to_owned()),
            _ => Err("Invalid symbol name.".to_string())
        },
        Expr::List(_) => Err("List is not a valid symbol name".to_string())
    }
}
