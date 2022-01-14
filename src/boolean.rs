use crate::env::Env;
use crate::env::EnvTrait;
use crate::syntax::Atom;
use crate::syntax::Expr;

pub trait LogicOps {
    fn and(&mut self, args: &Vec<Expr>) -> Result<Expr, &str>;
    fn or(&mut self, args: &Vec<Expr>) -> Result<Expr, &str>;
    fn not(&mut self, args: &Vec<Expr>) -> Result<Expr, &str>;
    fn atom(&mut self, args: &Vec<Expr>) -> Result<Expr, &str>;
    fn if_op(&mut self, args: &Vec<Expr>) -> Result<Expr, &str>;
}

impl LogicOps for Env {
    // Returns true if no arguments.
    fn and(&mut self, args: &Vec<Expr>) -> Result<Expr, &str> {
        let mut bool_result = true;

        for expr in args.iter() {
            let tree = self.simplify(&expr);

            let result = tree
                .and_then(|x| try_get_bool(&x));

            match result {
                Ok(b) => {
                    bool_result = bool_result && b;
                },
                Err(msg) => {
                    return Err(msg);
                }
            }
        }

        return Ok(Expr::Atom(Atom::Boolean(bool_result)));
    }

    fn or(&mut self, args: &Vec<Expr>) -> Result<Expr, &str> {
        let mut bool_result = false;

        for expr in args.iter() {
            let tree = self.simplify(&expr);

            let result = tree
                .and_then(|x| try_get_bool(&x));

            match result {
                Ok(b) => {
                    bool_result = bool_result || b;
                },
                Err(msg) => {
                    return Err(msg);
                }
            }
        }

        return Ok(Expr::Atom(Atom::Boolean(bool_result)));
    }

    fn not(&mut self, args: &Vec<Expr>) -> Result<Expr, &str> {
        if args.len() != 1 {
            return Err("Incorrect number of args for 'not' operator.");
        }
        else {
            let expr = &args[0];
            let tree = self.simplify(expr);

            let result = tree
                .and_then(|x| try_get_bool(&x))
                .map(|x| Expr::Atom(Atom::Boolean(!x)));

            return result;
        }
    }

    fn atom(&mut self, args: &Vec<Expr>) -> Result<Expr, &str> {
        if args.len() != 1 {
            return Err("Incorrect number of args for 'atom' operator.");
        }
        else {
            let expr = &args[0];
            let result = self.simplify(expr)
                .map(|x| match x {
                    Expr::Atom(_) => Expr::Atom(Atom::Boolean(true)),
                    Expr::List(_) => Expr::Atom(Atom::Boolean(false))
                });

            return result;
        }
    }

    fn if_op(&mut self, args: &Vec<Expr>) -> Result<Expr, &str> {
        if args.len() == 3 {
            let exp0 = self.simplify(&args[0]);

            let return_exp = exp0.and_then(|x| try_get_bool(&x))
                .and_then(|x| {
                    match x {
                        true => self.simplify(&args[1]),
                        false => self.simplify(&args[2])
                    }
                });

            return return_exp;
        }
        else {
            return Err("Incorrect number of arguments for 'if' operator.");
        }
    }
}

fn try_get_bool(expr: &Expr) -> Result<bool, &str> {
    match expr {
        Expr::Atom(atom) => match atom {
            Atom::Boolean(b) => Ok(b.to_owned()),
            _ => Err("Cannot perform operation on non-boolean value.")
        }
        Expr::List(_) => Err("Cannot perform operation on non-boolean value.")
    }
}
