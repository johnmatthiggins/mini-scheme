use std::collections::HashMap;
use crate::syntax::Expr;
use crate::syntax::Atom;

pub type Env = HashMap<String, Expr>;

pub struct SymbolMapping {
    name: String,
    expr: Atom
}

pub trait MapSymbol {
    fn map_symbol(&self, symbol: &String) -> Option<Expr>;
}

pub impl MapSymbol for Env {
    fn map_symbol(&self, symbol: &String) -> Option<&Expr> {
        return self.get(symbol);
    }
}
