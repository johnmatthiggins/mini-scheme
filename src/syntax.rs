use bigdecimal::BigDecimal;

// math operator key words
pub const EQ_OP: &str = "=";
pub const ADD_OP: &str = "+";
pub const SUB_OP: &str = "-";
pub const MUL_OP: &str = "*";
pub const DIV_OP: &str = "/";
pub const MOD_OP: &str = "%";

// built in keywords
pub const CAR_OP: &str = "car";
pub const CDR_OP: &str = "cdr";
pub const QT_OP: &str = "quote";
pub const DEF_OP: &str = "define";
pub const ATM_OP: &str = "atom";
pub const FUN_OP: &str = "lambda";

// logical operations
pub const IF_OP: &str = "if";
pub const OR_OP: &str = "or";
pub const AND_OP: &str = "and";
pub const NOT_OP: &str = "not";

// literals
pub const FALSE_LIT: &str = "#f";
pub const TRUE_LIT: &str = "#t";
pub const NIL_LIT: &str = "()";

#[derive(Clone, PartialEq, Eq)]
pub enum Atom {
    Boolean(bool),
    StringLiteral(String),
    Number(BigDecimal),
    Symbol(String),
    Definition(String),
    Nil
}

#[derive(Clone, PartialEq, Eq)]
pub enum Expr {
    List(Vec<Expr>),
    Atom(Atom)
}

impl ToString for Atom {
    fn to_string(&self) -> String {
        match self {
            Atom::Boolean(b) => b.to_string(),
            Atom::StringLiteral(s) => s,
            Atom::Number(n) => n.to_string(),
            Atom::Symbol(s) => s,
            Atom::Nil => NIL_LIT.to_string()
        }
    }
}
