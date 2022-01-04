use bigdecimal::BigDecimal;

#[derive(Clone)]
pub enum Atom {
   Boolean(bool),
   StringLiteral(String),
   Number(BigDecimal),
   Symbol(String)
}

#[derive(Clone)]
pub enum Expr {
    List(Vec<Expr>),
    Atom(Atom)
}
