use bigdecimal::BigDecimal;
use core::ops::Add;
use core::ops::Sub;
use core::ops::Mul;
use core::ops::Neg;
use crate::environment::Env;
use crate::environment::EnvTrait;
use crate::syntax::Expr;
use crate::syntax::Atom;

pub trait EnvOps {
    fn car(&mut self, expr: &Expr) -> Result<Expr, String>;

    fn cdr(&mut self, expr: &Expr) -> Result<Expr, String>;

    fn eq(&mut self, cdr: &[Expr]) -> Result<Expr, String>;

    fn add(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;

    fn sub(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;

    fn mul(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;

    fn div(&mut self, list: &Vec<Expr>);

    fn modulo(&mut self, list: &Vec<Expr>);
}

impl EnvOps for Env {
    // Return first element of list or just empty.
    fn car(&mut self, expr: &Expr) -> Result<Expr, String> {
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

    fn cdr(&mut self, expr: &Expr) -> Result<Expr, String> {
        match expr {
            Expr::Atom(_) => Result::Err(
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
                for exp in cdr.iter() {
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
                    Expr::List(_) => Result::Err(
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

    fn sub(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        // If only one argument, return negated number.
        // If multiple, start total as first arg,
        // then subtract every arg after that.
        let total: Option<BigDecimal> = args
            .first()
            .and_then(|x| match x {
                Expr::Atom(atom) => match atom {
                    Atom::Number(n) => Some(n.to_owned()),
                    _ => None
                },
                Expr::List(_) => None
            });
        
        if args.len() > 1 {
            let result = total
                .map(|n| sub_car_cdr(self, n, &args[..1].to_vec()))
                .unwrap_or(Result::Err("Cannot perform '-' operator on non-numeric type.".to_string()));

            return result;
        }
        else {
            let result = total
                .map(|x| Ok(Expr::Atom(Atom::Number(x.neg()))))
                .unwrap_or(
                    Err("Cannot perform '-' operator on non-numeric type.".to_string()));

            return result;
        }
    }

    fn mul(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        let mut total: BigDecimal = BigDecimal::from(1);
        let length = args.len();

        for expr in args.into_iter() {
            let simple_tree = self.simplify(expr);

            if simple_tree.is_ok() {
                let number = match simple_tree.unwrap() {
                    Expr::Atom(atom) => match atom {
                        Atom::Number(n) => Result::Ok(n),
                        _ => Result::Err(
                            "Non-number atom cannot have operator '*' applied to it.".to_string())
                    },
                    Expr::List(_) => Result::Err(
                        "List cannot have operator '*' applied to it.".to_string())
                };

                if number.is_ok() {
                    total = total.mul(&number.unwrap());
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

    fn div(&mut self, list: &Vec<Expr>) {
    }

    fn modulo(&mut self, list: &Vec<Expr>) {
    }
}

fn sub_car_cdr(env: &Env, car: BigDecimal, cdr: &Vec<Expr>) -> Result<Expr, String> {
    let mut total = car;

    for (i, expr) in cdr.into_iter().enumerate() {
        let simple_tree = env.to_owned().simplify(expr);

        match simple_tree {
            Ok(tree) => {
                let number = match tree {
                    Expr::Atom(atom) => match atom {
                        Atom::Number(n) => Result::Ok(n),
                        _ => Result::Err(
                            "Non-number atom cannot have operator '-' applied to it.".to_string())
                    },
                    Expr::List(_) => Result::Err(
                        "List cannot have operator '-' applied to it.".to_string())
                };

                match number {
                    Ok(n) => {
                        total = total.sub(n);
                    },
                    Err(msg) => {
                        return Result::Err(msg);
                    }
                }
            },
            Err(msg) => return Result::Err(msg)
        };
    }

    return Result::Ok(Expr::Atom(Atom::Number(total)));
}
