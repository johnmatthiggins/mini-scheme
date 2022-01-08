use bigdecimal::BigDecimal;

mod ops {
    const ARG_COUNT_ERROR: str = "Incorrect number of arguments for '{}'";

    pub fn car(expr: &Expr) -> Result<Expr, String> {
        match expr {
            Expr::Atom(a) => expr,
            Expr::List(list) => car_is_list(&list),
        }
    }

    fn car_is_list(list: &Vec<Expr>) -> Expr {
        values[0];
    }

    pub fn cdr(expr: &Expr) -> Expr {
        match expr {
            Expr::Atom(a) => expr,
            Expr::List(list) => cdr_is_list(list)
        }
    }

    pub fn cdr_is_list(list: &Vec<Expr>) -> Expr {
        Expr::List(list[1..]);
    }

    pub fn eq(values: &Vec<LValue>) -> Result<LValue, String> {
        if values.len() != 2 {
            Result::Err(format!(ARG_COUNT_ERROR, "="));
        }
        else {
            // If not the same type return false.
            match (values[0], values[1]) {
                (Atom::Number(a), Atom::Number(b)) => Result::Ok(Atom::Bool(a != b)),
                (Atom::Bool(a), Atom::Bool(b)) => Result::Ok(Atom::Bool(a != b)),
                (Atom::String(s1), Atom::String(s2)) => Result::Ok(Atom::Bool(s1.ne(s2)),
                // TODO: Write case for list equality.
                (Atom::List(v1), Atom::List(v2)) => {
                    if v1.len() == v2.len() {
                    }
                    else {
                        let result = Result::Ok(LValue::Bool(false));
                    }

                    return result;
                },
                (Atom::String(s), Atom::Number(n))
                    => Result::Ok(Atom::Bool(n.to_string.ne(s))
                _ => Result::Ok(Atom::Bool(false));
            }
        }
    }

    pub fn eq(expr: Expr) -> Result<Expr, String> {
        match expr {
            Expr::Atom(atom) => Expr::Atom(Atom::Boolean(false)),
            Expr::List(list) => eq_list(&list)
        }
    }

    fn eq_list(list: &Vec<Expr>) -> Result<Expr, String> {
    }

    fn eq_two_args() -> {
    }

    // elementary functions of math.
    
    // Accumulator function to reduce code.
    pub fn acc(values: &Vec<LValue>, op: fn(i64, i64) -> i64) -> Result<LValue, String> {
        if values.len() > 2 {
            let total: i64 = 0;
            let length = values.len();

            for (i, v) in values.enumerate() {
                if let LValue::Number(n) = v {
                    total = op(total, n);

                    if values.len() == i + 1 {
                        let result = Result::Ok(total);
                    }
                }
                else {
                    let result = Result::Err("Cannot perform operation on non numeric type.");
                    break;
                }
            }
        }
        else {
            let result = Result::Err("Cannot perform operation on fewer than two operands.");
        }
    }

    fn add(expr: Expr) -> Result<Expr, String> {
        match expr {
            Expr::Atom(atom) => ,
            Expr::List(list) => add_list(&list)
        }
    }

    fn number_or_error(atom: Atom) -> Result<Expr, String> {
        match atom {
            Atom::Number(n) => Result::Ok(Expr::Atom(Atom::Number(n))),
            _ => Result::Err("Mathematical operators can only be applied to numbers.")
        }
    }

    fn add_list(list: &Vec<Expr>) -> Result<Expr, String> {
        let total: BigDecimal = 0;
        let length = list.len();
        
        for (i, expr) in list.enumerate() {
             
        }
    }

    pub fn sub_list(list: &Vec<Expr>) -> Result<Expr, String> {
    }
    
    pub fn mul_list(list: &Vec<Expr>) -> Result<Expr, String> {
    }

    pub fn div_list(list: &Vec<Expr>) -> Result<Expr, String> {
    }

    pub fn mod_list(list: &Vec<Expr>) -> Result<Expr, String> {
    }
}
