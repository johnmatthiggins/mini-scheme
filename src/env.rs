use std::collections::HashMap;
use crate::lex;
use crate::built_in::EnvPrimitives;
use crate::boolean::LogicOps;
use crate::syntax::Expr;
use crate::syntax::Atom;
use crate::syntax::LambdaDef;
use crate::syntax;
use crate::math::MathOps;

pub struct Env {
    pub scopes: Vec<HashMap<String, Expr>>,
}

pub trait EnvTrait {
    fn eval(&mut self, input: &String) -> Result<Expr, String>;
    fn eval_list(&mut self, list: &Vec<Expr>) -> Result<Expr, String>;
    fn eval_car_cdr(&mut self, car: Atom, cdr: &Vec<Expr>) -> Result<Expr, String>;
    fn apply(&mut self, func: &String, args: &Vec<Expr>) -> Result<Expr, String>;
    fn simplify(&mut self, expr: &Expr) -> Result<Expr, String>;
    fn get_symbol(&mut self, s: &String) -> Result<Expr, String>;
    fn execute_lambda(&mut self, lambda_def: &LambdaDef, args: &Vec<Expr>) -> Result<Expr, String>;
    fn is_in_scope(&mut self, var_name: &String) -> bool;
    fn begin_scope(&mut self);
    fn end_scope(&mut self);
    fn add_var(&mut self, var_name: String, value: Expr);
}

impl EnvTrait for Env {
    fn eval(&mut self, input: &String) -> Result<Expr, String> {
        let ast = lex::lexical_analysis(input)
            .map(|x| {
                let tree = lex::parse_tokens(&x);
                dbg!(&tree);
                tree
            })
            .and_then(|x| self.simplify(&x));

        ast
    }

    fn eval_list(&mut self, list: &Vec<Expr>) -> Result<Expr, String> {
        let car = list
            .first()
            .map(|x| match x {
                Expr::List(_) => self.simplify(&x),
                Expr::Atom(_) => Ok(x.to_owned())
            })
            .and_then(|x| match x {
                Ok(v) => Some(v),
                Err(_) => None
            });

        match car {
            Some(expr) => match expr {
                Expr::Atom(atom)
                    => self.eval_car_cdr(atom.to_owned(), &list[1..].to_vec()),
                Expr::List(_)
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
        #[allow(non_snake_case)]
        let ERROR_MESSAGE = "Symbol cannot be used as function.";

        if !self.is_in_scope(func) {
           match func.as_str() {
                syntax::EQ_OP => self.eq(args),
                syntax::ADD_OP => self.add(args),
                syntax::SUB_OP => self.sub(args),
                syntax::MUL_OP => self.mul(args),
                syntax::DIV_OP => self.div(args),
                syntax::MOD_OP => self.modulo(args),
                syntax::CAR_OP => self.car(args),
                syntax::CDR_OP => self.cdr(args),
                syntax::QT_OP => self.quote(args),
                syntax::NOT_OP => self.not(args),
                syntax::AND_OP => self.and(args),
                syntax::OR_OP => self.or(args),
                syntax::ATM_OP => self.atom(args),
                syntax::IF_OP => self.if_op(args),
                syntax::DEF_OP => self.define(args),
                syntax::FUN_OP => self.lambda(args),
                _ => Result::Err("Function name not recognized.".to_string()),
            }
        }
        else {
            // Probably will have to remove this once we start implementing the LAMBDA.
            // return Result::Err("Value cannot be used as function.".to_string());
            
            // Grab possibly lambda function from local environment.
            self.begin_scope();
            let maybe_lambda = self
                .get_symbol(func)
                .map(|x| self.simplify(&x))
                .unwrap_or(Err(ERROR_MESSAGE.to_string()))
                .and_then(|x| match x {
                    Expr::Atom(v) => Ok(v),
                    Expr::List(_) => Err(ERROR_MESSAGE.to_string())
                })
                .and_then(|x| match x {
                    Atom::Lambda(def) => Ok(def),
                    _ => Err(ERROR_MESSAGE.to_string())
                });

            let result = match maybe_lambda {
                Ok(def) => {
                    self.scopes.push(HashMap::new());
                    self.execute_lambda(&def, args)
                },
                Err(msg) => Err(msg),
            };
            self.end_scope();

            result
        }
    }

    fn simplify(&mut self, expr: &Expr) -> Result<Expr, String> {
        match expr {
            Expr::List(list) => self.eval_list(list),
            // Don't worry about atoms right now.
            Expr::Atom(atom) => match atom {
                Atom::Symbol(s) => self.get_symbol(&s),
                _ => Ok(Expr::Atom(atom.to_owned()))
            }
        }
    }

    fn get_symbol(&mut self, s: &String) -> Result<Expr, String> {
        let mut i = 0;
        let mut result = Err(format!("Symbol of name '{}' is undefined.", s));
        let mut found = false;

        while i < self.scopes.len() && !found {
            let scope = self.scopes.get_mut(i).unwrap();

            if scope.contains_key(s) {
                result = scope.get(s)
                    .map(|x| self.simplify(&x))
                    .unwrap_or(Err("ERROR!".to_string()));
                found = true;
            }
            i += 1;
        }
        
        result
    }

    fn execute_lambda(&mut self, lambda_def: &LambdaDef, args: &Vec<Expr>)
        -> Result<Expr, String> {
        
        let body = &*lambda_def.body;
        let params = &lambda_def.params;

        if params.len() != args.len() {
            Err("Incorrect argument count for function call.".to_string())
        }
        else {
            // Create another scope for evaluating a function.
            self.begin_scope();

            // Take both vectors of args and params and combine them
            // in an ordered fashion.
            // Then after that insert each one into the local environment.
            for it in args.iter().zip(params.iter()) {
                // Deconstruct into separate parts.
                let (arg, name_expr) = it;

                let name = Result::Ok(name_expr)
                    .and_then(|x| match x {
                        Expr::Atom(v) => Ok(v),
                        Expr::List(_) => Err("List cannot be parameter name.".to_string())
                    })
                    .and_then(|x| match x {
                        Atom::Symbol(s) => Ok(s),
                        _ => Err("Parameter name must be valid symbol.".to_string())
                    });
                
                match name {
                    Ok(v) => {
                        self.scopes
                            .last_mut()
                            .unwrap()
                            .insert(
                                v.to_owned(),
                                arg.to_owned());
                    },
                    Err(msg) => {
                        return Err(msg);
                    }
                }
            }

            let expr = self.simplify(&body);
            // remove the scope we just created.
            self.end_scope();

            expr
        }
    }

    fn is_in_scope(&mut self, var_name: &String) -> bool {
        let symbol = self.get_symbol(var_name);
        matches!(symbol, Ok(_))
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn add_var(&mut self, var_name: String, value: Expr) {
        self.scopes
            .last_mut()
            .unwrap()
            .insert(var_name, value);
    }
}
