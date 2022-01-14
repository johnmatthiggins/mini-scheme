use crate::env::Env;
use crate::env::EnvTrait;
use crate::syntax::Atom;
use crate::syntax::Expr;
use crate::syntax::LambdaDef;

pub trait EnvPrimitives {
    fn define(&mut self, expr: &Vec<Expr>) -> Result<Expr, &str>;
    fn quote(&mut self, expr: &Vec<Expr>) -> Result<Expr, &str>;
    fn car(&mut self, expr: &Vec<Expr>) -> Result<Expr, &str>;
    fn cdr(&mut self, expr: &Vec<Expr>) -> Result<Expr, &str>;
    fn lambda(&mut self, expr: &Vec<Expr>) -> Result<Expr, &str>;
}

impl EnvPrimitives for Env {
    fn define(&mut self, expr: &Vec<Expr>) -> Result<Expr, &str> {
        if expr.len() != 2 {
            return Err("Incorrect number of arguments for 'define' operator.");
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

    fn quote(&mut self, expr: &Vec<Expr>) -> Result<Expr, &str> {
        if expr.len() != 1 {
            return Err("Incorrect argument count for 'quote' operator.");
        }
        else {
            return Ok(expr[0].to_owned());
        }
    }

    // Functions that don't require access to environment.
    fn car(&mut self, expr: &Vec<Expr>) -> Result<Expr, &str> {
        if expr.len() != 1 {
            return Err("Incorrect argument count for 'car' operator.");
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

    fn cdr(&mut self, expr: &Vec<Expr>) -> Result<Expr, &str> {
        // If no elements in list throw error.
        // If more than one arg throw error.
        // If first arg is not list throw error.
        if expr.len() != 1 {
            return Err("Incorrect argument count for 'cdr' operator.");
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

    fn lambda(&mut self, expr: &Vec<Expr>) -> Result<Expr, &str> {
        if expr.len() != 2 {
            return Err("Incorrect argument count for 'lambda' operator.");
        }
        else {
            let param_expr = &expr[0];
            let params =
                match param_expr {
                    Expr::Atom(v) => vec![Expr::Atom(v.to_owned())],
                    Expr::List(l) => l.to_vec()
                };

            let body = Box::new(expr[1].to_owned());

            let result = LambdaDef {
                    params: params,
                    body: body
                };
            
            return Ok(Expr::Atom(Atom::Lambda(result)));
        }
    }
}

// Return first element of list or just empty.
fn car_exp(expr: &Expr) -> Result<Expr, &str> {
    match expr {
        Expr::Atom(_) => Result::Err(
            "'car' can only be applied to lists."),
        Expr::List(list) => match list.first() {
            Some(element) => Result::Ok(element.to_owned()),
            None => Result::Err("'car' cannot be applied to empty lists.")
        }
    }
}

// Return elements after first.
fn cdr_exp(expr: &Expr) -> Result<Expr, &str> {
    match expr {
        Expr::Atom(_) => Result::Err("'cdr' can only be applied to lists."),
        Expr::List(list) => Result::Ok(Expr::List(list[1..].to_vec()))
    }
}

fn try_get_symbol_string(expr: &Expr) -> Result<String, &str> {
    match expr {
        Expr::Atom(atom) => match atom {
            Atom::Symbol(s) => Ok(s.to_owned()),
            _ => Err("Invalid symbol name.")
        },
        Expr::List(_) => Err("List is not a valid symbol name")
    }
}
