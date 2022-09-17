use std::vec::Vec;
use std::fs;
use std::fs::File;
use std::io::Read;
use crate::env::*;
use crate::syntax::*;

pub trait EnvSys {
    fn slurp(&mut self, expr: &Vec<Expr>) -> Result<Expr, String>;
    fn write(&mut self, expr: &Vec<Expr>) -> Result<Expr, String>;
    fn print(&mut self, expr: &Vec<Expr>) -> Result<Expr, String>;
    fn println(&mut self, expr: &Vec<Expr>) -> Result<Expr, String>;
}

impl EnvSys for Env {
    fn slurp(&mut self, expr: &Vec<Expr>) -> Result<Expr, String> {
        if expr.len() != 1 {
            Err("Incorrect argument count for 'slurp' function.".to_string())
        }
        else {
            let arg1 = self.simplify(&expr[0]);

            match arg1 {
                Ok(v) =>
                    match v {
                        Expr::Atom(a) =>
                            match a {
                                Atom::StringLiteral(s) => {
                                    let trimmed: &str = &s.as_str()[1..s.len() - 1];
                                    let mut f = File::open(trimmed);
                                    let mut buf = String::new();

                                    let result = f.and_then(|mut x| x.read_to_string(&mut buf));
                                    if let Ok(_) = result {
                                        Ok(Expr::Atom(Atom::StringLiteral(buf)))
                                    }
                                    else {
                                        Err(String::from("'slurp' could not read file..."))
                                    }
                                },
                                _ => Err(
                                    String::from("The argument of 'slurp' must be a string!")),
                            }
                        _ => Err(String::from("Argument of 'slurp' must be file path!")),
                    },
                Err(msg) => Err(msg),
            }
        }
    }

    fn write(&mut self, expr: &Vec<Expr>) -> Result<Expr, String> {
        todo!()
        // if expr.len() != 2 {
        //     Err("Incorrect argument count for 'write' function.".to_string())
        // }
        // else {
        //     let file_name = self.simplify(&expr[0]);
        //     let content = self.simplify(&expr[1]);

        //     match tree {
        //         Ok(v) => Ok(Expr::Atom(Atom::Nil)),
        //         Err(msg) => Err(msg),
        //     }
        // }
    }

    fn print(&mut self, expr: &Vec<Expr>) -> Result<Expr, String> {
        if expr.len() != 1 {
            Err("Incorrect argument count for 'slurp' function.".to_string())
        }
        else {
            let arg1 = self.simplify(&expr[0]);

            match arg1 {
                Ok(v) =>
                    match v {
                        Expr::Atom(a) =>
                            match a {
                                Atom::StringLiteral(s) => {
                                    print!("{}", s);
                                    Ok(Expr::Atom(Atom::Nil))
                                },
                                _ => Err(
                                    String::from("The argument of 'slurp' must be a string!")),
                            }
                        _ => Err(String::from("Argument of 'slurp' must be file path!")),
                    },
                Err(msg) => Err(msg),
            }
        }
    }

    fn println(&mut self, expr: &Vec<Expr>) -> Result<Expr, String> {
        if expr.len() != 1 {
            Err("Incorrect argument count for 'slurp' function.".to_string())
        }
        else {
            let arg1 = self.simplify(&expr[0]);

            match arg1 {
                Ok(v) =>
                    match v {
                        Expr::Atom(a) =>
                            match a {
                                Atom::StringLiteral(s) => {
                                    print!("{}\n", s);
                                    Ok(Expr::Atom(Atom::Nil))
                                },
                                _ => Err(
                                    String::from("The argument of 'slurp' must be a string!")),
                            }
                        _ => Err(String::from("Argument of 'slurp' must be file path!")),
                    },
                Err(msg) => Err(msg),
            }
        }
    }
}
