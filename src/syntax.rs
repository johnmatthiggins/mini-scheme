use bigdecimal::BigDecimal;

#[derive(Clone, PartialEq, Eq)]
pub struct LambdaDef {
    params: Vec<Expr>,
    body: Box<Expr>
}

#[derive(Clone, PartialEq, Eq)]
pub enum Atom {
   Boolean(bool),
   StringLiteral(String),
   Number(BigDecimal),
   Symbol(String),
   Lambda(LambdaDef),
   Nil
}

#[derive(Clone, PartialEq, Eq)]
pub enum Expr {
    List(Vec<Expr>),
    Atom(Atom)
}
