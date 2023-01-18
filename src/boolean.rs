use crate::env::Env;
use crate::env::Eval;
use crate::syntax::Atom;
use crate::syntax::Expr;

pub trait LogicOps {
    fn and(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
    fn or(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
    fn not(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
    fn atom(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
    fn if_op(&mut self, args: &Vec<Expr>) -> Result<Expr, String>;
}

impl LogicOps for Env {
    // Returns true if no arguments.
    fn and(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        let mut bool_result = true;

        for expr in args.iter() {
            let tree = self.simplify(&expr);

            let result = tree.and_then(|x| try_get_bool(&x));

            match result {
                Ok(b) => {
                    bool_result = bool_result && b;
                }
                Err(msg) => {
                    return Err(msg);
                }
            }
        }

        Ok(Expr::Atom(Box::new(Atom::Boolean(bool_result))))
    }

    fn or(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        let mut bool_result = false;

        for expr in args.iter() {
            let tree = self.simplify(&expr);

            let result = tree.and_then(|x| try_get_bool(&x));

            match result {
                Ok(b) => {
                    bool_result = bool_result || b;
                }
                Err(msg) => {
                    return Err(msg);
                }
            }
        }

        Ok(Expr::Atom(Box::new(Atom::Boolean(bool_result))))
    }

    fn not(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        if args.len() != 1 {
            Err("Incorrect number of args for 'not' operator.".to_string())
        } else {
            let expr = &args[0];
            let tree = self.simplify(expr);

            let result = tree
                .and_then(|x| try_get_bool(&x))
                .map(|x| Expr::Atom(Box::new(Atom::Boolean(!x))));

            result
        }
    }

    fn atom(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        if args.len() != 1 {
            Err("Incorrect number of args for 'atom' operator.".to_string())
        } else {
            let expr = &args[0];
            let result = self.simplify(expr).map(|x| match x {
                Expr::Atom(_) => Expr::Atom(Box::new(Atom::Boolean(true))),
                Expr::List(_) => Expr::Atom(Box::new(Atom::Boolean(false))),
            });

            result
        }
    }

    fn if_op(&mut self, args: &Vec<Expr>) -> Result<Expr, String> {
        if args.len() == 3 {
            let exp0 = self.simplify(&args[0]);

            let return_exp = exp0.and_then(|x| try_get_bool(&x)).and_then(|x| match x {
                true => self.simplify(&args[1]),
                false => self.simplify(&args[2]),
            });

            return_exp
        } else {
            Err("Incorrect number of arguments for 'if' operator.".to_string())
        }
    }
}

fn try_get_bool(expr: &Expr) -> Result<bool, String> {
    match expr {
        Expr::Atom(atom) => match **atom {
            Atom::Boolean(b) => Ok(b.to_owned()),
            _ => Err("Cannot perform operation on non-boolean value.".to_string()),
        },
        Expr::List(_) => Err("Cannot perform operation on non-boolean value.".to_string()),
    }
}
