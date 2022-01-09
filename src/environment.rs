use std::collections::HashMap;
use crate::syntax::Expr;
use crate::syntax::Atom;

pub type Env = HashMap<String, Expr>;

pub struct SymbolMapping {
    name: String,
    expr: Atom
}

pub trait EnvTrait {
    fn map_symbol(&self, symbol: &String) -> Option<&Expr>;
}

impl EnvTrait for Env {
    fn map_symbol(&self, symbol: &String) -> Option<&Expr> {
        return self.get(symbol);
    }

    fn eval(&mut self, input: &String) -> Result<String, String> {
        let tokens = lexer::lexical_analysis(input);

        let result = match tokens {
            Ok(vec) => print_tree(&lexer::parse_tokens(&vec)),
            Err(err) => err
        };

        return result;
    }

    fn simplify(&self, expr: Expr) -> Expr {
        match expr {
            Expr::Atom(atom) => Expr::Atom(&atom),
            Expr::List(list) => // figure out what to do now.
        }
    }
}
