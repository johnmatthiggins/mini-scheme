use bigdecimal::BigDecimal;

pub enum Atom {
   Boolean(bool),
   StringLiteral(String),
   Number(BigDecimal),
   Symbol(String)
}

pub enum Expr {
    List(Vec<Expr>),
    Atom(Atom)
}
