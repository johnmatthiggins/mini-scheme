use bigdecimal::BigDecimal;
use core::ops::Add;
use crate::environment::Env;
use crate::environment::EnvTrait;
use crate::syntax::Expr;
use crate::syntax::Atom;

pub trait EnvOps {
    fn car(&mut self, expr: &Expr) -> Result<Expr, String>;

    fn cdr(&mut self, expr: &Expr) -> Result<Expr, String>;

    fn eq(&mut self, cdr: &[Expr]) -> Result<Expr, String>;

    fn add(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;

    fn sub_list(&mut self, args: &Vec<Expr>);

    fn mul_list(&mut self, list: &Vec<Expr>);

    fn div_list(&mut self, list: &Vec<Expr>);

    fn mod_list(&mut self, list: &Vec<Expr>);
}

impl EnvOps for Env {
    // Return first element of list or just empty.
    fn car(&mut self, expr: &Expr) -> Result<Expr, String> {
        match expr {
            Expr::Atom(atom) => Result::Err(
                "'car' can only be applied to lists.".to_string()),
            Expr::List(list) => match list.first() {
                Some(element) => Result::Ok(element.to_owned()),
                None => Result::Err(
                    "'car' cannot be applied to empty lists.".to_string())
            }
        }
    }

    fn cdr(&mut self, expr: &Expr) -> Result<Expr, String> {
        match expr {
            Expr::Atom(a) => Result::Err(
                "'cdr' can only be applied to lists.".to_string()),
            Expr::List(list) => Result::Ok(Expr::List(list[1..].to_vec()))
        }
    }

    fn eq(&mut self, cdr: &[Expr]) -> Result<Expr, String> {
        let first = cdr.first();

        match first {
            Some(s) => {
                let mut is_eq = true;
                
                // Loop through all of them and make sure
                // they are equal to each other.
                for (i, exp) in cdr.iter().enumerate() {
                    is_eq = is_eq && s == exp;
                }

                return Result::Ok(Expr::Atom(Atom::Boolean(is_eq)));
            }
            None => Result::Err(
                "Incorrect argument count for '=' operator.".to_string()),
        }
    }

    // elementary functions of math.

    fn add(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        let mut total: BigDecimal = BigDecimal::from(0);
        let length = args.len();

        for expr in args.into_iter() {
            let simple_tree = self.simplify(expr);

            if simple_tree.is_ok() {
                let number = match simple_tree.unwrap() {
                    Expr::Atom(atom) => match atom {
                        Atom::Number(n) => Result::Ok(n),
                        _ => Result::Err(
                            "Non-number atom cannot have operator '+' applied to it.".to_string())
                    },
                    Expr::List(list) => Result::Err(
                        "List cannot have operator '+' applied to it.".to_string())
                };

                if number.is_ok() {
                    total = total.add(&number.unwrap());
                }
                else {
                    return Result::Err("Error".to_string());
                }
            }
            else {
                return Result::Err("Error".to_string());
            }
        }

        return Result::Ok(Expr::Atom(Atom::Number(total)));
    }

    fn sub_list(&mut self, args: &Vec<Expr>) {
    }

    fn mul_list(&mut self, list: &Vec<Expr>) {
    }

    fn div_list(&mut self, list: &Vec<Expr>) {
    }

    fn mod_list(&mut self, list: &Vec<Expr>) {
    }
}
