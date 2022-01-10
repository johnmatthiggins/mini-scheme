use bigdecimal::BigDecimal;

mod ops {
    // Return first element of list or just empty.
    pub fn car(expr: &Expr) -> Result<Expr, String> {
        match expr {
            Expr::Atom(atom) => Result::Err("'car' can only be applied to lists."),
            Expr::List(list) => values.first() match {
                Some(atom) => Result::Ok(atom),
                None => Result::Err("'car' cannot be applied to empty lists.")
            }
        }
    }

    pub fn cdr(expr: &Expr) -> Expr {
        match expr {
            Expr::Atom(a) => Result::Err("'cdr' can only be applied to lists."),
            Expr::List(list) => Expr::List(list[1..].to_vec())
        }
    }

    pub fn eq(cdr: &[Expr]) -> Result<Expr, String> {
        let first = cdr.first();

        match first {
            Some(s) => {
                let mut is_eq = true;
                
                // Loop through all of them and make sure
                // they are equal to each other.
                for (i, exp) in cdr.iter().enumerate() {
                    is_eq = is_eq && s == exp;
                }
            }
            None => Result::Err("Incorrect argument count for '=' operator."),
        }
    }

    // elementary functions of math.

    fn add(args: &Vec<Expr>) -> Result<Expr, String> {
        let total: BigDecimal = 0;
        let length = list.len();
        
        for (i, expr) in list.enumerate() {
        }
    }

    pub fn sub_list(args: &Vec<Expr>) {
    }
    
    pub fn mul_list(list: &Vec<Expr>) {
    }

    pub fn div_list(list: &Vec<Expr>) {
    }

    pub fn mod_list(list: &Vec<Expr>) {
    }
}
