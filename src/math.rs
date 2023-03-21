use crate::env::{Env, Eval};
use crate::syntax::{Atom, Expr};
use bigdecimal::BigDecimal;
use core::ops::Rem;

pub trait MathOps {
    fn eq(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
    fn add(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
    fn sub(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
    fn mul(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
    fn div(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
    fn modulo(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
}

struct OpInfo {
    name: String,
    op_fn: fn(&BigDecimal, &BigDecimal) -> BigDecimal,
    default: BigDecimal,
}

impl MathOps for Env {
    fn eq(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        let first = args
            .first()
            .map(|x| Result::Ok(x))
            .unwrap_or(Result::Err(
                "Operator '=' must have at least one argument.".to_string(),
            ))
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
                        }
                        Err(msg) => {
                            return Err(msg);
                        }
                    }
                }

                Result::Ok(Expr::Atom(Box::new(Atom::Boolean(is_eq))))
            }
            Err(msg) => Result::Err(msg),
        }
    }

    // elementary functions of math.

    fn add(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        //dbg!("MADE IT TO THE ADD FUNCTION");
        let mut total: Box<BigDecimal> = Box::new(BigDecimal::from(0));

        for expr in args.into_iter() {
            //dbg!("TIME TO SIMPLIFY");
            let simple_tree = self.simplify(expr);
            //dbg!("DONE SIMPLIFYING");

            if simple_tree.is_ok() {
                let number = match simple_tree.unwrap() {
                    Expr::Atom(atom) => match *atom {
                        Atom::Number(n) => Result::Ok(n),
                        _ => Result::Err(
                            "Non-number atom cannot have operator '+' applied to it.".to_string(),
                        ),
                    },
                    Expr::List(_) => {
                        Result::Err("List cannot have operator '+' applied to it.".to_string())
                    }
                };

                match number {
                    Ok(v) => {
                        *total += v;
                    }
                    Err(msg) => {
                        return Result::Err(msg);
                    }
                }
            } else {
                return Result::Err("Error".to_string());
            }
        }

        Result::Ok(Expr::Atom(Box::new(Atom::Number(*total))))
    }

    fn sub(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        let op_info = OpInfo {
            name: String::from("-"),
            op_fn: |a, b| a - b,
            default: BigDecimal::from(0),
        };

        return apply_neg_op(&self, args, &op_info);
    }

    fn mul(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        let mut total: BigDecimal = BigDecimal::from(1);

        for expr in args.into_iter() {
            let simple_tree = self.simplify(expr);

            match simple_tree {
                Ok(v) => {
                    let number = match v {
                        Expr::Atom(atom) => match *atom {
                            Atom::Number(n) => Result::Ok(n),
                            _ => Result::Err(
                                "Non-number atom cannot have operator '*' applied to it."
                                    .to_string(),
                            ),
                        },
                        Expr::List(_) => {
                            Result::Err("List cannot have operator '*' applied to it.".to_string())
                        }
                    };

                    if number.is_ok() {
                        total = total * number.unwrap();
                    } else {
                        return Result::Err("Error".to_string());
                    }
                }
                Err(msg) => {
                    return Err(msg);
                }
            }
        }

        Result::Ok(Expr::Atom(Box::new(Atom::Number(total))))
    }

    fn div(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        let op_info = OpInfo {
            name: String::from("/"),
            op_fn: |a, b| a / b,
            default: BigDecimal::from(1),
        };

        return apply_neg_op(&self, args, &op_info);
    }

    fn modulo(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        if args.len() == 2 {
            // Specifies default but it'll never actually be used.
            let op_info = OpInfo {
                name: String::from("%"),
                op_fn: |a, b| a.rem(b),
                default: BigDecimal::from(69),
            };

            let result = apply_neg_op(self, args, &op_info);

            return result;
        } else {
            return Err("Incorrect argument count for '%' operator.".to_string());
        }
    }
}

fn apply_neg_op(env: &Env, args: &Vec<Expr>, info: &OpInfo) -> Result<Expr, String> {
    let mut local_env = env.to_owned();
    let total = args
        .first()
        .ok_or(format!("Incorrect argument count for '{}' operator.", info.name).to_string())
        .and_then(|x| local_env.simplify(x))
        .and_then(|x| match x {
            Expr::Atom(atom) => match *atom {
                Atom::Number(n) => Ok(n.to_owned()),
                _ => Err(format!(
                    "Cannot perform '{}' operator on non-numeric type.",
                    info.name
                )
                .to_string()),
            },
            Expr::List(_) => Err(format!(
                "Cannot perform '{}' operator on non-numeric type.",
                info.name
            )
            .to_string()),
        });

    if args.len() > 1 {
        let result = total.and_then(|n| {
            apply_first_rest(&local_env, n, &args[1..].to_vec(), info.op_fn, &info.name)
        });

        return result;
    } else {
        let result = total
            // .map(|x| Ok(Expr::Atom(Atom::Number(BigDecimal::from(1) / x))))
            .map(|x| Ok(Expr::Atom(Box::new(Atom::Number((info.op_fn)(&info.default, &x))))))
            .unwrap_or(Err(format!(
                "Cannot perform '{}' operator on non-numeric type.",
                info.name
            )
            .to_string()));

        return result;
    }
}

fn apply_first_rest(
    env: &Env,
    car: BigDecimal,
    cdr: &Vec<Expr>,
    op: fn(&BigDecimal, &BigDecimal) -> BigDecimal,
    op_name: &str,
) -> Result<Expr, String> {
    let mut total = car;
    let mut local_env = env.clone();

    for expr in cdr.into_iter() {
        let simple_tree = local_env.simplify(expr);

        match simple_tree {
            Ok(tree) => {
                let number = match tree {
                    Expr::Atom(atom) => match *atom {
                        Atom::Number(n) => Result::Ok(n),
                        _ => Result::Err(
                            format!(
                                "Non-number atom cannot have operator '{}' applied to it.",
                                op_name
                            )
                            .to_string(),
                        ),
                    },
                    Expr::List(_) => Result::Err(
                        format!("List cannot have operator '{}' applied to it.", op_name)
                            .to_string(),
                    ),
                };

                match number {
                    Ok(n) => {
                        total = op(&total, &n);
                    }
                    Err(msg) => {
                        return Result::Err(msg);
                    }
                }
            }
            Err(msg) => return Result::Err(msg),
        };
    }

    Result::Ok(Expr::Atom(Box::new(Atom::Number(total))))
}
