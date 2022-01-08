use bigdecimal::BigDecimal;

#[derive(Clone, PartialEq, Eq)]
pub enum Atom {
   Boolean(bool),
   StringLiteral(String),
   Number(BigDecimal),
   Symbol(String)
}

#[derive(Clone, PartialEq, Eq)]
pub enum Expr {
    List(Vec<Expr>),
    Atom(Atom)
}
