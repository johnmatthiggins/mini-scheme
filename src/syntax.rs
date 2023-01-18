use bigdecimal::BigDecimal;
use ansi_term::Colour::{Blue, Green, Purple, Red, Yellow};

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
pub const STR_OP: &str = "string";

// logical operations
pub const IF_OP: &str = "if";
pub const OR_OP: &str = "or";
pub const AND_OP: &str = "and";
pub const NOT_OP: &str = "not";

// literals
pub const FALSE_LIT: &str = "#f";
pub const TRUE_LIT: &str = "#t";
pub const NIL_LIT: &str = "nil";

// IO functions
pub const SLURP_FN: &str = "slurp";
pub const WRITE_FILE_FN: &str = "write";
pub const PRINT_FN: &str = "print";
pub const PRINTLN_FN: &str = "println";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LambdaDef {
    pub params: Vec<Expr>,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Atom {
    Boolean(bool),
    StringLiteral(String),
    Number(BigDecimal),
    Symbol(String),
    Lambda(LambdaDef),
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    List(Vec<Expr>),
    Atom(Box<Atom>),
}

pub fn print_tree(expr_tree: &Expr) -> String {
    match expr_tree {
        Expr::List(v) => print_list(v),
        Expr::Atom(atom) => print_atom(&atom),
    }
}

fn print_lambda(lambda: &LambdaDef) -> String {
    // create new string with lambda at start.
    let mut acc = String::new();

    acc.push('(');
    acc.push_str(FUN_OP);
    acc.push(' ');
    acc.push_str(&print_list(&lambda.params));
    acc.push(' ');
    acc.push_str(&print_tree(&*lambda.body));
    acc.push(')');

    acc
}

fn print_list(expr_list: &Vec<Expr>) -> String {
    let mut acc = String::new();

    for (i, exp) in expr_list.iter().enumerate() {
        if i == 0 {
            acc.push('(');
            acc.push_str(&print_tree(exp));
        } else {
            acc.push_str(&print_tree(exp));
        }

        if i == expr_list.len() - 1 {
            acc.push(')');
        } else {
            acc.push(' ');
        }
    }

    acc
}

fn print_atom(expr_atom: &Atom) -> String {
    let result = match expr_atom {
        Atom::Boolean(b) => match b {
            true => Red.paint(TRUE_LIT).to_string(),
            false => Red.paint(FALSE_LIT).to_string(),
        },
        Atom::StringLiteral(s) => Yellow.paint(s.to_string()).to_string(),
        Atom::Number(n) => Red.paint(n.to_string()).to_string(),
        Atom::Symbol(s) => s.to_string(),
        Atom::Nil => Red.paint(NIL_LIT.to_string()).to_string(),
        Atom::Lambda(ld) => print_lambda(ld),
    };

    result
}
