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

pub trait ExprOps {
    pub fn child_count(exp: &Expr) -> u32;
    pub fn is_leaf(&self) -> bool;
    pub fn get_child(root: &Expr, path: &Vec<u32>) -> Option<Expr>;

    // Get node on top of current node path.
    pub fn get_parent(&self, path: &Vec<u32>) -> Option<Expr>;
}

impl ExprOps for Expr {
    fn child_count(&self) -> u32 {
        match self {
            Expr::Atom(_) => 0,
            Expr::List(l) => (u32)l.len()
        }
    }

    // check whether it has children or not.
    fn is_leaf(&self) -> bool {
        match self {
            Expr::Atom(_) => true,
            _ => false
        }
    }

    // Get node on top of current node path.
    fn get_parent(&self, path: &Vec<u32>) -> Option<Expr> {
        let mut parent_path = path.to_owned();
        parent_path.pop();

        let result = self.get_child(parent_path);

        return result;
    }

    fn get_child(&self, path: &Vec<u32>) -> Option<Expr> {
        let mut current = self.to_owned();
        let mut levels_traveled = 0;

        // Get total number of steps required.
        let step_count = path.len();

        loop {
            match current {
                Expr::List(list) => {
                    let next_index = path.get(levels_traveled).unwrap();
                    current = &list.get(next_index).unwrap();
                    
                    // increment levels traveled.
                    levels_traveled += 1;
                }
                _ => { break; }
            }
        }
        
        // If we walked the right number of steps.
        if levels_traveled == step_count {
            return Some(current.to_owned());
        }
        // If the right number of steps through tree were not taken,
        // just return nothing.
        else {
            return None;
        }
    }
}
