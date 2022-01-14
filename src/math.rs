use bigdecimal::BigDecimal;
use core::ops::Rem;
use core::ops::Neg;
use crate::env::Env;
use crate::env::EnvTrait;
use crate::syntax::Expr;
use crate::syntax::Atom;

pub trait MathOps {
    fn eq(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
    fn add(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
    fn sub(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
    fn mul(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
    fn div(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
    fn modulo(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
}

impl MathOps for Env {
    fn eq(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        let first = args.first()
            .map(|x| Result::Ok(x))
            .unwrap_or(Result::Err("Operator '=' must have at least one argument.".to_string()))
            .and_then(|x| self.simplify(x));

        match first {
            Ok(s) => {
                let mut is_eq = true;
                
                // Loop through all of them and make sure
                // they are equal to each other.
                for expr in args[1..].iter() {
                    let simple_exp = self.simplify(&expr);

                    match simple_exp {
                        Ok(v) => {
                            is_eq = is_eq && s == v;
                        },
                        Err(msg) => {
                            return Err(msg);
                        }
                    }
                }

                return Result::Ok(Expr::Atom(Atom::Boolean(is_eq)));
            }
            Err(msg) => Result::Err(msg),
        }
    }

    // elementary functions of math.

    fn add(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        let mut total: BigDecimal = BigDecimal::from(0);

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

                // if number.is_ok() {
                //     total = total + number.unwrap();
                // }
                // else {
                //     return Result::Err("Error".to_string());
                // }
                
                match number {
                    Ok(v) => {
                        total += v;
                    },
                    Err(msg) => {
                        return Result::Err(msg);
                    }
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
                .map(|n| apply_first_rest(self, n, &args[1..].to_vec(), |a, b| a - b, "-"))
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

        for expr in args.into_iter() {
            let simple_tree = self.simplify(expr);

            match simple_tree {
                Ok(v) => {
                    let number = match v {
                        Expr::Atom(atom) => match atom {
                            Atom::Number(n) => Result::Ok(n),
                            _ => Result::Err(
                                "Non-number atom cannot have operator '*' applied to it.".to_string())
                        },
                        Expr::List(_) => Result::Err(
                            "List cannot have operator '*' applied to it.".to_string())
                    };

                    if number.is_ok() {
                        total = total * number.unwrap();
                    }
                    else {
                        return Result::Err("Error".to_string());
                    }
                },
                Err(msg) => {
                    return Err(msg);
                }
            }
        }

        return Result::Ok(Expr::Atom(Atom::Number(total)));
    }

    fn div(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
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
                .map(|n| apply_first_rest(self, n, &args[1..].to_vec(), |a, b| a / b, "/"))
                .unwrap_or(Result::Err("Cannot perform '/' operator on non-numeric type.".to_string()));

            return result;
        }
        else {
            let result = total
                .map(|x| Ok(Expr::Atom(Atom::Number(BigDecimal::from(1) / x))))
                .unwrap_or(
                    Err("Cannot perform '/' operator on non-numeric type.".to_string()));

            return result;
        }
    }

    fn modulo(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        let total: Option<BigDecimal> = args
            .first()
            .and_then(|x| match x {
                Expr::Atom(atom) => match atom {
                    Atom::Number(n) => Some(n.to_owned()),
                    _ => None
                },
                Expr::List(_) => None
            });
        
        if args.len() == 2 {
            let result = total
                .map(|n| apply_first_rest(self, n, &args[1..].to_vec(), |a, b| a.rem(b), "%"))
                .unwrap_or(Result::Err("Cannot perform '%' operator on non-numeric type.".to_string()));

            return result;
        }
        else {
            return Err("Incorrect argument count for '%' operator.".to_string());
        }
    }
}

fn apply_first_rest(env: &Env, car: BigDecimal, cdr: &Vec<Expr>,
        op: fn(BigDecimal, BigDecimal) -> BigDecimal, op_name: &str) -> Result<Expr, String> {
    let mut total = car;
    let mut local_env = env.clone();

    for expr in cdr.into_iter() {
        let simple_tree = local_env.simplify(expr);

        match simple_tree {
            Ok(tree) => {
                let number = match tree {
                    Expr::Atom(atom) => match atom {
                        Atom::Number(n) => Result::Ok(n),
                        _ => Result::Err(
                            format!("Non-number atom cannot have operator '{}' applied to it.", op_name)
                                .to_string())
                    },
                    Expr::List(_) => Result::Err(
                        format!("List cannot have operator '{}' applied to it.", op_name).to_string())
                };

                match number {
                    Ok(n) => {
                        total = op(total, n);
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
